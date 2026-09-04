#![forbid(unsafe_code)]

use std::ffi::OsString;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

use apls_compiler::{
    CompileOutcome, CompileSuccess, CompileThrough, Diagnostic, OrderedDiagnostics, compile,
    envelope_bytes,
};
use clap::{ArgAction, Parser, Subcommand, ValueEnum};

pub const VERSION_LINE: &str = "apls 0.1.0 language=0.1 ir=0.1\n";
const ARGUMENT_FAILURE: &str = "APLS-T0001 error: invalid command arguments\n";
const STDOUT_FAILURE: &str = "APLS-T0005 error: stdout artifact transmission failed\n";

#[derive(Debug, Eq, PartialEq)]
pub struct CommandResult {
    pub exit_code: u8,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}

impl CommandResult {
    fn success(stdout: impl Into<Vec<u8>>) -> Self {
        Self {
            exit_code: 0,
            stdout: stdout.into(),
            stderr: Vec::new(),
        }
    }

    fn failure(exit_code: u8, stderr: impl Into<Vec<u8>>) -> Self {
        Self {
            exit_code,
            stdout: Vec::new(),
            stderr: stderr.into(),
        }
    }
}

#[derive(Debug, Parser)]
#[command(name = "apls", disable_version_flag = true)]
struct Cli {
    #[arg(long, action = ArgAction::SetTrue, exclusive = true)]
    version: bool,

    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, Subcommand)]
enum Command {
    Parse {
        entry: PathBuf,
    },
    Check {
        entry: PathBuf,
    },
    EmitIr {
        entry: PathBuf,
        #[arg(long)]
        output: String,
    },
    Diagnose {
        entry: PathBuf,
        #[arg(long, value_enum, default_value_t = Through::Check)]
        through: Through,
        #[arg(long, value_enum, default_value_t = DiagnosticFormat::Json)]
        format: DiagnosticFormat,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
enum Through {
    Parse,
    Check,
    Emit,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
enum DiagnosticFormat {
    Json,
    Human,
}

pub fn run<I, T>(args: I) -> CommandResult
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let cli = match Cli::try_parse_from(args) {
        Ok(cli) => cli,
        Err(_) => return CommandResult::failure(2, ARGUMENT_FAILURE),
    };

    if cli.version {
        return CommandResult::success(VERSION_LINE.as_bytes());
    }

    match cli.command {
        Some(command) => run_command(command),
        None => CommandResult::failure(2, ARGUMENT_FAILURE),
    }
}

pub fn write_command_result<W: Write, E: Write>(
    result: &CommandResult,
    stdout: &mut W,
    stderr: &mut E,
) -> u8 {
    if !result.stdout.is_empty()
        && (stdout.write_all(&result.stdout).is_err() || stdout.flush().is_err())
    {
        let _ = stderr
            .write_all(STDOUT_FAILURE.as_bytes())
            .and_then(|_| stderr.flush());
        return 2;
    }
    if stderr.write_all(&result.stderr).is_err() || stderr.flush().is_err() {
        return 2;
    }
    result.exit_code
}

fn run_command(command: Command) -> CommandResult {
    let (entry, through, diagnose, output) = match command {
        Command::Parse { entry } => (entry, CompileThrough::Parse, None, None),
        Command::Check { entry } => (entry, CompileThrough::Check, None, None),
        Command::EmitIr { entry, output } => (entry, CompileThrough::Emit, None, Some(output)),
        Command::Diagnose {
            entry,
            through,
            format,
        } => {
            let through = match through {
                Through::Parse => CompileThrough::Parse,
                Through::Check => CompileThrough::Check,
                Through::Emit => CompileThrough::Emit,
            };
            (entry, through, Some(format), None)
        }
    };
    let logical_path = match entry.file_name().and_then(|name| name.to_str()) {
        Some(name) if name.ends_with(".apls") => name.to_owned(),
        _ => {
            return finish_failure(
                vec![Diagnostic::tool(
                    "APLS-T0003",
                    "entry must have one valid UTF-8 .apls logical filename",
                )],
                diagnose,
            );
        }
    };
    let original = match read_source(&entry) {
        Ok(bytes) => bytes,
        Err(error) => return finish_failure(vec![error], diagnose),
    };
    let outcome = compile(&original, &logical_path, through);
    if let Some(format) = diagnose {
        let (status, diagnostics, exit_code) = outcome_parts(outcome);
        return diagnose_result(status, diagnostics, exit_code, format);
    }
    match outcome {
        CompileOutcome::Accepted(success) => {
            if through != CompileThrough::Emit {
                return CommandResult::success(Vec::new());
            }
            let CompileSuccess::Verified(artifact) = success else {
                return finish_failure(
                    vec![Diagnostic::tool(
                        "APLS-T0006",
                        "emit pipeline did not produce a verified artifact",
                    )],
                    None,
                );
            };
            let reread = match fs::read(&entry) {
                Ok(bytes) => bytes,
                Err(_) => {
                    return finish_failure(
                        vec![Diagnostic::tool(
                            "APLS-T0004",
                            "source could not be reread before publication",
                        )],
                        None,
                    );
                }
            };
            if reread != original {
                return finish_failure(
                    vec![Diagnostic::tool(
                        "APLS-T0004",
                        "source changed during compilation",
                    )],
                    None,
                );
            }
            publish(
                &entry,
                output.as_deref().expect("emit output"),
                artifact.as_bytes(),
            )
        }
        CompileOutcome::Rejected(d) => CommandResult::failure(1, render_human(&d)),
        CompileOutcome::ToolFailure(d) => CommandResult::failure(2, render_human(&d)),
        CompileOutcome::InternalFailure(d) => CommandResult::failure(3, render_human(&d)),
    }
}

fn read_source(path: &PathBuf) -> Result<Vec<u8>, Diagnostic> {
    reject_symlink_components(path)?;
    let metadata = fs::symlink_metadata(path)
        .map_err(|_| Diagnostic::tool("APLS-T0002", "entry does not exist or is not readable"))?;
    if metadata.file_type().is_symlink() {
        return Err(Diagnostic::tool(
            "APLS-T0003",
            "symbolic-link entry paths are forbidden",
        ));
    }
    if !metadata.is_file() {
        return Err(Diagnostic::tool(
            "APLS-T0002",
            "entry is not a regular file",
        ));
    }
    fs::read(path).map_err(|_| Diagnostic::tool("APLS-T0002", "entry could not be read"))
}

fn reject_symlink_components(path: &std::path::Path) -> Result<(), Diagnostic> {
    let mut prefix = PathBuf::new();
    for component in path.components() {
        prefix.push(component.as_os_str());
        if matches!(component, std::path::Component::ParentDir) {
            return Err(Diagnostic::tool(
                "APLS-T0003",
                "parent-directory traversal is forbidden",
            ));
        }
        if let Ok(metadata) = fs::symlink_metadata(&prefix) {
            if metadata.file_type().is_symlink() {
                return Err(Diagnostic::tool(
                    "APLS-T0003",
                    "symbolic-link path components are forbidden",
                ));
            }
        }
    }
    Ok(())
}

fn publish(entry: &PathBuf, output: &str, bytes: &[u8]) -> CommandResult {
    if output == "-" {
        return CommandResult::success(bytes.to_vec());
    }
    let target = PathBuf::from(output);
    if target == *entry || same_existing_file(entry, &target) {
        return finish_failure(
            vec![Diagnostic::tool(
                "APLS-T0003",
                "output must not be the source path",
            )],
            None,
        );
    }
    if reject_symlink_components(&target).is_err() {
        return finish_failure(
            vec![Diagnostic::tool(
                "APLS-T0003",
                "symbolic-link or traversing output paths are forbidden",
            )],
            None,
        );
    }
    if let Ok(metadata) = fs::symlink_metadata(&target) {
        if metadata.file_type().is_symlink() || !metadata.is_file() {
            return finish_failure(
                vec![Diagnostic::tool(
                    "APLS-T0003",
                    "output must be a regular non-symbolic-link file",
                )],
                None,
            );
        }
    }
    let parent = target
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or_else(|| std::path::Path::new("."));
    let mut temporary = match tempfile::NamedTempFile::new_in(parent) {
        Ok(file) => file,
        Err(_) => {
            return finish_failure(
                vec![Diagnostic::tool(
                    "APLS-T0005",
                    "cannot create an atomic output transaction",
                )],
                None,
            );
        }
    };
    if temporary.write_all(bytes).is_err()
        || temporary.flush().is_err()
        || temporary.as_file().sync_all().is_err()
    {
        return finish_failure(
            vec![Diagnostic::tool(
                "APLS-T0005",
                "cannot write the complete artifact atomically",
            )],
            None,
        );
    }
    if temporary.persist(&target).is_err() {
        return finish_failure(
            vec![Diagnostic::tool(
                "APLS-T0005",
                "cannot atomically replace the output target",
            )],
            None,
        );
    }
    CommandResult::success(Vec::new())
}

fn same_existing_file(left: &std::path::Path, right: &std::path::Path) -> bool {
    match (fs::canonicalize(left), fs::canonicalize(right)) {
        (Ok(left), Ok(right)) => left == right,
        _ => false,
    }
}

fn outcome_parts(outcome: CompileOutcome) -> (&'static str, OrderedDiagnostics, u8) {
    match outcome {
        CompileOutcome::Accepted(_) => ("accepted", Vec::new(), 0),
        CompileOutcome::Rejected(d) => ("rejected", d, 1),
        CompileOutcome::ToolFailure(d) => ("tool_failure", d, 2),
        CompileOutcome::InternalFailure(d) => ("internal_failure", d, 3),
    }
}

fn diagnose_result(
    status: &str,
    diagnostics: OrderedDiagnostics,
    exit_code: u8,
    format: DiagnosticFormat,
) -> CommandResult {
    let stdout = match format {
        DiagnosticFormat::Json => match envelope_bytes(status, &diagnostics) {
            Ok(bytes) => bytes,
            Err(()) => {
                return CommandResult::failure(
                    3,
                    b"APLS-T0006 error: diagnostic envelope failed revalidation\n".to_vec(),
                );
            }
        },
        DiagnosticFormat::Human => render_human(&diagnostics),
    };
    CommandResult {
        exit_code,
        stdout,
        stderr: Vec::new(),
    }
}

fn finish_failure(
    diagnostics: OrderedDiagnostics,
    diagnose: Option<DiagnosticFormat>,
) -> CommandResult {
    if let Some(format) = diagnose {
        diagnose_result("tool_failure", diagnostics, 2, format)
    } else {
        CommandResult::failure(2, render_human(&diagnostics))
    }
}

fn render_human(diagnostics: &[Diagnostic]) -> Vec<u8> {
    let mut text = String::new();
    for diagnostic in diagnostics {
        if let Some(span) = &diagnostic.primary_source_span {
            text.push_str(&format!(
                "{}:{}:{}: ",
                span.logical_path, span.start_line, span.start_column
            ));
        }
        text.push_str(&format!(
            "{} error: {}\n",
            diagnostic.code, diagnostic.message
        ));
    }
    text.into_bytes()
}

#[cfg(test)]
mod tests {
    use super::{CommandResult, VERSION_LINE, run, write_command_result};
    use std::fs;
    use std::io::{self, Write};

    fn owned_ascii_name(mut index: usize) -> String {
        let mut suffix = String::new();
        loop {
            suffix.push((b'a' + (index % 26) as u8) as char);
            index /= 26;
            if index == 0 {
                break;
            }
        }
        format!("e{suffix}")
    }

    fn resource_failure(output: &[u8], resource: &str) {
        let envelope = String::from_utf8(output.to_vec()).unwrap();
        assert!(envelope.contains("\"status\":\"tool_failure\""));
        assert!(envelope.contains(&format!("\"resource\":\"{resource}\"")));
        assert!(
            envelope.contains("\"observed\":1000001") || envelope.contains("\"observed\":4097")
        );
    }

    #[test]
    fn version_matches_the_approved_contract() {
        let result = run(["apls", "--version"]);
        assert_eq!(result.exit_code, 0);
        assert_eq!(result.stdout, VERSION_LINE.as_bytes());
        assert!(result.stderr.is_empty());
    }

    #[test]
    fn missing_entry_is_a_tool_failure() {
        let result = run(["apls", "check", "this-file-does-not-exist.apls"]);
        assert_eq!(result.exit_code, 2);
        assert!(result.stdout.is_empty());
        assert!(
            String::from_utf8(result.stderr)
                .unwrap()
                .starts_with("APLS-T0002")
        );
    }

    #[test]
    fn invalid_arguments_are_tool_failures() {
        let result = run(["apls", "--unknown"]);
        assert_eq!(result.exit_code, 2);
        assert!(result.stdout.is_empty());
        assert!(
            String::from_utf8(result.stderr)
                .unwrap()
                .starts_with("APLS-T0001")
        );
    }

    #[test]
    fn stdout_midstream_failure_returns_t0005_and_exit_two() {
        struct FailAfter {
            bytes: Vec<u8>,
            limit: usize,
        }
        impl Write for FailAfter {
            fn write(&mut self, input: &[u8]) -> io::Result<usize> {
                if self.bytes.len() == self.limit {
                    return Err(io::Error::new(io::ErrorKind::BrokenPipe, "closed pipe"));
                }
                let count = input.len().min(self.limit - self.bytes.len());
                self.bytes.extend_from_slice(&input[..count]);
                Ok(count)
            }

            fn flush(&mut self) -> io::Result<()> {
                Ok(())
            }
        }

        let result = CommandResult::success(b"verified-artifact".to_vec());
        let mut stdout = FailAfter {
            bytes: Vec::new(),
            limit: 8,
        };
        let mut stderr = Vec::new();
        let exit = write_command_result(&result, &mut stdout, &mut stderr);

        assert_eq!(exit, 2);
        assert_eq!(stdout.bytes, b"verified");
        assert_eq!(
            stderr,
            b"APLS-T0005 error: stdout artifact transmission failed\n"
        );
    }

    #[test]
    fn all_public_commands_enforce_state_list_and_literal_spacing_grammar() {
        let directory = tempfile::tempdir_in(std::env::current_dir().unwrap()).unwrap();
        let entry = directory.path().join("main.apls");
        let path = entry.to_str().unwrap();
        let commands = || {
            [
                vec!["apls", "parse", path],
                vec!["apls", "check", path],
                vec!["apls", "emit-ir", path, "--output", "-"],
                vec!["apls", "diagnose", path, "--through", "parse"],
            ]
        };

        let valid = "本规范采用 APLS 简体中文语言版本 0.1。
“水泵”是执行器。
“启动命令”是事件。
“水泵”的状态包括“待机”和“运行”，初始状态是“待机”。
当系统收到启动命令时，水泵从待机状态进入运行状态。";
        fs::write(&entry, valid).unwrap();
        for command in commands() {
            assert_eq!(run(command).exit_code, 0);
        }

        fs::write(&entry, valid.replace("“待机”和“运行”", "“待机”且“运行”")).unwrap();
        for command in commands() {
            assert_eq!(run(command).exit_code, 1);
        }

        let valid_spacing = "本规范采用 APLS 简体中文语言版本 0.1。
“水泵”是执行器。
“液位”是百分比类型的可观测属性。
“启动”是“水泵”支持的动作。
当液位低于20 %时，系统必须启动水泵。";
        fs::write(&entry, valid_spacing).unwrap();
        for command in commands() {
            assert_eq!(run(command).exit_code, 0);
        }

        fs::write(&entry, valid_spacing.replace("20 %", "20  %")).unwrap();
        let parsed = run(["apls", "parse", path]);
        assert_eq!(parsed.exit_code, 0);
        let diagnosed = run(["apls", "diagnose", path, "--through", "parse"]);
        assert_eq!(diagnosed.exit_code, 0);
        assert!(
            String::from_utf8(diagnosed.stdout)
                .unwrap()
                .contains("\"status\":\"accepted\"")
        );

        for command in [
            vec!["apls", "check", path],
            vec!["apls", "emit-ir", path, "--output", "-"],
        ] {
            let result = run(command);
            assert_eq!(result.exit_code, 1);
            let diagnostic = String::from_utf8(result.stderr).unwrap();
            assert!(diagnostic.contains("APLS-E1401"));
            assert!(!diagnostic.contains("APLS-E1101"));
        }
    }

    #[test]
    fn public_cnl_commands_preserve_the_stdout_contract() {
        let directory = tempfile::tempdir_in(std::env::current_dir().unwrap()).unwrap();
        let entry = directory.path().join("main.apls");
        fs::write(
            &entry,
            "本规范采用 APLS 简体中文语言版本 0.1。\n说明：『可编译的自然语言』。",
        )
        .unwrap();
        let path = entry.to_str().unwrap();

        let checked = run(["apls", "check", path]);
        assert_eq!(checked.exit_code, 0);
        assert!(checked.stdout.is_empty());

        let emitted = run(["apls", "emit-ir", path, "--output", "-"]);
        assert_eq!(emitted.exit_code, 0);
        assert!(
            String::from_utf8(emitted.stdout)
                .unwrap()
                .contains("\"status\":\"verified\"")
        );

        let diagnosed = run(["apls", "diagnose", path]);
        assert_eq!(diagnosed.exit_code, 0);
        assert!(
            String::from_utf8(diagnosed.stdout)
                .unwrap()
                .contains("\"status\":\"accepted\"")
        );
    }

    #[test]
    fn failures_do_not_publish_partial_or_legacy_artifacts() {
        let directory = tempfile::tempdir_in(std::env::current_dir().unwrap()).unwrap();
        let entry = directory.path().join("main.apls");
        let vague = "本规范采用 APLS 简体中文语言版本 0.1。\n温度高的时候适当降低一点速度。";
        fs::write(&entry, vague).unwrap();
        let path = entry.to_str().unwrap();
        let diagnosed = run(["apls", "diagnose", path]);
        assert_eq!(diagnosed.exit_code, 1);
        assert!(diagnosed.stderr.is_empty());
        let envelope = String::from_utf8(diagnosed.stdout).unwrap();
        assert!(envelope.contains("\"status\":\"rejected\""));
        assert!(envelope.contains("\"code\":\"APLS-E1301\""));
        assert!(!envelope.contains("\"rules\":"));

        fs::write(&entry, "spec Demo { rule old {} }。").unwrap();
        let legacy = run(["apls", "check", path]);
        assert_eq!(legacy.exit_code, 1);
        assert!(legacy.stdout.is_empty());

        fs::write(
            &entry,
            "本规范采用 APLS 简体中文语言版本 0.1。\n说明：『发布必须原子完成』。",
        )
        .unwrap();
        let same_path = run(["apls", "emit-ir", path, "--output", path]);
        assert_eq!(same_path.exit_code, 2);
        assert_eq!(fs::read_to_string(&entry).unwrap().lines().count(), 2);

        let missing_parent = directory.path().join("missing").join("out.json");
        let publish_failure = run([
            "apls",
            "emit-ir",
            path,
            "--output",
            missing_parent.to_str().unwrap(),
        ]);
        assert_eq!(publish_failure.exit_code, 2);
        assert!(publish_failure.stdout.is_empty());
        assert!(!missing_parent.exists());
        assert!(
            String::from_utf8(publish_failure.stderr)
                .unwrap()
                .contains("APLS-T0005")
        );
    }

    #[test]
    #[ignore = "explicit P0 resource-boundary conformance; intentionally expensive"]
    fn public_candidate_resource_boundaries_use_the_normal_cli_pipeline() {
        let directory = tempfile::tempdir_in(std::env::current_dir().unwrap()).unwrap();
        let entry = directory.path().join("resource.apls");
        let path = entry.to_str().unwrap();
        let declarations = "本规范采用 APLS 简体中文语言版本 0.1。
“水泵”是执行器。
“启动”是“水泵”支持的动作。
“次数”是整数类型的可观测属性。
“有效”是布尔类型的可观测属性。
";
        let numeric_rule = |count: usize, total_atoms: usize| {
            let mut atoms = vec!["次数等于1"; count];
            atoms.extend(vec!["有效等于真"; total_atoms - count]);
            format!("当{}时，系统必须启动水泵。", atoms.join("并且"))
        };

        fs::write(&entry, format!("{declarations}{}", numeric_rule(12, 12))).unwrap();
        let stream_boundary = run(["apls", "parse", path]);
        assert_eq!(stream_boundary.exit_code, 0, "{stream_boundary:?}");
        fs::write(&entry, format!("{declarations}{}", numeric_rule(13, 13))).unwrap();
        let stream_over = run(["apls", "diagnose", path, "--through", "parse"]);
        assert_eq!(stream_over.exit_code, 2, "{stream_over:?}");
        resource_failure(&stream_over.stdout, "complete_token_streams");

        let token_source = |simple_rules: usize, informative: usize| {
            let mut source = format!("{declarations}{}\n", numeric_rule(12, 59));
            source.push_str(&"当有效等于真时，系统必须启动水泵。\n".repeat(simple_rules));
            source.push_str(&"说明：『z』。\n".repeat(informative));
            source
        };
        fs::write(&entry, token_source(417, 7)).unwrap();
        let token_boundary = run(["apls", "parse", path]);
        assert_eq!(token_boundary.exit_code, 0, "{token_boundary:?}");
        fs::write(&entry, token_source(420, 3)).unwrap();
        let token_over = run(["apls", "diagnose", path, "--through", "parse"]);
        assert_eq!(token_over.exit_code, 2, "{token_over:?}");
        resource_failure(&token_over.stdout, "candidate_token_occurrences");

        let lattice_source = |extra_edge: bool| {
            let mut source = String::from("本规范采用 APLS 简体中文语言版本 0.1。\n");
            for index in 0..257 {
                let owner = owned_ascii_name(index);
                source.push_str(&format!("“{owner}”是设备。\n"));
                source.push_str(&format!("“{owner}”的状态包括“x”和“y”，初始状态是“x”。\n"));
            }
            source.push_str("说明：『");
            source.push_str(&"x".repeat(3_875));
            if extra_edge {
                source.push('真');
            }
            source.push_str("』。");
            source
        };
        fs::write(&entry, lattice_source(false)).unwrap();
        let lattice_boundary = run(["apls", "parse", path]);
        assert_eq!(lattice_boundary.exit_code, 0, "{lattice_boundary:?}");
        fs::write(&entry, lattice_source(true)).unwrap();
        let lattice_over = run(["apls", "diagnose", path, "--through", "parse"]);
        assert_eq!(lattice_over.exit_code, 2, "{lattice_over:?}");
        resource_failure(&lattice_over.stdout, "candidate_lattice_edges");
    }
}
