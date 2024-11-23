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
    LBrack,
    RBrack,
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
    "and" => Tag::KWAnd,
    "anyframe" => Tag::KWAnyframe,
    "anytype" => Tag::KWAnytype,
    "asm" => Tag::KWAsm,
    "async" => Tag::KWAsync,
    "await" => Tag::KWAwait,
    "break" => Tag::KWBreak,
    "callconv" => Tag::KWCallconv,
    "catch" => Tag::KWCatch,
    "comptime" => Tag::KWComptime,
    "const" => Tag::KWConst,
    "continue" => Tag::KWContinue,
    "defer" => Tag::KWDefer,
    "else" => Tag::KWElse,
    "enum" => Tag::KWEnum,
    "errdefer" => Tag::KWErrdefer,
    "error" => Tag::KWError,
    "export" => Tag::KWExport,
    "extern" => Tag::KWExtern,
    "fn" => Tag::KWFn,
    "for" => Tag::KWFor,
    "if" => Tag::KWIf,
    "inline" => Tag::KWInline,
    "noalias" => Tag::KWNoalias,
    "noinline" => Tag::KWNoinline,
    "nosuspend" => Tag::KWNosuspend,
    "opaque" => Tag::KWOpaque,
    "or" => Tag::KWOr,
    "orelse" => Tag::KWOrelse,
    "packed" => Tag::KWPacked,
    "pub" => Tag::KWPub,
    "resume" => Tag::KWResume,
    "return" => Tag::KWReturn,
    "linksection" => Tag::KWLinksection,
    "struct" => Tag::KWStruct,
    "suspend" => Tag::KWSuspend,
    "switch" => Tag::KWSwitch,
    "test" => Tag::KWTest,
    "threadlocal" => Tag::KWThreadlocal,
    "try" => Tag::KWTry,
    "union" => Tag::KWUnion,
    "unreachable" => Tag::KWUnreachable,
    "usingnamespace" => Tag::KWUsingnamespace,
    "var" => Tag::KWVar,
    "volatile" => Tag::KWVolatile,
    "while" => Tag::KWWhile,


};

pub fn parse_keyword(tag: &[u8]) -> Option<Tag> {
    if let Ok(as_str) = std::str::from_utf8(tag) {
        TAGS.get(as_str).cloned()
    } else {
        None
    }
}

impl Tag {
    pub fn lexeme(&self) -> Option<&'static str> {
        match self {
            Tag::Invalid
            | Tag::Identifier
            | Tag::StringLiteral
            | Tag::MultilineStringLiteralLine
            | Tag::CharLiteral
            | Tag::Eof
            | Tag::Builtin
            | Tag::NumberLiteral
            | Tag::DocComment
            | Tag::ContainerDocComment => None,

            Tag::InvalidPeriodAsterisks => Some(".**"),
            Tag::Bang => Some("!"),
            Tag::Pipe => Some("|"),
            Tag::PipePipe => Some("||"),
            Tag::PipeEqual => Some("|="),
            Tag::Equal => Some("="),
            Tag::EqualEqual => Some("=="),
            Tag::EqualAngleBrackRight => Some("=>"),
            Tag::BangEqual => Some("!="),
            Tag::LParen => Some("("),
            Tag::RParen => Some(")"),
            Tag::Semicolon => Some(";"),
            Tag::Percent => Some("%"),
            Tag::PercentEqual => Some("%="),
            Tag::LBrace => Some("{"),
            Tag::RBrace => Some("}"),
            Tag::LBrack => Some("["),
            Tag::RBrack => Some("]"),
            Tag::Period => Some("."),
            Tag::PeriodAsterisk => Some(".*"),
            Tag::Ellipsis2 => Some(".."),
            Tag::Ellipsis3 => Some("..."),
            Tag::Caret => Some("^"),
            Tag::CaretEqual => Some("^="),
            Tag::Plus => Some("+"),
            Tag::PlusPlus => Some("++"),
            Tag::PlusEqual => Some("+="),
            Tag::PlusPercent => Some("+%"),
            Tag::PlusPercentEqual => Some("+%="),
            Tag::PlusPipe => Some("+|"),
            Tag::PlusPipeEqual => Some("+|="),
            Tag::Minus => Some("-"),
            Tag::MinusEqual => Some("-="),
            Tag::MinusPercent => Some("-%"),
            Tag::MinusPercentEqual => Some("-%="),
            Tag::MinusPipe => Some("-|"),
            Tag::MinusPipeEqual => Some("-|="),
            Tag::Asterisk => Some("*"),
            Tag::AsteriskEqual => Some("*="),
            Tag::AsteriskAsterisk => Some("**"),
            Tag::AsteriskPercent => Some("*%"),
            Tag::AsteriskPercentEqual => Some("*%="),
            Tag::AsteriskPipe => Some("*|"),
            Tag::AsteriskPipeEqual => Some("*|="),
            Tag::Arrow => Some("->"),
            Tag::Colon => Some(":"),
            Tag::Slash => Some("/"),
            Tag::SlashEqual => Some("/="),
            Tag::Comma => Some(","),
            Tag::Ampersand => Some("&"),
            Tag::AmpersandEqual => Some("&="),
            Tag::QuestionMark => Some("?"),
            Tag::AngleBrackLeft => Some("<"),
            Tag::AngleBrackLeftEqual => Some("<="),
            Tag::AngleBrackAngleBrackLeft => Some("<<"),
            Tag::AngleBrackAngleBrackLeftEqual => Some("<<="),
            Tag::AngleBrackAngleBrackLeftPipe => Some("<<|"),
            Tag::AngleBrackAngleBrackLeftPipeEqual => Some("<<|="),
            Tag::AngleBrackRight => Some(">"),
            Tag::AngleBrackRightEqual => Some(">="),
            Tag::AngleBrackAngleBrackRight => Some(">>"),
            Tag::AngleBrackAngleBrackRightEqual => Some(">>="),
            Tag::Tilde => Some("~"),
            Tag::KWAddrspace => Some("addrspace"),
            Tag::KWAlign => Some("align"),
            Tag::KWAllowzero => Some("allowzero"),
            Tag::KWAnd => Some("and"),
            Tag::KWAnyframe => Some("anyframe"),
            Tag::KWAnytype => Some("anytype"),
            Tag::KWAsm => Some("asm"),
            Tag::KWAsync => Some("async"),
            Tag::KWAwait => Some("await"),
            Tag::KWBreak => Some("break"),
            Tag::KWCallconv => Some("callconv"),
            Tag::KWCatch => Some("catch"),
            Tag::KWComptime => Some("comptime"),
            Tag::KWConst => Some("const"),
            Tag::KWContinue => Some("continue"),
            Tag::KWDefer => Some("defer"),
            Tag::KWElse => Some("else"),
            Tag::KWEnum => Some("enum"),
            Tag::KWErrdefer => Some("errdefer"),
            Tag::KWError => Some("error"),
            Tag::KWExport => Some("export"),
            Tag::KWExtern => Some("extern"),
            Tag::KWFn => Some("fn"),
            Tag::KWFor => Some("for"),
            Tag::KWIf => Some("if"),
            Tag::KWInline => Some("inline"),
            Tag::KWNoalias => Some("noalias"),
            Tag::KWNoinline => Some("noinline"),
            Tag::KWNosuspend => Some("nosuspend"),
            Tag::KWOpaque => Some("opaque"),
            Tag::KWOr => Some("or"),
            Tag::KWOrelse => Some("orelse"),
            Tag::KWPacked => Some("packed"),
            Tag::KWPub => Some("pub"),
            Tag::KWResume => Some("resume"),
            Tag::KWReturn => Some("return"),
            Tag::KWLinksection => Some("linksection"),
            Tag::KWStruct => Some("struct"),
            Tag::KWSuspend => Some("suspend"),
            Tag::KWSwitch => Some("switch"),
            Tag::KWTest => Some("test"),
            Tag::KWThreadlocal => Some("threadlocal"),
            Tag::KWTry => Some("try"),
            Tag::KWUnion => Some("union"),
            Tag::KWUnreachable => Some("unreachable"),
            Tag::KWUsingnamespace => Some("usingnamespace"),
            Tag::KWVar => Some("var"),
            Tag::KWVolatile => Some("volatile"),
            Tag::KWWhile => Some("while"),
        }
    }

    pub fn symbol(&self) -> &'static str {
        self.lexeme().unwrap_or_else(|| match self {
            Tag::Invalid => "invalid token",
            Tag::Identifier => "an identifier",
            Tag::StringLiteral | Tag::MultilineStringLiteralLine => "a string literal",
            Tag::CharLiteral => "a character literal",
            Tag::Eof => "EOF",
            Tag::Builtin => "a builtin function",
            Tag::NumberLiteral => "a number literal",
            Tag::DocComment | Tag::ContainerDocComment => "a document comment",
            _ => unreachable!(),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum State {
    Start,
    ExpectNewline,
    Identifier,
    Builtin,
    StringLiteral,
    StringLiteralBackslash,
    MultilineStringLiteralLine,
    CharLiteral,
    CharLiteralBackslash,
    Backslash,
    Equal,
    Bang,
    Pipe,
    Minus,
    MinusPercent,
    MinusPipe,
    Asterisk,
    AsteriskPercent,
    AsteriskPipe,
    Slash,
    LineCommentStart,
    LineComment,
    DocCommentStart,
    DocComment,
    Int,
    IntExponent,
    IntPeriod,
    Float,
    FloatExponent,
    Ampersand,
    Caret,
    Percent,
    Plus,
    PlusPercent,
    PlusPipe,
    AngleBrackLeft,
    AngleBrackAngleBrackLeft,
    AngleBrackAngleBrackLeftPipe,
    AngleBrackRight,
    AngleBrackAngleBrackRight,
    Period,
    Period2,
    PeriodAsterisk,
    SawAtSign,
    Invalid,
}

pub struct TokenStream<'a> {
    buffer: &'a [u8],
    index: usize,
}

impl<'a> TokenStream<'a> {
    pub fn new(buffer: &'a [u8]) -> Self {
        let index = if buffer.starts_with(&[0xEF, 0xBB, 0xBF]) {
            3
        } else {
            0
        };
        Self { buffer, index }
    }

    #[cfg(debug_assertions)]
    pub fn dump(&self, token: &Token) {
        match std::str::from_utf8(&self.buffer[token.loc.start..token.loc.end]) {
            Ok(text) => println!("{:?} \"{}\"", token.tag, text),
            Err(_) => println!("{:?} <invalid utf8>", token.tag),
        }
    }

    pub fn next_token(&mut self) -> Token {
        let mut token = Token {
            tag: Tag::Invalid,
            loc: Loc {
                start: self.index,
                end: self.index,
            },
        };

        let mut current_state = State::Start;

        loop {
            if self.index >= self.buffer.len() {
                if current_state == State::Start {
                    return Token {
                        tag: Tag::Eof,
                        loc: Loc {
                            start: self.index,
                            end: self.index,
                        },
                    };
                }
                break;
            }

            match current_state {
                State::Start => match self.buffer[self.index] {
                    b' ' | b'\n' | b'\t' | b'\r' => {
                        self.index += 1;
                        token.loc.start = self.index;
                        continue;
                    }
                    b'"' => {
                        token.tag = Tag::StringLiteral;
                        self.index += 1;
                        current_state = State::StringLiteral;
                        continue;
                    }
                    b'\'' => {
                        token.tag = Tag::CharLiteral;
                        self.index += 1;
                        current_state = State::CharLiteral;
                        continue;
                    }
                    b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                        token.tag = Tag::Identifier;
                        self.index += 1;
                        current_state = State::Identifier;
                        continue;
                    }
                    b'@' => {
                        self.index += 1;
                        current_state = State::SawAtSign;
                        continue;
                    }
                    b'=' => {
                        self.index += 1;
                        current_state = State::Equal;
                        continue;
                    }
                    b'!' => {
                        self.index += 1;
                        current_state = State::Bang;
                        continue;
                    }
                    b'|' => {
                        self.index += 1;
                        current_state = State::Pipe;
                        continue;
                    }
                    b'(' => {
                        token.tag = Tag::LParen;
                        self.index += 1;
                        break;
                    }
                    b')' => {
                        token.tag = Tag::RParen;
                        self.index += 1;
                        break;
                    }
                    b'[' => {
                        token.tag = Tag::LBrack;
                        self.index += 1;
                        break;
                    }
                    b']' => {
                        token.tag = Tag::RBrack;
                        self.index += 1;
                        break;
                    }
                    b';' => {
                        token.tag = Tag::Semicolon;
                        self.index += 1;
                        break;
                    }
                    b',' => {
                        token.tag = Tag::Comma;
                        self.index += 1;
                        break;
                    }
                    b'?' => {
                        token.tag = Tag::QuestionMark;
                        self.index += 1;
                        break;
                    }
                    b':' => {
                        token.tag = Tag::Colon;
                        self.index += 1;
                        break;
                    }
                    b'%' => {
                        self.index += 1;
                        current_state = State::Percent;
                        continue;
                    }
                    b'*' => {
                        self.index += 1;
                        current_state = State::Asterisk;
                        continue;
                    }
                    b'+' => {
                        self.index += 1;
                        current_state = State::Plus;
                        continue;
                    }
                    b'<' => {
                        self.index += 1;
                        current_state = State::AngleBrackLeft;
                        continue;
                    }
                    b'>' => {
                        self.index += 1;
                        current_state = State::AngleBrackRight;
                        continue;
                    }
                    b'^' => {
                        self.index += 1;
                        current_state = State::Caret;
                        continue;
                    }
                    b'\\' => {
                        token.tag = Tag::MultilineStringLiteralLine;
                        self.index += 1;
                        current_state = State::Backslash;
                        continue;
                    }
                    b'{' => {
                        token.tag = Tag::LBrace;
                        self.index += 1;
                        break;
                    }
                    b'}' => {
                        token.tag = Tag::RBrace;
                        self.index += 1;
                        break;
                    }
                    b'~' => {
                        token.tag = Tag::Tilde;
                        self.index += 1;
                        break;
                    }
                    b'.' => {
                        self.index += 1;
                        current_state = State::Period;
                        continue;
                    }
                    b'-' => {
                        self.index += 1;
                        current_state = State::Minus;
                        continue;
                    }
                    b'/' => {
                        self.index += 1;
                        current_state = State::Slash;
                        continue;
                    }
                    b'&' => {
                        self.index += 1;
                        current_state = State::Ampersand;
                        continue;
                    }
                    b'0'..=b'9' => {
                        token.tag = Tag::NumberLiteral;
                        self.index += 1;
                        current_state = State::Int;
                        continue;
                    }
                    _ => {
                        current_state = State::Invalid;
                        continue;
                    }
                },
                State::ExpectNewline => {
                    self.index += 1;
                    match self.buffer.get(self.index) {
                        Some(b'\n') => {
                            self.index += 1;
                            token.loc.start = self.index;
                            current_state = State::Start;
                            continue;
                        }
                        Some(_) => {
                            current_state = State::Invalid;
                            continue;
                        }
                        None => {
                            token.tag = Tag::Invalid;
                            break;
                        }
                    }
                }

                State::Invalid => {
                    self.index += 1;
                    if self.index >= self.buffer.len() {
                        token.tag = Tag::Invalid;
                        break;
                    }
                    match self.buffer[self.index] {
                        b'\n' => {
                            token.tag = Tag::Invalid;
                            break;
                        }
                        _ => continue,
                    }
                }

                State::SawAtSign => match self.buffer.get(self.index) {
                    Some(b'\n') | None => {
                        token.tag = Tag::Invalid;
                        break;
                    }
                    Some(b'"') => {
                        token.tag = Tag::Identifier;
                        current_state = State::StringLiteral;
                        continue;
                    }
                    Some(b'a'..=b'z') | Some(b'A'..=b'Z') | Some(b'_') => {
                        token.tag = Tag::Builtin;
                        current_state = State::Builtin;
                        continue;
                    }
                    Some(_) => {
                        current_state = State::Invalid;
                        continue;
                    }
                },

                State::Ampersand => match self.buffer.get(self.index) {
                    Some(b'=') => {
                        token.tag = Tag::AmpersandEqual;
                        self.index += 1;
                        break;
                    }
                    _ => {
                        token.tag = Tag::Ampersand;
                        break;
                    }
                },

                State::Asterisk => match self.buffer.get(self.index) {
                    Some(b'=') => {
                        token.tag = Tag::AsteriskEqual;
                        self.index += 1;
                        break;
                    }
                    Some(b'*') => {
                        token.tag = Tag::AsteriskAsterisk;
                        self.index += 1;
                        break;
                    }
                    Some(b'%') => {
                        self.index += 1;
                        current_state = State::AsteriskPercent;
                        continue;
                    }
                    Some(b'|') => {
                        self.index += 1;
                        current_state = State::AsteriskPipe;
                        continue;
                    }
                    _ => {
                        token.tag = Tag::Asterisk;
                        break;
                    }
                },

                State::AsteriskPercent => match self.buffer.get(self.index) {
                    Some(b'=') => {
                        token.tag = Tag::AsteriskPercentEqual;
                        self.index += 1;
                        break;
                    }
                    _ => {
                        token.tag = Tag::AsteriskPercent;
                        break;
                    }
                },

                State::AsteriskPipe => match self.buffer.get(self.index) {
                    Some(b'=') => {
                        token.tag = Tag::AsteriskPipeEqual;
                        self.index += 1;
                        break;
                    }
                    _ => {
                        token.tag = Tag::AsteriskPipe;
                        break;
                    }
                },

                State::Identifier => match self.buffer.get(self.index) {
                    Some(b'a'..=b'z') | Some(b'A'..=b'Z') | Some(b'_') | Some(b'0'..=b'9') => {
                        self.index += 1;
                        continue;
                    }
                    _ => {
                        if let Some(keyword_tag) =
                            parse_keyword(&self.buffer[token.loc.start..self.index])
                        {
                            token.tag = keyword_tag;
                        }
                        break;
                    }
                },

                State::Builtin => match self.buffer.get(self.index) {
                    Some(b'a'..=b'z') | Some(b'A'..=b'Z') | Some(b'_') | Some(b'0'..=b'9') => {
                        self.index += 1;
                        continue;
                    }
                    _ => break,
                },

                State::Backslash => match self.buffer.get(self.index) {
                    None => {
                        token.tag = Tag::Invalid;
                        break;
                    }
                    Some(b'\\') => {
                        current_state = State::MultilineStringLiteralLine;
                        continue;
                    }
                    Some(b'\n') => {
                        token.tag = Tag::Invalid;
                        break;
                    }
                    Some(_) => {
                        current_state = State::Invalid;
                        continue;
                    }
                },

                State::StringLiteral => {
                    if self.index >= self.buffer.len() {
                        if self.index != self.buffer.len() {
                            current_state = State::Invalid;
                            continue;
                        } else {
                            token.tag = Tag::Invalid;
                            break;
                        }
                    }
                    match self.buffer[self.index] {
                        b'\n' => {
                            token.tag = Tag::Invalid;
                            break;
                        }
                        b'\\' => {
                            self.index += 1;
                            current_state = State::StringLiteralBackslash;
                            continue;
                        }
                        b'"' => {
                            self.index += 1;
                            break;
                        }
                        0x01..=0x09 | 0x0b..=0x1f | 0x7f => {
                            current_state = State::Invalid;
                            continue;
                        }
                        _ => {
                            self.index += 1;
                            continue;
                        }
                    }
                }

                State::StringLiteralBackslash => match self.buffer.get(self.index) {
                    Some(b'\n') | None => {
                        token.tag = Tag::Invalid;
                        break;
                    }
                    Some(_) => {
                        current_state = State::StringLiteral;
                        continue;
                    }
                },

                State::CharLiteral => {
                    if self.index >= self.buffer.len() {
                        if self.index != self.buffer.len() {
                            current_state = State::Invalid;
                            continue;
                        } else {
                            token.tag = Tag::Invalid;
                            break;
                        }
                    }
                    match self.buffer[self.index] {
                        b'\n' => {
                            token.tag = Tag::Invalid;
                            break;
                        }
                        b'\\' => {
                            self.index += 1;
                            current_state = State::CharLiteralBackslash;
                            continue;
                        }
                        b'\'' => {
                            self.index += 1;
                            break;
                        }
                        0x01..=0x09 | 0x0b..=0x1f | 0x7f => {
                            current_state = State::Invalid;
                            continue;
                        }
                        _ => {
                            self.index += 1;
                            continue;
                        }
                    }
                }

                State::CharLiteralBackslash => {
                    if self.index >= self.buffer.len() {
                        if self.index != self.buffer.len() {
                            current_state = State::Invalid;
                            continue;
                        } else {
                            token.tag = Tag::Invalid;
                            break;
                        }
                    }
                    match self.buffer[self.index] {
                        b'\n' => {
                            token.tag = Tag::Invalid;
                            break;
                        }
                        0x01..=0x09 | 0x0b..=0x1f | 0x7f => {
                            current_state = State::Invalid;
                            continue;
                        }
                        _ => {
                            current_state = State::CharLiteral;
                            continue;
                        }
                    }
                }

                State::MultilineStringLiteralLine => {
                    if self.index >= self.buffer.len() {
                        if self.index != self.buffer.len() {
                            current_state = State::Invalid;
                            continue;
                        }
                        break;
                    }
                    match self.buffer[self.index] {
                        b'\n' => break,
                        b'\r' => {
                            if self.buffer.get(self.index + 1) != Some(&b'\n') {
                                current_state = State::Invalid;
                                continue;
                            }
                            break;
                        }
                        0x01..=0x09 | 0x0b..=0x0c | 0x0e..=0x1f | 0x7f => {
                            current_state = State::Invalid;
                            continue;
                        }
                        _ => {
                            self.index += 1;
                            continue;
                        }
                    }
                }
                State::Bang => {
                    match self.buffer.get(self.index) {
                        Some(b'=') => {
                            token.tag = Tag::BangEqual;
                            self.index += 1;
                        }
                        _ => token.tag = Tag::Bang,
                    }
                    break;
                }

                State::Pipe => {
                    match self.buffer.get(self.index) {
                        Some(b'=') => {
                            token.tag = Tag::PipeEqual;
                            self.index += 1;
                        }
                        Some(b'|') => {
                            token.tag = Tag::PipePipe;
                            self.index += 1;
                        }
                        _ => token.tag = Tag::Pipe,
                    }
                    break;
                }

                State::Equal => {
                    match self.buffer.get(self.index) {
                        Some(b'=') => {
                            token.tag = Tag::EqualEqual;
                            self.index += 1;
                        }
                        Some(b'>') => {
                            token.tag = Tag::EqualAngleBrackRight;
                            self.index += 1;
                        }
                        _ => token.tag = Tag::Equal,
                    }
                    break;
                }

                State::Minus => {
                    match self.buffer.get(self.index) {
                        Some(b'>') => {
                            token.tag = Tag::Arrow;
                            self.index += 1;
                        }
                        Some(b'=') => {
                            token.tag = Tag::MinusEqual;
                            self.index += 1;
                        }
                        Some(b'%') => {
                            self.index += 1;
                            current_state = State::MinusPercent;
                            continue;
                        }
                        Some(b'|') => {
                            self.index += 1;
                            current_state = State::MinusPipe;
                            continue;
                        }
                        _ => token.tag = Tag::Minus,
                    }
                    break;
                }

                State::MinusPercent => {
                    match self.buffer.get(self.index) {
                        Some(b'=') => {
                            token.tag = Tag::MinusPercentEqual;
                            self.index += 1;
                        }
                        _ => token.tag = Tag::MinusPercent,
                    }
                    break;
                }

                State::MinusPipe => {
                    match self.buffer.get(self.index) {
                        Some(b'=') => {
                            token.tag = Tag::MinusPipeEqual;
                            self.index += 1;
                        }
                        _ => token.tag = Tag::MinusPipe,
                    }
                    break;
                }

                State::AngleBrackLeft => {
                    match self.buffer.get(self.index) {
                        Some(b'<') => {
                            self.index += 1;
                            current_state = State::AngleBrackAngleBrackLeft;
                            continue;
                        }
                        Some(b'=') => {
                            token.tag = Tag::AngleBrackLeftEqual;
                            self.index += 1;
                        }
                        _ => token.tag = Tag::AngleBrackLeft,
                    }
                    break;
                }

                State::AngleBrackAngleBrackLeft => {
                    match self.buffer.get(self.index) {
                        Some(b'=') => {
                            token.tag = Tag::AngleBrackAngleBrackLeftEqual;
                            self.index += 1;
                        }
                        Some(b'|') => {
                            self.index += 1;
                            current_state = State::AngleBrackAngleBrackLeftPipe;
                            continue;
                        }
                        _ => token.tag = Tag::AngleBrackAngleBrackLeft,
                    }
                    break;
                }

                State::AngleBrackAngleBrackLeftPipe => {
                    match self.buffer.get(self.index) {
                        Some(b'=') => {
                            token.tag = Tag::AngleBrackAngleBrackLeftPipeEqual;
                            self.index += 1;
                        }
                        _ => token.tag = Tag::AngleBrackAngleBrackLeftPipe,
                    }
                    break;
                }

                State::AngleBrackRight => {
                    match self.buffer.get(self.index) {
                        Some(b'>') => {
                            self.index += 1;
                            current_state = State::AngleBrackAngleBrackRight;
                            continue;
                        }
                        Some(b'=') => {
                            token.tag = Tag::AngleBrackRightEqual;
                            self.index += 1;
                        }
                        _ => token.tag = Tag::AngleBrackRight,
                    }
                    break;
                }

                State::AngleBrackAngleBrackRight => {
                    match self.buffer.get(self.index) {
                        Some(b'=') => {
                            token.tag = Tag::AngleBrackAngleBrackRightEqual;
                            self.index += 1;
                        }
                        _ => token.tag = Tag::AngleBrackAngleBrackRight,
                    }
                    break;
                }
                State::Period => {
                    match self.buffer.get(self.index) {
                        Some(b'.') => {
                            self.index += 1;
                            current_state = State::Period2;
                            continue;
                        }
                        Some(b'*') => {
                            self.index += 1;
                            current_state = State::PeriodAsterisk;
                            continue;
                        }
                        _ => token.tag = Tag::Period,
                    }
                    break;
                }

                State::Period2 => {
                    match self.buffer.get(self.index) {
                        Some(b'.') => {
                            token.tag = Tag::Ellipsis3;
                            self.index += 1;
                        }
                        _ => token.tag = Tag::Ellipsis2,
                    }
                    break;
                }

                State::PeriodAsterisk => {
                    match self.buffer.get(self.index) {
                        Some(b'*') => {
                            token.tag = Tag::InvalidPeriodAsterisks;
                        }
                        _ => token.tag = Tag::PeriodAsterisk,
                    }
                    break;
                }

                State::Slash => {
                    match self.buffer.get(self.index) {
                        Some(b'/') => {
                            self.index += 1;
                            current_state = State::LineCommentStart;
                            continue;
                        }
                        Some(b'=') => {
                            token.tag = Tag::SlashEqual;
                            self.index += 1;
                        }
                        _ => token.tag = Tag::Slash,
                    }
                    break;
                }

                State::LineCommentStart => {
                    if self.index >= self.buffer.len() {
                        if self.index != self.buffer.len() {
                            current_state = State::Invalid;
                            continue;
                        } else {
                            return Token {
                                tag: Tag::Eof,
                                loc: Loc {
                                    start: self.index,
                                    end: self.index,
                                },
                            };
                        }
                    }
                    match self.buffer[self.index] {
                        b'!' => {
                            token.tag = Tag::ContainerDocComment;
                            current_state = State::DocComment;
                            continue;
                        }
                        b'\n' => {
                            self.index += 1;
                            token.loc.start = self.index;
                            current_state = State::Start;
                            continue;
                        }
                        b'/' => {
                            self.index += 1;
                            current_state = State::DocCommentStart;
                            continue;
                        }
                        b'\r' => {
                            current_state = State::ExpectNewline;
                            continue;
                        }
                        0x01..=0x09 | 0x0b..=0x0c | 0x0e..=0x1f | 0x7f => {
                            current_state = State::Invalid;
                            continue;
                        }
                        _ => {
                            self.index += 1;
                            current_state = State::LineComment;
                            continue;
                        }
                    }
                }

                State::DocCommentStart => match self.buffer.get(self.index) {
                    None | Some(b'\n') => {
                        token.tag = Tag::DocComment;
                        break;
                    }
                    Some(b'\r') => {
                        if self.buffer.get(self.index + 1) == Some(&b'\n') {
                            token.tag = Tag::DocComment;
                            break;
                        } else {
                            current_state = State::Invalid;
                            continue;
                        }
                    }
                    Some(b'/') => {
                        self.index += 1;
                        current_state = State::LineComment;
                        continue;
                    }
                    Some(0x01..=0x09) | Some(0x0b..=0x0c) | Some(0x0e..=0x1f) | Some(0x7f) => {
                        current_state = State::Invalid;
                        continue;
                    }
                    Some(_) => {
                        token.tag = Tag::DocComment;
                        current_state = State::DocComment;
                        continue;
                    }
                },

                State::LineComment => {
                    if self.index >= self.buffer.len() {
                        if self.index != self.buffer.len() {
                            current_state = State::Invalid;
                            continue;
                        } else {
                            return Token {
                                tag: Tag::Eof,
                                loc: Loc {
                                    start: self.index,
                                    end: self.index,
                                },
                            };
                        }
                    }
                    match self.buffer[self.index] {
                        b'\n' => {
                            self.index += 1;
                            token.loc.start = self.index;
                            current_state = State::Start;
                            continue;
                        }
                        b'\r' => {
                            current_state = State::ExpectNewline;
                            continue;
                        }
                        0x01..=0x09 | 0x0b..=0x0c | 0x0e..=0x1f | 0x7f => {
                            current_state = State::Invalid;
                            continue;
                        }
                        _ => {
                            self.index += 1;
                            continue;
                        }
                    }
                }

                State::DocComment => match self.buffer.get(self.index) {
                    None | Some(b'\n') => break,
                    Some(b'\r') => {
                        if self.buffer.get(self.index + 1) != Some(&b'\n') {
                            current_state = State::Invalid;
                            continue;
                        }
                        break;
                    }
                    Some(0x01..=0x09) | Some(0x0b..=0x0c) | Some(0x0e..=0x1f) | Some(0x7f) => {
                        current_state = State::Invalid;
                        continue;
                    }
                    Some(_) => {
                        self.index += 1;
                        continue;
                    }
                },
                State::Int => match self.buffer.get(self.index) {
                    Some(b'.') => {
                        current_state = State::IntPeriod;
                        continue;
                    }
                    Some(b'_') | Some(b'a'..=b'd') | Some(b'f'..=b'o') | Some(b'q'..=b'z')
                    | Some(b'A'..=b'D') | Some(b'F'..=b'O') | Some(b'Q'..=b'Z')
                    | Some(b'0'..=b'9') => {
                        self.index += 1;
                        continue;
                    }
                    Some(b'e') | Some(b'E') | Some(b'p') | Some(b'P') => {
                        current_state = State::IntExponent;
                        continue;
                    }
                    _ => break,
                },

                State::IntExponent => match self.buffer.get(self.index) {
                    Some(b'-') | Some(b'+') => {
                        self.index += 1;
                        current_state = State::Float;
                        continue;
                    }
                    _ => {
                        current_state = State::Int;
                        continue;
                    }
                },

                State::IntPeriod => {
                    self.index += 1;
                    match self.buffer.get(self.index) {
                        Some(b'_') | Some(b'a'..=b'd') | Some(b'f'..=b'o') | Some(b'q'..=b'z')
                        | Some(b'A'..=b'D') | Some(b'F'..=b'O') | Some(b'Q'..=b'Z')
                        | Some(b'0'..=b'9') => {
                            self.index += 1;
                            current_state = State::Float;
                            continue;
                        }
                        Some(b'e') | Some(b'E') | Some(b'p') | Some(b'P') => {
                            current_state = State::FloatExponent;
                            continue;
                        }
                        _ => {
                            self.index -= 1;
                            break;
                        }
                    }
                }

                State::Float => match self.buffer.get(self.index) {
                    Some(b'_') | Some(b'a'..=b'd') | Some(b'f'..=b'o') | Some(b'q'..=b'z')
                    | Some(b'A'..=b'D') | Some(b'F'..=b'O') | Some(b'Q'..=b'Z')
                    | Some(b'0'..=b'9') => {
                        self.index += 1;
                        continue;
                    }
                    Some(b'e') | Some(b'E') | Some(b'p') | Some(b'P') => {
                        current_state = State::FloatExponent;
                        continue;
                    }
                    _ => break,
                },

                State::FloatExponent => match self.buffer.get(self.index) {
                    Some(b'-') | Some(b'+') => {
                        self.index += 1;
                        current_state = State::Float;
                        continue;
                    }
                    _ => {
                        current_state = State::Float;
                        continue;
                    }
                },
                State::Caret => {
                    match self.buffer.get(self.index) {
                        Some(b'=') => {
                            token.tag = Tag::CaretEqual;
                            self.index += 1;
                        }
                        _ => token.tag = Tag::Caret,
                    }
                    break;
                }

                State::Percent => {
                    match self.buffer.get(self.index) {
                        Some(b'=') => {
                            token.tag = Tag::PercentEqual;
                            self.index += 1;
                        }
                        _ => token.tag = Tag::Percent,
                    }
                    break;
                }

                State::Plus => {
                    match self.buffer.get(self.index) {
                        Some(b'=') => {
                            token.tag = Tag::PlusEqual;
                            self.index += 1;
                        }
                        Some(b'+') => {
                            token.tag = Tag::PlusPlus;
                            self.index += 1;
                        }
                        Some(b'%') => {
                            self.index += 1;
                            current_state = State::PlusPercent;
                            continue;
                        }
                        Some(b'|') => {
                            self.index += 1;
                            current_state = State::PlusPipe;
                            continue;
                        }
                        _ => token.tag = Tag::Plus,
                    }
                    break;
                }

                State::PlusPercent => {
                    match self.buffer.get(self.index) {
                        Some(b'=') => {
                            token.tag = Tag::PlusPercentEqual;
                            self.index += 1;
                        }
                        _ => token.tag = Tag::PlusPercent,
                    }
                    break;
                }

                State::PlusPipe => {
                    match self.buffer.get(self.index) {
                        Some(b'=') => {
                            token.tag = Tag::PlusPipeEqual;
                            self.index += 1;
                        }
                        _ => token.tag = Tag::PlusPipe,
                    }
                    break;
                }
            }
        }

        token.loc.end = self.index;
        token
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tokens() {
        let input = b"fn main() {}";
        let mut tokenizer = TokenStream::new(input);

        let tokens = vec![
            Tag::KWFn,
            Tag::Identifier,
            Tag::LParen,
            Tag::RParen,
            Tag::LBrace,
            Tag::RBrace,
            Tag::Eof,
        ];

        for expected_tag in tokens {
            let token = tokenizer.next_token();
            assert_eq!(token.tag, expected_tag);
        }
    }
}
