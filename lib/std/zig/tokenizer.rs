pub struct Tokenizer;
use phf::phf_map;

#[derive(Debug, Clone)]
pub struct Token {
    pub tag: Tag,
    pub loc: Loc,
}

#[derive(Debug, Clone, Copy)]
pub struct Loc {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Tag {
    Invalid,
    InvalidPeriodAsterisks,
    Identifier,
    StringLiteral,
    MultilineStringLiteralLine,
    CharLiteral,
    Eof,
    Builtin,
    Bang,
    Pipe,
    PipePipe,
    PipeEqual,
    Equal,
    EqualEqual,
    EqualAngleBrackRight,
    BangEqual,
    LParen,
    RParen,
    Semicolon,
    Percent,
    PercentEqual,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Period,
    PeriodAsterisk,
    Ellipsis2,
    Ellipsis3,
    Caret,
    CaretEqual,
    Plus,
    PlusPlus,
    PlusEqual,
    PlusPercent,
    PlusPercentEqual,
    PlusPipe,
    PlusPipeEqual,
    Minus,
    MinusEqual,
    MinusPercent,
    MinusPercentEqual,
    MinusPipe,
    MinusPipeEqual,
    Asterisk,
    AsteriskEqual,
    AsteriskAsterisk,
    AsteriskPercent,
    AsteriskPercentEqual,
    AsteriskPipe,
    AsteriskPipeEqual,
    Arrow,
    Colon,
    Slash,
    SlashEqual,
    Comma,
    Ampersand,
    AmpersandEqual,
    QuestionMark,
    AngleBrackLeft,
    AngleBrackLeftEqual,
    AngleBrackAngleBrackLeft,
    AngleBrackAngleBrackLeftEqual,
    AngleBrackAngleBrackLeftPipe,
    AngleBrackAngleBrackLeftPipeEqual,
    AngleBrackRight,
    AngleBrackRightEqual,
    AngleBrackAngleBrackRight,
    AngleBrackAngleBrackRightEqual,
    Tilde,
    NumberLiteral,
    DocComment,
    ContainerDocComment,
    KWAddrspace,
    KWAlign,
    KWAllowzero,
    KWAnd,
    KWAnyframe,
    KWAnytype,
    KWAsm,
    KWAsync,
    KWAwait,
    KWBreak,
    KWCallconv,
    KWCatch,
    KWComptime,
    KWConst,
    KWContinue,
    KWDefer,
    KWElse,
    KWEnum,
    KWErrdefer,
    KWError,
    KWExport,
    KWExtern,
    KWFn,
    KWFor,
    KWIf,
    KWInline,
    KWNoalias,
    KWNoinline,
    KWNosuspend,
    KWOpaque,
    KWOr,
    KWOrelse,
    KWPacked,
    KWPub,
    KWResume,
    KWReturn,
    KWLinksection,
    KWStruct,
    KWSuspend,
    KWSwitch,
    KWTest,
    KWThreadlocal,
    KWTry,
    KWUnion,
    KWUnreachable,
    KWUsingnamespace,
    KWVar,
    KWVolatile,
    KWWhile,
}

static TAGS: phf::Map<&'static str, Tag> = phf_map! {
    "addrspace" => Tag::KWAddrspace,
    "align" => Tag::KWAlign,
    "allowzero" => Tag::KWAllowzero,


};

pub fn parse_tag(tag: &str) -> Option<Tag> {
    TAGS.get(tag).cloned()
}