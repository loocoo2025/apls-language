fn main() {
    println!("cargo:rerun-if-env-changed=LALRPOP_LANE_TABLE");
    println!("cargo:rerun-if-changed=src/apls_grammar.lalrpop");
    println!("cargo:rerun-if-changed=../../../04_design/ir/apls-cnl-ir-0.1.schema.json");
    println!(
        "cargo:rerun-if-changed=../../../04_design/diagnostics/apls-cnl-diagnostic-0.1.schema.json"
    );

    if std::env::var_os("LALRPOP_LANE_TABLE").is_some() {
        panic!("LALRPOP_LANE_TABLE must be unset; APLS requires the default lane-table LR(1) mode");
    }

    // The only executable grammar is the complete approved APLS 0.1 CNL grammar.
    // Every generation error or conflict is a hard build failure, and generated
    // Rust is written only to Cargo's OUT_DIR.
    lalrpop::process_root().expect("APLS parser generation must succeed with zero conflicts");
}
