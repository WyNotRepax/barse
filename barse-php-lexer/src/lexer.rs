use crate::{Token, TokenName};

pub fn lex(code: impl AsRef<str>) -> Vec<Token> {
    let mut remaining = code.as_ref();
    let mut tokens = Vec::new();
    while !remaining.is_empty() {
        let (token, len) = None
            .or_else(|| Lex::lex(Whitespace::<true>, remaining))
            .or_else(|| Lex::lex(OpenTag, remaining))
            .or_else(|| Lex::lex((Keyword("implements"), TokenName::Implements), remaining))
            .or_else(|| Lex::lex((Keyword("include"), TokenName::Include), remaining))
            .or_else(|| Lex::lex((Keyword("include_once"), TokenName::IncludeOnce), remaining))
            .or_else(|| Lex::lex((Keyword("eval"), TokenName::Eval), remaining))
            .or_else(|| Lex::lex((Keyword("require"), TokenName::Require), remaining))
            .or_else(|| Lex::lex((Keyword("or"), TokenName::LogicalOr), remaining))
            .or_else(|| Lex::lex((Keyword("xor"), TokenName::LogicalXor), remaining))
            .or_else(|| Lex::lex((Keyword("and"), TokenName::LogicalAnd), remaining))
            .or_else(|| Lex::lex((Keyword("print"), TokenName::Print), remaining))
            .or_else(|| Lex::lex(YieldFrom, remaining))
            .or_else(|| Lex::lex((Keyword("yield"), TokenName::Yield), remaining))
            .or_else(|| Lex::lex(("=>", TokenName::DoubleArrow), remaining))
            .or_else(|| Lex::lex(("+=", TokenName::PlusEqual), remaining))
            .or_else(|| Lex::lex(("-=", TokenName::MinusEqual), remaining))
            .or_else(|| Lex::lex(("*=", TokenName::MulEqual), remaining))
            .or_else(|| Lex::lex(("/=", TokenName::DivEqual), remaining))
            .or_else(|| Lex::lex((".=", TokenName::ConcatEqual), remaining))
            .or_else(|| Lex::lex(("%=", TokenName::ModEqual), remaining))
            .or_else(|| Lex::lex(("&=", TokenName::AndEqual), remaining))
            .or_else(|| Lex::lex(("|=", TokenName::OrEqual), remaining))
            .or_else(|| Lex::lex(("^=", TokenName::XorEqual), remaining))
            .or_else(|| Lex::lex(("<<=", TokenName::SlEqual), remaining))
            .or_else(|| Lex::lex((">>=", TokenName::SrEqual), remaining))
            .or_else(|| Lex::lex(("**=", TokenName::PowEqual), remaining))
            .or_else(|| Lex::lex(("**", TokenName::Pow), remaining))
            .or_else(|| Lex::lex(("??", TokenName::Coalesce), remaining))
            .or_else(|| Lex::lex(("||", TokenName::BooleanOr), remaining))
            .or_else(|| Lex::lex(("&&", TokenName::BooleanAnd), remaining))
            .or_else(|| Lex::lex(("===", TokenName::IsIdentical), remaining))
            .or_else(|| Lex::lex(("==", TokenName::IsEqual), remaining))
            .or_else(|| Lex::lex(("!==", TokenName::IsNotIdentical), remaining))
            .or_else(|| Lex::lex(("!=", TokenName::IsNotEqual), remaining))
            .or_else(|| Lex::lex(("<=>", TokenName::Spaceship), remaining))
            .or_else(|| Lex::lex(("<=", TokenName::IsSmallerOrEqual), remaining))
            .or_else(|| Lex::lex((">=", TokenName::IsGreaterOrEqual), remaining))
            .or_else(|| Lex::lex(("<<", TokenName::Sl), remaining))
            .or_else(|| Lex::lex((">>", TokenName::Sr), remaining))
            .or_else(|| Lex::lex(("++", TokenName::Inc), remaining))
            .or_else(|| Lex::lex(("--", TokenName::Dec), remaining))
            .or_else(|| Lex::lex((Keyword("instanceof"), TokenName::Instanceof), remaining))
            .or_else(|| Lex::lex((Cast("int"), TokenName::IntCast), remaining))
            .or_else(|| Lex::lex((Cast("integer"), TokenName::IntCast), remaining))
            .or_else(|| Lex::lex((Cast("float"), TokenName::DoubleCast), remaining))
            .or_else(|| Lex::lex((Cast("double"), TokenName::DoubleCast), remaining))
            .or_else(|| Lex::lex((Cast("real"), TokenName::DoubleCast), remaining))
            .or_else(|| Lex::lex((Cast("string"), TokenName::StringCast), remaining))
            .or_else(|| Lex::lex((Cast("binary"), TokenName::StringCast), remaining))
            .or_else(|| Lex::lex((Cast("array"), TokenName::ArrayCast), remaining))
            .or_else(|| Lex::lex((Cast("object"), TokenName::ObjectCast), remaining))
            .or_else(|| Lex::lex((Cast("bool"), TokenName::BoolCast), remaining))
            .or_else(|| Lex::lex((Cast("boolean"), TokenName::BoolCast), remaining))
            .or_else(|| Lex::lex((Cast("unset"), TokenName::UnsetCast), remaining))
            .or_else(|| Lex::lex((Keyword("new"), TokenName::New), remaining))
            .or_else(|| Lex::lex((Keyword("clone"), TokenName::Clone), remaining))
            .or_else(|| Lex::lex(Simple, remaining))
            .or_else(|| Lex::lex((Keyword("elseif"), TokenName::Elseif), remaining))
            .or_else(|| Lex::lex((Keyword("else"), TokenName::Else), remaining))
            .or_else(|| Lex::lex((Keyword("endif"), TokenName::Endif), remaining))
            .or_else(|| Lex::lex((Keyword("static"), TokenName::Static), remaining))
            .or_else(|| Lex::lex((Keyword("abstract"), TokenName::Abstract), remaining))
            .or_else(|| Lex::lex((Keyword("final"), TokenName::Final), remaining))
            .or_else(|| Lex::lex((Keyword("private"), TokenName::Private), remaining))
            .or_else(|| Lex::lex((Keyword("protected"), TokenName::Protected), remaining))
            .or_else(|| Lex::lex((Keyword("public"), TokenName::Public), remaining))
            .or_else(|| Lex::lex(Lnumber, remaining))
            .or_else(|| Lex::lex(Dnumber, remaining))
            .unwrap();
        tokens.push(token);
        remaining = &remaining[len..];
    }
    tokens
}

trait Lex {
    fn lex(self, code: &str) -> Option<(Token, usize)>;
}

trait PeekLen {
    fn peek_len(self, code: &str) -> Option<usize>;
}

impl<T: PeekLen> Lex for (T, TokenName) {
    fn lex(self, code: &str) -> Option<(Token, usize)> {
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
    fn peek_len(self, code: &str) -> Option<usize> {
        if let Some(len) = CaseInsensitive(self.0).peek_len(code) {
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
    fn lex(self, code: &str) -> Option<(Token, usize)> {
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
        while let Some(b'a'..=b'z' | b'A'..=b'Z' | 0x80..=0xff | b'_') = bytes.next() {
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
    fn lex(self, code: &str) -> Option<(Token, usize)> {
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
sequence! {T1, T2, T3, T4, T5, T6, T8}

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
        struct Octal;
        impl PeekLen for Octal {
            fn peek_len(self, code: &str) -> Option<usize> {
                if !code.starts_with("0") {
                    return None;
                }
                let length = code[1..]
                    .chars()
                    .take_while(|c| c.is_digit(8))
                    .map(char::len_utf8)
                    .sum();
                if length > 0 {
                    Some(length)
                } else {
                    None
                }
            }
        }

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

        struct Hex;
        impl PeekLen for Hex {
            fn peek_len(self, code: &str) -> Option<usize> {
                todo!()
            }
        }
        todo!()
    }
}

impl ConstName for Lnumber {
    const NAME: TokenName = TokenName::Lnumber;
}

// Doubles
struct Dnumber;

impl PeekLen for Dnumber {
    fn peek_len(self, code: &str) -> Option<usize> {
        todo!()
    }
}

impl ConstName for Dnumber {
    const NAME: TokenName = TokenName::Dnumber;
}
