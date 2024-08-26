use itertools::Itertools;

use crate::{Token, TokenName};

struct InlineHtml;

impl PeekLen for InlineHtml {
    fn peek_len(self, code: &str) -> Option<usize> {
        let mut l = 0;
        for c in code.chars() {
            if OpenTag.peek_len(&code[l..]).is_some() {
                break;
            }
            l += c.len_utf8();
        }
        if l > 0 {
            Some(l)
        } else {
            None
        }
    }
}

impl ConstName for InlineHtml {
    const NAME: TokenName = TokenName::InlineHtml;
}

macro_rules! lex_next {
    (from $remaining:ident with $object_operator:ident match $($rules:expr),*) => {
        None
        $(
            .or_else(|| Lex::lex($rules, $remaining, $object_operator))
        )*
        .unwrap()
    };
}

pub fn lex(code: impl AsRef<str>) -> Vec<Token> {
    enum LexerMode {
        InlineHtml,
        Code,
        String,
        ComplexString
    }

    let mut remaining = code.as_ref();
    let mut tokens = Vec::new();
    let mut mode = LexerMode::InlineHtml;
    let mut object_operator = false;
    while !remaining.is_empty() {
        match mode {
            LexerMode::InlineHtml => {
                if let Some((token, len)) = InlineHtml.lex(remaining, false) {
                    tokens.push(token);
                    remaining = &remaining[len..];
                }
                mode = LexerMode::Code;
            }
            LexerMode::Code => {
                let (token, len) = lex_next!(from remaining with object_operator match
                    Whitespace::<true>,
                    OpenTag,
                    DocComment,
                    Comment,
                    ("\\", TokenName::NsSeparator),
                    (Keyword("include"), TokenName::Include),
                    (Keyword("include_once"), TokenName::IncludeOnce),
                    (Keyword("eval"), TokenName::Eval),
                    (Keyword("require"), TokenName::Require),
                    (Keyword("or"), TokenName::LogicalOr),
                    (Keyword("xor"), TokenName::LogicalXor),
                    (Keyword("and"), TokenName::LogicalAnd),
                    (Keyword("print"), TokenName::Print),
                    YieldFrom,
                    (Keyword("yield"), TokenName::Yield),
                    ("=>", TokenName::DoubleArrow),
                    ("+=", TokenName::PlusEqual),
                    ("-=", TokenName::MinusEqual),
                    ("*=", TokenName::MulEqual),
                    ("/=", TokenName::DivEqual),
                    (".=", TokenName::ConcatEqual),
                    ("%=", TokenName::ModEqual),
                    ("&=", TokenName::AndEqual),
                    ("|=", TokenName::OrEqual),
                    ("^=", TokenName::XorEqual),
                    ("<<=", TokenName::SlEqual),
                    (">>=", TokenName::SrEqual),
                    ("**=", TokenName::PowEqual),
                    ("**", TokenName::Pow),
                    ("??", TokenName::Coalesce),
                    ("||", TokenName::BooleanOr),
                    ("&&", TokenName::BooleanAnd),
                    ("===", TokenName::IsIdentical),
                    ("==", TokenName::IsEqual),
                    ("!==", TokenName::IsNotIdentical),
                    ("!=", TokenName::IsNotEqual),
                    ("<=>", TokenName::Spaceship),
                    ("<=", TokenName::IsSmallerOrEqual),
                    (">=", TokenName::IsGreaterOrEqual),
                    ("<<", TokenName::Sl),
                    (">>", TokenName::Sr),
                    ("++", TokenName::Inc),
                    ("--", TokenName::Dec),
                    ("->", TokenName::ObjectOperator),
                    ("::", TokenName::DoubleColon),
                    ("...", TokenName::Ellipsis),
                    (Keyword("instanceof"), TokenName::Instanceof),
                    (Cast("int"), TokenName::IntCast),
                    (Cast("integer"), TokenName::IntCast),
                    (Cast("float"), TokenName::DoubleCast),
                    (Cast("double"), TokenName::DoubleCast),
                    (Cast("real"), TokenName::DoubleCast),
                    (Cast("string"), TokenName::StringCast),
                    (Cast("binary"), TokenName::StringCast),
                    (Cast("array"), TokenName::ArrayCast),
                    (Cast("object"), TokenName::ObjectCast),
                    (Cast("bool"), TokenName::BoolCast),
                    (Cast("boolean"), TokenName::BoolCast),
                    (Cast("unset"), TokenName::UnsetCast),
                    (Keyword("new"), TokenName::New),
                    (Keyword("clone"), TokenName::Clone),
                    (Keyword("elseif"), TokenName::Elseif),
                    (Keyword("else"), TokenName::Else),
                    (Keyword("endif"), TokenName::Endif),
                    (Keyword("static"), TokenName::Static),
                    (Keyword("abstract"), TokenName::Abstract),
                    (Keyword("final"), TokenName::Final),
                    (Keyword("private"), TokenName::Private),
                    (Keyword("protected"), TokenName::Protected),
                    (Keyword("public"), TokenName::Public),
                    (Keyword("function"), TokenName::Function),
                    (Keyword("callable"), TokenName::Callable),
                    (Keyword("class"), TokenName::Class),
                    (Keyword("interface"), TokenName::Interface),
                    (Keyword("trait"), TokenName::Trait),
                    (Keyword("namespace"), TokenName::Namespace),
                    (Keyword("use"), TokenName::Use),
                    (Keyword("implements"), TokenName::Implements),
                    (Keyword("extends"), TokenName::Extends),
                    (Keyword("array"), TokenName::Array),
                    (Keyword("unset"), TokenName::Unset),
                    (Keyword("isset"), TokenName::Isset),
                    (Keyword("empty"), TokenName::Empty),
                    (Keyword("if"), TokenName::If),
                    (Keyword("else"), TokenName::Else),
                    (Keyword("elseif"), TokenName::Elseif),
                    (Keyword("switch"), TokenName::Switch),
                    (Keyword("case"), TokenName::Case),
                    (Keyword("default"), TokenName::Default),
                    (Keyword("foreach"), TokenName::Foreach),
                    (Keyword("while"), TokenName::While),
                    (Keyword("continue"), TokenName::Continue),
                    (Keyword("break"), TokenName::Break),
                    (Keyword("as"), TokenName::As),
                    (Keyword("for"), TokenName::For),
                    (Keyword("return"), TokenName::Return),
                    (Keyword("try"), TokenName::Try),
                    (Keyword("catch"), TokenName::Catch),
                    (Keyword("finally"), TokenName::Finally),
                    (Keyword("throw"), TokenName::Throw),
                    (Keyword("__class__"), TokenName::ClassC),
                    (Keyword("__function__"), TokenName::FuncC),
                    (Keyword("__dir__"), TokenName::Dir),
                    (Keyword("__file__"), TokenName::File),
                    (Keyword("__method__"), TokenName::MethodC),
                    Lnumber,
                    Dnumber,
                    ConstantEncapsedString,
                    Variable,
                    Name,
                    Simple
                );
                if token.is_complex_named(TokenName::CloseTag) {
                    mode = LexerMode::InlineHtml;
                }
                if token.is_simple('\"') {
                    mode = LexerMode::String;
                }
                object_operator = token.is_complex_named(TokenName::ObjectOperator);
                tokens.push(token);
                remaining = &remaining[len..];
            }
            LexerMode::String => {
                let (token, len) = lex_next!(from remaining with object_operator match
                    EncapsedAndWhitespace,
                    CurlyOpen,
                    Variable,
                    Simple
                );
                if token.is_simple('\"') {
                    mode = LexerMode::Code;
                }
                object_operator = token.is_complex_named(TokenName::ObjectOperator);
                tokens.push(token);
                remaining = &remaining[len..];
            }
            LexerMode::ComplexString => {
                let (token, len) = lex_next!(from remaining with object_operator match
                    Variable,
                    Simple
                );
                if token.is_simple('\"') {
                    mode = LexerMode::Code;
                }
                object_operator = token.is_complex_named(TokenName::ObjectOperator);
                tokens.push(token);
                remaining = &remaining[len..];
            }
        }
    }
    tokens
}

trait Lex {
    fn lex(self, code: &str, object_operator: bool) -> Option<(Token, usize)>;
}

trait PeekLen {
    const AFTER_OBJECT_OPERATOR: bool = true;
    fn peek_len(self, code: &str) -> Option<usize>;
}

impl<T: PeekLen> Lex for (T, TokenName) {
    fn lex(self, code: &str, object_operator: bool) -> Option<(Token, usize)> {
        if !T::AFTER_OBJECT_OPERATOR && object_operator {
            return None;
        }
        let (peek_len, name) = self;
        peek_len.peek_len(code).map(|l| {
            (
                Token::Complex {
                    content: code[..l].to_string(),
                    name,
                },
                l,
            )
        })
    }
}

impl PeekLen for &str {
    fn peek_len(self, code: &str) -> Option<usize> {
        if code.starts_with(self) {
            Some(self.len())
        } else {
            None
        }
    }
}

struct CaseInsensitive<'a>(&'a str);
impl PeekLen for CaseInsensitive<'_> {
    fn peek_len(self, code: &str) -> Option<usize> {
        let l = self.0.len();
        if code.len() < l || !code.is_char_boundary(l) {
            return None;
        }
        let code = code[..l].to_lowercase();
        if code.as_str() == self.0 {
            Some(l)
        } else {
            None
        }
    }
}
struct Keyword(&'static str);

impl PeekLen for Keyword {
    const AFTER_OBJECT_OPERATOR: bool = false;
    fn peek_len(self, code: &str) -> Option<usize> {
        let Self(kw) = self;
        if let Some(len) = CaseInsensitive(kw).peek_len(code) {
            if let Some(b'a'..=b'z' | b'A'..=b'Z' | 0x80..=0xff | b'_') = code[len..].bytes().next()
            {
                None
            } else {
                Some(len)
            }
        } else {
            None
        }
    }
}

struct Simple;

impl Lex for Simple {
    fn lex(self, code: &str, _object_operator: bool) -> Option<(Token, usize)> {
        code.chars()
            .next()
            .map(|c| (Token::Simple(c), c.len_utf8()))
    }
}

struct Name;

impl ConstName for Name {
    const NAME: TokenName = TokenName::String;
}

impl PeekLen for Name {
    fn peek_len(self, code: &str) -> Option<usize> {
        let mut bytes = code.bytes();
        let mut bytelen = 0;
        if let Some(b'a'..=b'z' | b'A'..=b'Z' | 0x80..=0xff | b'_') = bytes.next() {
            bytelen += 1;
        } else {
            return None;
        }
        while let Some(b'a'..=b'z' | b'A'..=b'Z' | b'0'..b'9' | 0x80..=0xff | b'_') = bytes.next() {
            bytelen += 1;
        }
        Some(bytelen)
    }
}

struct Whitespace<const NEWLINE: bool>;

impl<const NEWLINE: bool> PeekLen for Whitespace<NEWLINE> {
    fn peek_len(self, code: &str) -> Option<usize> {
        let len = code
            .chars()
            .take_while(|c| c.is_whitespace() && (NEWLINE || *c != '\n' || *c != '\r'))
            .map(char::len_utf8)
            .sum();
        if len > 0 {
            Some(len)
        } else {
            None
        }
    }
}

impl ConstName for Whitespace<true> {
    const NAME: TokenName = TokenName::Whitespace;
}

trait ConstName {
    const NAME: TokenName;
}

impl<T> Lex for T
where
    T: PeekLen + ConstName,
{
    fn lex(self, code: &str, _object_operator: bool) -> Option<(Token, usize)> {
        self.peek_len(code).map(|l| {
            (
                Token::Complex {
                    content: code[..l].to_string(),
                    name: Self::NAME,
                },
                l,
            )
        })
    }
}

struct YieldFrom;

impl PeekLen for YieldFrom {
    fn peek_len(self, code: &str) -> Option<usize> {
        Sequence((
            CaseInsensitive("yield"),
            Whitespace::<true>,
            CaseInsensitive("from"),
        ))
        .peek_len(code)
    }
}

impl ConstName for YieldFrom {
    const NAME: TokenName = TokenName::YieldFrom;
}

struct Sequence<S>(S);
const _: () = {
    macro_rules! sequence {
        ($($types:ident),+) => {
            impl<$($types: PeekLen),*> PeekLen for Sequence<($($types,)*)> {
                #[allow(non_snake_case)]
                fn peek_len(self, code: &str) -> Option<usize> {
                    let ($($types),*) = self.0;
                    Some(0)
                    $(
                        .and_then(|l| $types.peek_len(&code[l..]).map(|lnew| l + lnew))
                    )*
                }
            }
        };
    }

    sequence! {T1, T2}
    sequence! {T1, T2, T3}
    sequence! {T1, T2, T3, T4}
    sequence! {T1, T2, T3, T4, T5}
    sequence! {T1, T2, T3, T4, T5, T6}
    sequence! {T1, T2, T3, T4, T5, T6, T7}
    sequence! {T1, T2, T3, T4, T5, T6, T7, T8}
};
struct Or<O>(O);
const _: () = {
    macro_rules! or {
        ($($types:ident),+) => {
            impl<$($types: PeekLen),*> PeekLen for Or<($($types,)*)> {
                #[allow(non_snake_case)]
                #[inline]
                fn peek_len(self, code: &str) -> Option<usize> {
                    let ($($types),*) = self.0;
                    None
                    $(
                        .or_else(|| $types.peek_len(code))
                    )*
                }
            }
        };
    }

    or! {T1, T2}
    or! {T1, T2, T3}
    or! {T1, T2, T3, T4}
    or! {T1, T2, T3, T4, T5}
    or! {T1, T2, T3, T4, T5, T6}
    or! {T1, T2, T3, T4, T5, T6, T7}
    or! {T1, T2, T3, T4, T5, T6, T7, T8}
};

struct OpenTag;

impl PeekLen for OpenTag {
    fn peek_len(self, code: &str) -> Option<usize> {
        struct SingleWhitespace;
        impl PeekLen for SingleWhitespace {
            fn peek_len(self, code: &str) -> Option<usize> {
                code.chars().next().and_then(|c| {
                    if c.is_whitespace() {
                        Some(c.len_utf8())
                    } else {
                        None
                    }
                })
            }
        }
        Sequence(("<?", CaseInsensitive("php"), SingleWhitespace)).peek_len(code)
    }
}

impl ConstName for OpenTag {
    const NAME: TokenName = TokenName::OpenTag;
}

struct Cast(&'static str);

struct Optional<T>(T);

impl<T> PeekLen for Optional<T>
where
    T: PeekLen,
{
    fn peek_len(self, code: &str) -> Option<usize> {
        self.0.peek_len(code).or(Some(0))
    }
}

impl PeekLen for Cast {
    fn peek_len(self, code: &str) -> Option<usize> {
        Sequence((
            "(",
            Optional(Whitespace::<false>),
            CaseInsensitive(self.0),
            Optional(Whitespace::<false>),
            ")",
        ))
        .peek_len(code)
    }
}

// Integers
struct Lnumber;

impl PeekLen for Lnumber {
    fn peek_len(self, code: &str) -> Option<usize> {
        struct Decimal;
        impl PeekLen for Decimal {
            fn peek_len(self, code: &str) -> Option<usize> {
                let len = code
                    .chars()
                    .take_while(|c| c.is_digit(10))
                    .map(char::len_utf8)
                    .sum();
                if len == 0 {
                    None
                } else if code.starts_with("0") && len > 1 {
                    None
                } else {
                    Some(len)
                }
            }
        }

        struct Octal;
        impl PeekLen for Octal {
            fn peek_len(self, code: &str) -> Option<usize> {
                if !code.starts_with("0") {
                    return None;
                }
                let length: usize = code[1..]
                    .chars()
                    .take_while(|c| c.is_digit(8))
                    .map(char::len_utf8)
                    .sum();
                if length > 0 {
                    Some(length + 1)
                } else {
                    None
                }
            }
        }

        struct Hex;
        impl PeekLen for Hex {
            fn peek_len(self, code: &str) -> Option<usize> {
                if !(code.starts_with("0x") || code.starts_with("0X")) {
                    return None;
                }
                let length: usize = code[2..]
                    .chars()
                    .take_while(char::is_ascii_hexdigit)
                    .map(char::len_utf8)
                    .sum();
                if length > 0 {
                    Some(length + 2)
                } else {
                    None
                }
            }
        }

        struct Bin;
        impl PeekLen for Bin {
            fn peek_len(self, code: &str) -> Option<usize> {
                if !(code.starts_with("0b") || code.starts_with("0B")) {
                    return None;
                }
                let length: usize = code[2..]
                    .chars()
                    .take_while(|c| *c == '0' || *c == '1')
                    .map(char::len_utf8)
                    .sum();
                if length > 0 {
                    Some(length + 2)
                } else {
                    None
                }
            }
        }
        Or((Hex, Octal, Bin, Decimal)).peek_len(code)
    }
}

impl ConstName for Lnumber {
    const NAME: TokenName = TokenName::Lnumber;
}

// Doubles
struct Dnumber;

impl PeekLen for Dnumber {
    fn peek_len(self, code: &str) -> Option<usize> {
        struct Lnum;
        impl PeekLen for Lnum {
            fn peek_len(self, code: &str) -> Option<usize> {
                let len = code
                    .chars()
                    .take_while(|c| c.is_ascii_digit())
                    .map(char::len_utf8)
                    .sum();
                if len > 0 {
                    Some(len)
                } else {
                    None
                }
            }
        }
        struct Dnum;
        impl PeekLen for Dnum {
            fn peek_len(self, code: &str) -> Option<usize> {
                Or((
                    Sequence((Optional(Lnum), ".", Lnum)),
                    Sequence((Lnum, ".", Optional(Lnum))),
                ))
                .peek_len(code)
            }
        }
        struct ExponentDnum;
        impl PeekLen for ExponentDnum {
            fn peek_len(self, code: &str) -> Option<usize> {
                Sequence((
                    Or((Dnum, Lnum)),
                    CaseInsensitive("e"),
                    Optional(Or(("+", "-"))),
                    Lnum,
                ))
                .peek_len(code)
            }
        }
        Or((ExponentDnum, Dnum, Lnum)).peek_len(code)
    }
}

impl ConstName for Dnumber {
    const NAME: TokenName = TokenName::Dnumber;
}

struct Variable;

impl PeekLen for Variable {
    fn peek_len(self, code: &str) -> Option<usize> {
        Sequence(("$", Name)).peek_len(code)
    }
}

impl ConstName for Variable {
    const NAME: TokenName = TokenName::Variable;
}

struct Comment;

impl ConstName for Comment {
    const NAME: TokenName = TokenName::Comment;
}

impl PeekLen for Comment {
    fn peek_len(self, code: &str) -> Option<usize> {
        struct SingleLineComment;
        impl PeekLen for SingleLineComment {
            fn peek_len(self, code: &str) -> Option<usize> {
                PeekLen::peek_len("//", code)
                    .or_else(|| PeekLen::peek_len("#", code))
                    .map(|l| {
                        code[l..]
                            .find("\n")
                            .map(|n| l + n + 1)
                            .unwrap_or(code.len())
                    })
            }
        }
        struct MultiLineComment;
        impl PeekLen for MultiLineComment {
            fn peek_len(self, code: &str) -> Option<usize> {
                PeekLen::peek_len("/*", code).map(|l| {
                    code[l..]
                        .find("*/")
                        .map(|n| l + n + 2)
                        .unwrap_or(code.len())
                })
            }
        }
        None.or_else(|| SingleLineComment.peek_len(code))
            .or_else(|| MultiLineComment.peek_len(code))
    }
}
struct DocComment;

impl ConstName for DocComment {
    const NAME: TokenName = TokenName::DocComment;
}

impl PeekLen for DocComment {
    fn peek_len(self, code: &str) -> Option<usize> {
        PeekLen::peek_len("/**", code).map(|l| {
            code[l..]
                .find("*/")
                .map(|n| l + n + 2)
                .unwrap_or(code.len())
        })
    }
}

struct ConstantEncapsedString;

impl ConstName for ConstantEncapsedString {
    const NAME: TokenName = TokenName::ConstantEncapsedString;
}

impl PeekLen for ConstantEncapsedString {
    fn peek_len(self, code: &str) -> Option<usize> {
        if !code.starts_with('\'') {
            return None;
        }
        let mut escape = false;
        let mut l = '\''.len_utf8();
        for c in code['\''.len_utf8()..].chars() {
            l += c.len_utf8();
            if !escape && c == '\'' {
                return Some(l);
            }
            escape = !escape && c == '\\';
        }
        None
    }
}

struct EncapsedAndWhitespace;

impl ConstName for EncapsedAndWhitespace {
    const NAME: TokenName = TokenName::EncapsedAndWhitespace;
}

impl PeekLen for EncapsedAndWhitespace {
    fn peek_len(self, code: &str) -> Option<usize> {
        let mut l = 0;
        let mut escape = false;
        for c in code.chars() {
            if !escape {
                match c {
                    '\\' => escape = true,
                    '$' | '"' => break,
                    '{' if &code[l..][1..=1] == "$" => break,
                    _ => (),
                }
            }
            l += c.len_utf8()
        }
        if l > 0 {
            Some(l)
        } else {
            None
        }
    }
}

struct CurlyOpen;

impl ConstName for CurlyOpen {
    const NAME: TokenName = TokenName::CurlyOpen;
}

impl PeekLen for CurlyOpen {
    fn peek_len(self, code: &str) -> Option<usize> {
        if code.starts_with("{") {
            Some("{".len())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    macro_rules! assert_peeks_completely {
        ($t:expr, $what:literal) => {{
            assert!(
                $t.peek_len($what) == Some($what.len()),
                "{} == {:?} != {:?}",
                stringify!($t.peek_len($what)),
                $t.peek_len($what),
                Some($what.len())
            );
        }};
    }

    use crate::lexer::Dnumber;

    use super::{Lnumber, PeekLen};

    #[test]
    fn dnumber() {
        assert_peeks_completely!(Dnumber, "1.234");
        assert_peeks_completely!(Dnumber, "1.2e3");
        assert_peeks_completely!(Dnumber, "7E-10");
    }

    #[test]
    fn lnumber() {
        assert_peeks_completely!(Lnumber, "1234"); // decimal number
        assert_peeks_completely!(Lnumber, "0123"); // octal number (equivalent to 83 decimal)
                                                   //  0o123; // octal number (as of PHP 8.1.0)
        assert_peeks_completely!(Lnumber, "0x1A"); // hexadecimal number (equivalent to 26 decimal)
        assert_peeks_completely!(Lnumber, "0b11111111"); // binary number (equivalent to 255 decimal)
    }
}
