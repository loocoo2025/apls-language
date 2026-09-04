//! Crate-private typed Surface AST for the complete APLS 0.1 grammar.

#![allow(dead_code)]

use crate::lexer::ByteSpan;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Spanned<T> {
    pub span: ByteSpan,
    pub value: T,
}

pub(crate) fn spanned<T>(start: usize, value: T, end: usize) -> Spanned<T> {
    Spanned {
        span: ByteSpan { start, end },
        value,
    }
}

pub(crate) type Identifier = Spanned<String>;
pub(crate) type StringLiteral = Spanned<String>;
pub(crate) type QualifiedName = Spanned<Vec<String>>;
pub(crate) type TypeRef = Spanned<TypeRefKind>;
pub(crate) type Expression = Spanned<ExpressionKind>;
pub(crate) type Literal = Spanned<LiteralKind>;
pub(crate) type ConstExpr = Spanned<ConstExprKind>;
pub(crate) type Action = Spanned<ActionKind>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ParsedProgram {
    pub unit: CompilationUnit,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct CompilationUnit {
    pub language_version: StringLiteral,
    pub imports: Vec<ImportDecl>,
    pub spec: SpecDecl,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ImportDecl {
    pub path: StringLiteral,
    pub alias: Identifier,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct SpecDecl {
    pub name: Identifier,
    pub version: StringLiteral,
    pub members: Vec<SpecMember>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum SpecMember {
    Intent(StringLiteral),
    Dimension(DimensionDecl),
    Unit(UnitDecl),
    Type(TypeDecl),
    Enum(EnumDecl),
    Record(RecordDecl),
    Domain(DomainDecl),
    Component(ComponentDecl),
    Operation(OperationDecl),
    Transport(TransportDecl),
    Event(EventDecl),
    Execution(ExecutionDecl),
    Channel(ChannelDecl),
    StateMachine(StateMachineDecl),
    Rule(RuleDecl),
    Constraint(ConstraintDecl),
    Safety(SafetyDecl),
    Acceptance(AcceptanceDecl),
    Decision(DecisionDecl),
    Open(OpenDecl),
    Unknown(UnknownDecl),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct DimensionDecl {
    pub name: Identifier,
    pub base_unit: Identifier,
    pub rationale: Option<StringLiteral>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct UnitDecl {
    pub name: Identifier,
    pub dimension: QualifiedName,
    pub symbol: StringLiteral,
    pub scale: Spanned<String>,
    pub offset: Option<Spanned<SignedNumber>>,
    pub rationale: Option<StringLiteral>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct SignedNumber {
    pub negative: bool,
    pub magnitude: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct TypeDecl {
    pub name: Identifier,
    pub target: TypeRef,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct EnumDecl {
    pub name: Identifier,
    pub variants: Vec<Identifier>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct RecordDecl {
    pub name: Identifier,
    pub fields: Vec<FieldDecl>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct FieldDecl {
    pub name: Identifier,
    pub type_ref: TypeRef,
    pub default: Option<ConstExpr>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum TypeRefKind {
    Primitive(PrimitiveType),
    Named(QualifiedName),
    List(Box<TypeRef>),
    Optional(Box<TypeRef>),
    Quantity {
        dimension: QualifiedName,
        unit: QualifiedName,
    },
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PrimitiveType {
    Bool,
    Int,
    Decimal,
    String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct DomainDecl {
    pub name: Identifier,
    pub model: DomainModel,
    pub supports: Vec<ExecutionKind>,
    pub rationale: Option<StringLiteral>,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum DomainModel {
    Os,
    Rtos,
    BareMetal,
    ManagedRuntime,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ComponentDecl {
    pub name: Identifier,
    pub responsibilities: Vec<StringLiteral>,
    pub rationale: Option<StringLiteral>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct OperationDecl {
    pub name: Identifier,
    pub parameters: Vec<ParameterDecl>,
    pub returns: Option<TypeRef>,
    pub kind: OperationKind,
    pub rationale: Option<StringLiteral>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ParameterDecl {
    pub name: Identifier,
    pub type_ref: TypeRef,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum OperationKind {
    Pure,
    Action,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct TransportDecl {
    pub name: Identifier,
    pub kind: TransportKind,
    pub rationale: Option<StringLiteral>,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum TransportKind {
    InProcess,
    Ipc,
    Network,
    Bus,
    Custom,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct EventDecl {
    pub name: Identifier,
    pub payload: Option<TypeRef>,
    pub source: Option<QualifiedName>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ExecutionDecl {
    pub name: Identifier,
    pub kind: ExecutionKind,
    pub domain: QualifiedName,
    pub owner: Option<QualifiedName>,
    pub trigger: TriggerExpr,
    pub schedule: ScheduleExpr,
    pub responsibilities: Vec<StringLiteral>,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ExecutionKind {
    Process,
    Thread,
    Task,
    MainLoop,
    Interrupt,
    EventLoop,
    Coroutine,
    Actor,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum TriggerExpr {
    Call(QualifiedName),
    Event(QualifiedName),
    Message(QualifiedName),
    Timer(Expression),
    Interrupt(QualifiedName),
    StateChange(QualifiedName),
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum ScheduleExpr {
    Periodic(Vec<NamedArg>),
    EventDriven,
    Preemptive,
    Cooperative,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ChannelDecl {
    pub name: Identifier,
    pub from: QualifiedName,
    pub to: QualifiedName,
    pub payload: TypeRef,
    pub mode: ChannelMode,
    pub transport: QualifiedName,
    pub ordering: Ordering,
    pub delivery: Delivery,
    pub capacity: Option<Spanned<String>>,
    pub timeout: Option<Expression>,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ChannelMode {
    Message,
    Event,
    Queue,
    Rpc,
    Stream,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Ordering {
    Fifo,
    Priority,
    Unordered,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Delivery {
    AtMostOnce,
    AtLeastOnce,
    ExactlyOnce,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct StateMachineDecl {
    pub name: Identifier,
    pub initial: Identifier,
    pub states: Vec<StateDecl>,
    pub transitions: Vec<TransitionDecl>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct StateDecl {
    pub name: Identifier,
    pub terminal: bool,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct TransitionDecl {
    pub name: Identifier,
    pub from: Identifier,
    pub on: QualifiedName,
    pub guard: Option<Expression>,
    pub to: Identifier,
    pub priority: Option<Spanned<String>>,
    pub then: Option<Vec<Action>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct RuleDecl {
    pub name: Identifier,
    pub when: Expression,
    pub hold_for: Option<Expression>,
    pub then: Vec<Action>,
    pub rationale: Option<StringLiteral>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ConstraintDecl {
    pub name: Identifier,
    pub class: ConstraintClass,
    pub assertion: Expression,
    pub rationale: Option<StringLiteral>,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ConstraintClass {
    Architecture,
    Performance,
    Resource,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct SafetyDecl {
    pub name: Identifier,
    pub when: Expression,
    pub prohibited: Vec<QualifiedName>,
    pub then: Vec<Action>,
    pub rationale: Option<StringLiteral>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct AcceptanceDecl {
    pub name: Identifier,
    pub given: Vec<GivenStmt>,
    pub when: Vec<Action>,
    pub expect: Expression,
    pub within: Option<Expression>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct GivenStmt {
    pub name: Identifier,
    pub type_ref: TypeRef,
    pub value: ConstExpr,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct DecisionDecl {
    pub name: Identifier,
    pub status: DecisionStatus,
    pub statement: StringLiteral,
    pub rationale: StringLiteral,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum DecisionStatus {
    Frozen,
    Approved,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct OpenDecl {
    pub name: Identifier,
    pub question: StringLiteral,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct UnknownDecl {
    pub name: Identifier,
    pub statement: StringLiteral,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct NamedArg {
    pub name: Identifier,
    pub value: Expression,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum ActionKind {
    Emit {
        target: QualifiedName,
        arguments: Vec<NamedArg>,
    },
    Invoke {
        target: QualifiedName,
        arguments: Vec<NamedArg>,
    },
    Transition {
        machine: QualifiedName,
        state: QualifiedName,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum ExpressionKind {
    Literal(Literal),
    Name(QualifiedName),
    Call {
        target: QualifiedName,
        arguments: Vec<NamedArg>,
    },
    Unary {
        operator: UnaryOperator,
        operand: Box<Expression>,
    },
    Binary {
        operator: BinaryOperator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UnaryOperator {
    Not,
    Negate,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum BinaryOperator {
    Or,
    And,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum LiteralKind {
    Bool(bool),
    String(String),
    Numeric(NumericLiteral),
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct NumericLiteral {
    pub magnitude: String,
    pub unit: Option<QualifiedName>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum ConstExprKind {
    Literal(Literal),
    List(Vec<Literal>),
}
