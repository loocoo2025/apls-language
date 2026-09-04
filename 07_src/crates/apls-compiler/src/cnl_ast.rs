use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct ByteSpan {
    pub start_byte: usize,
    pub end_byte: usize,
}

impl ByteSpan {
    pub const fn new(start_byte: usize, end_byte: usize) -> Self {
        Self {
            start_byte,
            end_byte,
        }
    }

    pub fn cover(self, other: Self) -> Self {
        Self::new(
            self.start_byte.min(other.start_byte),
            self.end_byte.max(other.end_byte),
        )
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Spanned<T> {
    pub value: T,
    pub span: ByteSpan,
}

pub fn spanned<T>(left: usize, value: T, right: usize) -> Spanned<T> {
    Spanned {
        value,
        span: ByteSpan::new(left, right),
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum SymbolKind {
    Entity,
    Property,
    Action,
    Event,
    State,
    Unit,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct SymbolRef {
    pub id: String,
    pub display_name: String,
    pub kind: SymbolKind,
    pub owner_ref: Option<String>,
    pub target_ref: Option<String>,
    pub span: ByteSpan,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum EntityKind {
    Device,
    Component,
    Sensor,
    Actuator,
}

impl EntityKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Device => "device",
            Self::Component => "component",
            Self::Sensor => "sensor",
            Self::Actuator => "actuator",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ValueType {
    Boolean,
    Integer,
    Decimal,
    Percentage,
    Text,
    Duration,
}

impl ValueType {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Boolean => "boolean",
            Self::Integer => "integer",
            Self::Decimal => "decimal",
            Self::Percentage => "percentage",
            Self::Text => "text",
            Self::Duration => "duration",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum AliasKind {
    Entity,
    Property,
    Action,
    Event,
    Unit,
}

impl AliasKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Entity => "entity",
            Self::Property => "property",
            Self::Action => "action",
            Self::Event => "event",
            Self::Unit => "unit",
        }
    }

    pub const fn symbol_kind(self) -> SymbolKind {
        match self {
            Self::Entity => SymbolKind::Entity,
            Self::Property => SymbolKind::Property,
            Self::Action => SymbolKind::Action,
            Self::Event => SymbolKind::Event,
            Self::Unit => SymbolKind::Unit,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum PropertyAccess {
    Observable,
    Writable,
    Both,
}

impl PropertyAccess {
    pub const fn flags(self) -> (bool, bool) {
        match self {
            Self::Observable => (true, false),
            Self::Writable => (false, true),
            Self::Both => (true, true),
        }
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Declaration {
    Entity {
        name: Spanned<String>,
        kind: Spanned<EntityKind>,
    },
    Unit {
        name: Spanned<String>,
    },
    Property {
        name: Spanned<String>,
        value_type: Spanned<ValueType>,
        access: Spanned<PropertyAccess>,
        unit: Option<Spanned<String>>,
    },
    Action {
        name: Spanned<String>,
        target: Spanned<String>,
    },
    Event {
        name: Spanned<String>,
    },
    Alias {
        alias: Spanned<String>,
        kind: Spanned<AliasKind>,
        target: Spanned<String>,
    },
    State {
        owner: Spanned<String>,
        states: Vec<Spanned<String>>,
        initial: Spanned<String>,
    },
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ComparisonOp {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

impl ComparisonOp {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Eq => "eq",
            Self::Ne => "ne",
            Self::Lt => "lt",
            Self::Le => "le",
            Self::Gt => "gt",
            Self::Ge => "ge",
        }
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum RawLiteralKind {
    Boolean(bool),
    Integer(String),
    Decimal(String),
    Percentage(String),
    Temperature(String),
    Duration { value: String, unit: String },
    Quantity { value: String, unit: SymbolRef },
    Text(String),
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct RawLiteral {
    pub kind: RawLiteralKind,
    pub span: ByteSpan,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum AtomicCondition {
    Comparison {
        property: SymbolRef,
        operator: Spanned<ComparisonOp>,
        right: RawLiteral,
        span: ByteSpan,
    },
    State {
        entity: SymbolRef,
        state: SymbolRef,
        span: ByteSpan,
    },
    Event {
        receiver: ActorRef,
        event: SymbolRef,
        span: ByteSpan,
    },
}

impl AtomicCondition {
    pub const fn span(&self) -> ByteSpan {
        match self {
            Self::Comparison { span, .. } | Self::State { span, .. } | Self::Event { span, .. } => {
                *span
            }
        }
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Condition {
    pub atoms: Vec<AtomicCondition>,
    pub span: ByteSpan,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ActorRef {
    System(ByteSpan),
    Entity(SymbolRef),
}

impl ActorRef {
    pub fn id(&self) -> &str {
        match self {
            Self::System(_) => "builtin:system",
            Self::Entity(value) => &value.id,
        }
    }
    pub const fn span(&self) -> ByteSpan {
        match self {
            Self::System(span) => *span,
            Self::Entity(value) => value.span,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Modality {
    Require,
    Prohibit,
}

impl Modality {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Require => "require",
            Self::Prohibit => "prohibit",
        }
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ActionInvocation {
    pub actor: ActorRef,
    pub action: SymbolRef,
    pub target: SymbolRef,
    pub span: ByteSpan,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum WhenContinuation {
    Rule {
        actor: ActorRef,
        modality: Spanned<Modality>,
        action: SymbolRef,
        target: SymbolRef,
    },
    Transition {
        entity: SymbolRef,
        source_state: SymbolRef,
        target_state: SymbolRef,
    },
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum NormativeSentence {
    Rule {
        condition: Condition,
        modality: Spanned<Modality>,
        behavior: ActionInvocation,
    },
    Transition {
        trigger: Condition,
        entity: SymbolRef,
        source_state: SymbolRef,
        target_state: SymbolRef,
    },
    Invariant {
        condition: Condition,
        required_entity: SymbolRef,
        required_state: SymbolRef,
    },
    Acceptance {
        trigger: Condition,
        expected: Condition,
        deadline: RawLiteral,
    },
    Informative {
        text: Spanned<String>,
    },
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Token {
    Fixed(FixedToken),
    DeclaredTerm(String),
    EntityRef(SymbolRef),
    PropertyRef(SymbolRef),
    ActionRef(SymbolRef),
    EventRef(SymbolRef),
    StateRef(SymbolRef),
    UnitRef(SymbolRef),
    SignedInteger(String),
    SignedDecimal(String),
    UnsignedInteger(String),
    UnsignedDecimal(String),
    Text(String),
    InformativeText(String),
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum FixedToken {
    ProfilePrefix,
    Apls,
    LanguageVersion,
    Version01,
    FullStop,
    Comma,
    Colon,
    EnumerationComma,
    Is,
    Unit,
    EntityDevice,
    EntityComponent,
    EntitySensor,
    EntityActuator,
    TypeSuffix,
    Observable,
    Writable,
    Both,
    Property,
    UnitIs,
    Supported,
    Action,
    Event,
    Alias,
    Entity,
    StateIncludes,
    InitialStateIs,
    AndList,
    When,
    If,
    Then,
    System,
    Must,
    MustNot,
    Prohibit,
    From,
    StateEnter,
    StateWord,
    SafetyRequirement,
    AcceptanceRequirement,
    MustWithin,
    HoldsWithin,
    Explanation,
    Conjunction,
    Equals,
    NotEquals,
    Below,
    NotAbove,
    Above,
    NotBelow,
    InState,
    Received,
    True,
    False,
    Percent,
    Celsius,
    Millisecond,
    Second,
    Minute,
    BooleanType,
    IntegerType,
    DecimalType,
    PercentageType,
    TextType,
    DurationType,
}
