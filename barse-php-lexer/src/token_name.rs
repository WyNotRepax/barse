#[cfg_attr(feature = "native", derive(serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenName {

    /**
     * Syntax: `abstract`
     * Reference: [`Class Abstraction`](https://www.php.net/manual/en/language.oop5.abstract.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_ABSTRACT"))]
    Abstract,

    /**
     * Syntax: `&`
     * Reference: [`Type declarations`](https://www.php.net/manual/en/language.types.declarations.php) (available as of PHP 8.1.0) 
     */
    #[cfg(feature = "php_8_1")]
    #[cfg_attr(feature = "native", serde(rename = "T_AMPERSAND_FOLLOWED_BY_VAR_OR_VARARG"))]
    AmpersandFollowedByVarOrVararg,

    /**
     * Syntax: `&`
     * Reference: [`Type declarations`](https://www.php.net/manual/en/language.types.declarations.php) (available as of PHP 8.1.0) 
     */
    #[cfg(feature = "php_8_1")]
    #[cfg_attr(feature = "native", serde(rename = "T_AMPERSAND_NOT_FOLLOWED_BY_VAR_OR_VARARG"))]
    AmpersandNotFollowedByVarOrVararg,

    /**
     * Syntax: `&=`
     * Reference: [`assignment operators`](https://www.php.net/manual/en/language.operators.assignment.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_AND_EQUAL"))]
    AndEqual,

    /**
     * Syntax: `array()`
     * Reference: [`array()`](https://www.php.net/manual/en/function.array.php), [`array syntax`](https://www.php.net/manual/en/language.types.array.php#language.types.array.syntax) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_ARRAY"))]
    Array,

    /**
     * Syntax: `(array)`
     * Reference: [`type-casting`](https://www.php.net/manual/en/language.types.type-juggling.php#language.types.typecasting) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_ARRAY_CAST"))]
    ArrayCast,

    /**
     * Syntax: `as`
     * Reference: [`foreach`](https://www.php.net/manual/en/control-structures.foreach.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_AS"))]
    As,

    /**
     * Syntax: `#[`
     * Reference: [`attributes`](https://www.php.net/manual/en/language.attributes.php) (available as of PHP 8.0.0) 
     */
    #[cfg(feature = "php_8_0")]
    #[cfg_attr(feature = "native", serde(rename = "T_ATTRIBUTE"))]
    Attribute,

    /**
     * Syntax: ` `
     * Reference: anything below ASCII 32 except \t (0x09), \n (0x0a) and \r (0x0d)
      (available as of PHP 7.4.0) 
      */
    #[cfg(feature = "php_7_4")]
    #[cfg_attr(feature = "native", serde(rename = "T_BAD_CHARACTER"))]
    BadCharacter,

    /**
     * Syntax: `&&`
     * Reference: [`logical operators`](https://www.php.net/manual/en/language.operators.logical.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_BOOLEAN_AND"))]
    BooleanAnd,

    /**
     * Syntax: `||`
     * Reference: [`logical operators`](https://www.php.net/manual/en/language.operators.logical.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_BOOLEAN_OR"))]
    BooleanOr,

    /**
     * Syntax: `(bool) or (boolean)`
     * Reference: [`type-casting`](https://www.php.net/manual/en/language.types.type-juggling.php#language.types.typecasting) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_BOOL_CAST"))]
    BoolCast,

    /**
     * Syntax: `break`
     * Reference: [`break`](https://www.php.net/manual/en/control-structures.break.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_BREAK"))]
    Break,

    /**
     * Syntax: `callable`
     * Reference: [`callable`](https://www.php.net/manual/en/language.types.callable.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_CALLABLE"))]
    Callable,

    /**
     * Syntax: `case`
     * Reference: [`switch`](https://www.php.net/manual/en/control-structures.switch.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_CASE"))]
    Case,

    /**
     * Syntax: `catch`
     * Reference: [`Exceptions`](https://www.php.net/manual/en/language.exceptions.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_CATCH"))]
    Catch,

    /**
     * Syntax: `class`
     * Reference: [`classes and objects`](https://www.php.net/manual/en/language.oop5.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_CLASS"))]
    Class,

    /**
     * Syntax: `__CLASS__`
     * Reference: [`magic constants`](https://www.php.net/manual/en/language.constants.predefined.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_CLASS_C"))]
    ClassC,

    /**
     * Syntax: `clone`
     * Reference: [`classes and objects`](https://www.php.net/manual/en/language.oop5.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_CLONE"))]
    Clone,

    /**
     * Syntax: `?> or %>`
     * Reference: [`escaping from HTML`](https://www.php.net/manual/en/language.basic-syntax.phpmode.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_CLOSE_TAG"))]
    CloseTag,

    /**
     * Syntax: `??`
     * Reference: [`comparison operators`](https://www.php.net/manual/en/language.operators.comparison.php#language.operators.comparison.coalesce) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_COALESCE"))]
    Coalesce,

    /**
     * Syntax: `??=`
     * Reference: [`assignment operators`](https://www.php.net/manual/en/language.operators.assignment.php)
      (available as of PHP 7.4.0) 
     */
    #[cfg(feature = "php_7_4")]
    #[cfg_attr(feature = "native", serde(rename = "T_COALESCE_EQUAL"))]
    CoalesceEqual,

    /**
     * Syntax: `// or #, and /* */`
     * Reference: [`comments`](https://www.php.net/manual/en/language.basic-syntax.comments.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_COMMENT"))]
    Comment,

    /**
     * Syntax: `.=`
     * Reference: [`assignment operators`](https://www.php.net/manual/en/language.operators.assignment.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_CONCAT_EQUAL"))]
    ConcatEqual,

    /**
     * Syntax: `const`
     * Reference: [`class constants`](https://www.php.net/manual/en/language.constants.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_CONST"))]
    Const,

    /**
     * Syntax: `"foo" or 'bar'`
     * Reference: [`string syntax`](https://www.php.net/manual/en/language.types.string.php#language.types.string.syntax) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_CONSTANT_ENCAPSED_STRING"))]
    ConstantEncapsedString,

    /**
     * Syntax: `continue`
     * Reference: [`continue`](https://www.php.net/manual/en/control-structures.continue.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_CONTINUE"))]
    Continue,

    /**
     * Syntax: `{$`
     * Reference: [`complex variable parsed syntax`](https://www.php.net/manual/en/language.types.string.php#language.types.string.parsing.complex) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_CURLY_OPEN"))]
    CurlyOpen,

    /**
     * Syntax: `--`
     * Reference: [`incrementing/decrementing operators`](https://www.php.net/manual/en/language.operators.increment.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_DEC"))]
    Dec,

    /**
     * Syntax: `declare`
     * Reference: [`declare`](https://www.php.net/manual/en/control-structures.declare.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_DECLARE"))]
    Declare,

    /**
     * Syntax: `default`
     * Reference: [`switch`](https://www.php.net/manual/en/control-structures.switch.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_DEFAULT"))]
    Default,

    /**
     * Syntax: `__DIR__`
     * Reference: [`magic constants`](https://www.php.net/manual/en/language.constants.predefined.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_DIR"))]
    Dir,

    /**
     * Syntax: `/=`
     * Reference: [`assignment operators`](https://www.php.net/manual/en/language.operators.assignment.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_DIV_EQUAL"))]
    DivEqual,

    /**
     * Syntax: `0.12, etc.`
     * Reference: [`floating point numbers`](https://www.php.net/manual/en/language.types.float.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_DNUMBER"))]
    Dnumber,

    /**
     * Syntax: `do`
     * Reference: [`do..while`](https://www.php.net/manual/en/control-structures.do.while.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_DO"))]
    Do,

    /**
     * Syntax: `/** */`
     * Reference: [`PHPDoc style comments`](https://www.php.net/manual/en/language.basic-syntax.comments.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_DOC_COMMENT"))]
    DocComment,

    /**
     * Syntax: `${`
     * Reference: [`complex variable parsed syntax`](https://www.php.net/manual/en/language.types.string.php#language.types.string.parsing.complex) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_DOLLAR_OPEN_CURLY_BRACES"))]
    DollarOpenCurlyBraces,

    /**
     * Syntax: `=>`
     * Reference: [`array syntax`](https://www.php.net/manual/en/language.types.array.php#language.types.array.syntax) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_DOUBLE_ARROW"))]
    DoubleArrow,

    /**
     * Syntax: `(real), (double) or (float)`
     * Reference: [`type-casting`](https://www.php.net/manual/en/language.types.type-juggling.php#language.types.typecasting) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_DOUBLE_CAST"))]
    DoubleCast,

    /**
     * Syntax: `::`
     * Reference: see [`T_PAAMAYIM_NEKUDOTAYIM`](https://www.php.net/manual/en/tokens.php#constant.t-paamayim-nekudotayim) below 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_DOUBLE_COLON"))]
    DoubleColon,

    /**
     * Syntax: `echo`
     * Reference: [`echo`](https://www.php.net/manual/en/function.echo.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_ECHO"))]
    Echo,

    /**
     * Syntax: `...`
     * Reference: [`function arguments`](https://www.php.net/manual/en/functions.arguments.php#functions.variable-arg-list) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_ELLIPSIS"))]
    Ellipsis,

    /**
     * Syntax: `else`
     * Reference: [`else`](https://www.php.net/manual/en/control-structures.else.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_ELSE"))]
    Else,

    /**
     * Syntax: `elseif`
     * Reference: [`elseif`](https://www.php.net/manual/en/control-structures.elseif.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_ELSEIF"))]
    Elseif,

    /**
     * Syntax: `empty`
     * Reference: [`empty()`](https://www.php.net/manual/en/function.empty.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_EMPTY"))]
    Empty,

    /**
     * Syntax: `" $a"`
     * Reference: [`constant part of string with variables`](https://www.php.net/manual/en/language.types.string.php#language.types.string.parsing) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_ENCAPSED_AND_WHITESPACE"))]
    EncapsedAndWhitespace,

    /**
     * Syntax: `enddeclare`
     * Reference: [`declare`](https://www.php.net/manual/en/control-structures.declare.php), [`alternative syntax`](https://www.php.net/manual/en/control-structures.alternative-syntax.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_ENDDECLARE"))]
    Enddeclare,

    /**
     * Syntax: `endfor`
     * Reference: [`for`](https://www.php.net/manual/en/control-structures.for.php), [`alternative syntax`](https://www.php.net/manual/en/control-structures.alternative-syntax.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_ENDFOR"))]
    Endfor,

    /**
     * Syntax: `endforeach`
     * Reference: [`foreach`](https://www.php.net/manual/en/control-structures.foreach.php), [`alternative syntax`](https://www.php.net/manual/en/control-structures.alternative-syntax.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_ENDFOREACH"))]
    Endforeach,

    /**
     * Syntax: `endif`
     * Reference: [`if`](https://www.php.net/manual/en/control-structures.if.php), [`alternative syntax`](https://www.php.net/manual/en/control-structures.alternative-syntax.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_ENDIF"))]
    Endif,

    /**
     * Syntax: `endswitch`
     * Reference: [`switch`](https://www.php.net/manual/en/control-structures.switch.php), [`alternative syntax`](https://www.php.net/manual/en/control-structures.alternative-syntax.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_ENDSWITCH"))]
    Endswitch,

    /**
     * Syntax: `endwhile`
     * Reference: [`while`](https://www.php.net/manual/en/control-structures.while.php), [`alternative syntax`](https://www.php.net/manual/en/control-structures.alternative-syntax.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_ENDWHILE"))]
    Endwhile,

    /**
     * Syntax: `enum`
     * Reference: [`Enumerations`](https://www.php.net/manual/en/language.types.enumerations.php) (available as of PHP 8.1.0) 
     */
    #[cfg(feature = "php_8_1")]
    #[cfg_attr(feature = "native", serde(rename = "T_ENUM"))]
    Enum,

    /**
     * Syntax: ` `
     * Reference: [`heredoc syntax`](https://www.php.net/manual/en/language.types.string.php#language.types.string.syntax.heredoc) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_END_HEREDOC"))]
    EndHeredoc,

    /**
     * Syntax: `eval()`
     * Reference: [`eval()`](https://www.php.net/manual/en/function.eval.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_EVAL"))]
    Eval,

    /**
     * Syntax: `exit or die`
     * Reference: [`exit()`](https://www.php.net/manual/en/function.exit.php), [`die()`](https://www.php.net/manual/en/function.die.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_EXIT"))]
    Exit,

    /**
     * Syntax: `extends`
     * Reference: [`extends`](https://www.php.net/manual/en/language.oop5.basic.php#language.oop5.basic.extends), [`classes and objects`](https://www.php.net/manual/en/language.oop5.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_EXTENDS"))]
    Extends,

    /**
     * Syntax: `__FILE__`
     * Reference: [`magic constants`](https://www.php.net/manual/en/language.constants.predefined.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_FILE"))]
    File,

    /**
     * Syntax: `final`
     * Reference: [`Final Keyword`](https://www.php.net/manual/en/language.oop5.final.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_FINAL"))]
    Final,

    /**
     * Syntax: `finally`
     * Reference: [`Exceptions`](https://www.php.net/manual/en/language.exceptions.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_FINALLY"))]
    Finally,

    /**
     * Syntax: `fn`
     * Reference: [`arrow functions`](https://www.php.net/manual/en/functions.arrow.php)
      (available as of PHP 7.4.0) 
     */
    #[cfg(feature = "php_7_4")]
    #[cfg_attr(feature = "native", serde(rename = "T_FN"))]
    Fn,

    /**
     * Syntax: `for`
     * Reference: [`for`](https://www.php.net/manual/en/control-structures.for.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_FOR"))]
    For,

    /**
     * Syntax: `foreach`
     * Reference: [`foreach`](https://www.php.net/manual/en/control-structures.foreach.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_FOREACH"))]
    Foreach,

    /**
     * Syntax: `function`
     * Reference: [`functions`](https://www.php.net/manual/en/language.functions.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_FUNCTION"))]
    Function,

    /**
     * Syntax: `__FUNCTION__`
     * Reference: [`magic constants`](https://www.php.net/manual/en/language.constants.predefined.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_FUNC_C"))]
    FuncC,

    /**
     * Syntax: `global`
     * Reference: [`variable scope`](https://www.php.net/manual/en/language.variables.scope.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_GLOBAL"))]
    Global,

    /**
     * Syntax: `goto`
     * Reference: [`goto`](https://www.php.net/manual/en/control-structures.goto.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_GOTO"))]
    Goto,

    /**
     * Syntax: `__halt_compiler()`
     * Reference: [`__halt_compiler`](https://www.php.net/manual/en/function.halt-compiler.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_HALT_COMPILER"))]
    HaltCompiler,

    /**
     * Syntax: `if`
     * Reference: [`if`](https://www.php.net/manual/en/control-structures.if.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_IF"))]
    If,

    /**
     * Syntax: `implements`
     * Reference: [`Object Interfaces`](https://www.php.net/manual/en/language.oop5.interfaces.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_IMPLEMENTS"))]
    Implements,

    /**
     * Syntax: `++`
     * Reference: [`incrementing/decrementing operators`](https://www.php.net/manual/en/language.operators.increment.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_INC"))]
    Inc,

    /**
     * Syntax: `include`
     * Reference: [`include`](https://www.php.net/manual/en/function.include.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_INCLUDE"))]
    Include,

    /**
     * Syntax: `include_once`
     * Reference: [`include_once`](https://www.php.net/manual/en/function.include-once.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_INCLUDE_ONCE"))]
    IncludeOnce,

    /**
     * Syntax: ` `
     * Reference: [`text outside PHP`](https://www.php.net/manual/en/language.basic-syntax.phpmode.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_INLINE_HTML"))]
    InlineHtml,

    /**
     * Syntax: `instanceof`
     * Reference: [`type operators`](https://www.php.net/manual/en/language.operators.type.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_INSTANCEOF"))]
    Instanceof,

    /**
     * Syntax: `insteadof`
     * Reference: [`Traits`](https://www.php.net/manual/en/language.oop5.traits.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_INSTEADOF"))]
    Insteadof,

    /**
     * Syntax: `interface`
     * Reference: [`Object Interfaces`](https://www.php.net/manual/en/language.oop5.interfaces.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_INTERFACE"))]
    Interface,

    /**
     * Syntax: `(int) or (integer)`
     * Reference: [`type-casting`](https://www.php.net/manual/en/language.types.type-juggling.php#language.types.typecasting) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_INT_CAST"))]
    IntCast,

    /**
     * Syntax: `isset()`
     * Reference: [`isset()`](https://www.php.net/manual/en/function.isset.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_ISSET"))]
    Isset,

    /**
     * Syntax: `==`
     * Reference: [`comparison operators`](https://www.php.net/manual/en/language.operators.comparison.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_IS_EQUAL"))]
    IsEqual,

    /**
     * Syntax: `>=`
     * Reference: [`comparison operators`](https://www.php.net/manual/en/language.operators.comparison.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_IS_GREATER_OR_EQUAL"))]
    IsGreaterOrEqual,

    /**
     * Syntax: `===`
     * Reference: [`comparison operators`](https://www.php.net/manual/en/language.operators.comparison.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_IS_IDENTICAL"))]
    IsIdentical,

    /**
     * Syntax: `!= or <>`
     * Reference: [`comparison operators`](https://www.php.net/manual/en/language.operators.comparison.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_IS_NOT_EQUAL"))]
    IsNotEqual,

    /**
     * Syntax: `!==`
     * Reference: [`comparison operators`](https://www.php.net/manual/en/language.operators.comparison.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_IS_NOT_IDENTICAL"))]
    IsNotIdentical,

    /**
     * Syntax: `<=`
     * Reference: [`comparison operators`](https://www.php.net/manual/en/language.operators.comparison.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_IS_SMALLER_OR_EQUAL"))]
    IsSmallerOrEqual,

    /**
     * Syntax: `__LINE__`
     * Reference: [`magic constants`](https://www.php.net/manual/en/language.constants.predefined.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_LINE"))]
    Line,

    /**
     * Syntax: `list()`
     * Reference: [`list()`](https://www.php.net/manual/en/function.list.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_LIST"))]
    List,

    /**
     * Syntax: `123, 012, 0x1ac, etc.`
     * Reference: [`integers`](https://www.php.net/manual/en/language.types.integer.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_LNUMBER"))]
    Lnumber,

    /**
     * Syntax: `and`
     * Reference: [`logical operators`](https://www.php.net/manual/en/language.operators.logical.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_LOGICAL_AND"))]
    LogicalAnd,

    /**
     * Syntax: `or`
     * Reference: [`logical operators`](https://www.php.net/manual/en/language.operators.logical.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_LOGICAL_OR"))]
    LogicalOr,

    /**
     * Syntax: `xor`
     * Reference: [`logical operators`](https://www.php.net/manual/en/language.operators.logical.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_LOGICAL_XOR"))]
    LogicalXor,

    /**
     * Syntax: `match`
     * Reference: [`match`](https://www.php.net/manual/en/control-structures.match.php) (available as of PHP 8.0.0) 
     */
    #[cfg(feature = "php_8_0")]
    #[cfg_attr(feature = "native", serde(rename = "T_MATCH"))]
    Match,

    /**
     * Syntax: `__METHOD__`
     * Reference: [`magic constants`](https://www.php.net/manual/en/language.constants.predefined.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_METHOD_C"))]
    MethodC,

    /**
     * Syntax: `-=`
     * Reference: [`assignment operators`](https://www.php.net/manual/en/language.operators.assignment.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_MINUS_EQUAL"))]
    MinusEqual,

    /**
     * Syntax: `%=`
     * Reference: [`assignment operators`](https://www.php.net/manual/en/language.operators.assignment.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_MOD_EQUAL"))]
    ModEqual,

    /**
     * Syntax: `*=`
     * Reference: [`assignment operators`](https://www.php.net/manual/en/language.operators.assignment.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_MUL_EQUAL"))]
    MulEqual,

    /**
     * Syntax: `namespace`
     * Reference: [`namespaces`](https://www.php.net/manual/en/language.namespaces.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_NAMESPACE"))]
    Namespace,

    /**
     * Syntax: `\App\Namespace`
     * Reference: [`namespaces`](https://www.php.net/manual/en/language.namespaces.php) (available as of PHP 8.0.0) 
     */
    #[cfg(feature = "php_8_0")]
    #[cfg_attr(feature = "native", serde(rename = "T_NAME_FULLY_QUALIFIED"))]
    NameFullyQualified,

    /**
     * Syntax: `App\Namespace`
     * Reference: [`namespaces`](https://www.php.net/manual/en/language.namespaces.php) (available as of PHP 8.0.0) 
     */
    #[cfg(feature = "php_8_0")]
    #[cfg_attr(feature = "native", serde(rename = "T_NAME_QUALIFIED"))]
    NameQualified,

    /**
     * Syntax: `namespace\Namespace`
     * Reference: [`namespaces`](https://www.php.net/manual/en/language.namespaces.php) (available as of PHP 8.0.0) 
     */
    #[cfg(feature = "php_8_0")]
    #[cfg_attr(feature = "native", serde(rename = "T_NAME_RELATIVE"))]
    NameRelative,

    /**
     * Syntax: `new`
     * Reference: [`classes and objects`](https://www.php.net/manual/en/language.oop5.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_NEW"))]
    New,

    /**
     * Syntax: `__NAMESPACE__`
     * Reference: [`namespaces`](https://www.php.net/manual/en/language.namespaces.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_NS_C"))]
    NsC,

    /**
     * Syntax: `\`
     * Reference: [`namespaces`](https://www.php.net/manual/en/language.namespaces.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_NS_SEPARATOR"))]
    NsSeparator,

    /**
     * Syntax: `"$a[0]"`
     * Reference: [`numeric array index inside string`](https://www.php.net/manual/en/language.types.string.php#language.types.string.parsing) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_NUM_STRING"))]
    NumString,

    /**
     * Syntax: `(object)`
     * Reference: [`type-casting`](https://www.php.net/manual/en/language.types.type-juggling.php#language.types.typecasting) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_OBJECT_CAST"))]
    ObjectCast,

    /**
     * Syntax: `->`
     * Reference: [`classes and objects`](https://www.php.net/manual/en/language.oop5.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_OBJECT_OPERATOR"))]
    ObjectOperator,

    /**
     * Syntax: `?->`
     * Reference: [`classes and objects`](https://www.php.net/manual/en/language.oop5.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_NULLSAFE_OBJECT_OPERATOR"))]
    NullsafeObjectOperator,

    /**
     * Syntax: `<?php, <? or <%`
     * Reference: [`escaping from HTML`](https://www.php.net/manual/en/language.basic-syntax.phpmode.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_OPEN_TAG"))]
    OpenTag,

    /**
     * Syntax: `<?= or <%=`
     * Reference: [`escaping from HTML`](https://www.php.net/manual/en/language.basic-syntax.phpmode.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_OPEN_TAG_WITH_ECHO"))]
    OpenTagWithEcho,

    /**
     * Syntax: `|=`
     * Reference: [`assignment operators`](https://www.php.net/manual/en/language.operators.assignment.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_OR_EQUAL"))]
    OrEqual,

    /**
     * Syntax: `::`
     * Reference: [`::`](https://www.php.net/manual/en/language.oop5.paamayim-nekudotayim.php). Also defined as
      [`T_DOUBLE_COLON`](https://www.php.net/manual/en/tokens.php#constant.t-double-colon). 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_PAAMAYIM_NEKUDOTAYIM"))]
    PaamayimNekudotayim,

    /**
     * Syntax: `+=`
     * Reference: [`assignment operators`](https://www.php.net/manual/en/language.operators.assignment.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_PLUS_EQUAL"))]
    PlusEqual,

    /**
     * Syntax: `**`
     * Reference: [`arithmetic operators`](https://www.php.net/manual/en/language.operators.arithmetic.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_POW"))]
    Pow,

    /**
     * Syntax: `**=`
     * Reference: [`assignment operators`](https://www.php.net/manual/en/language.operators.assignment.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_POW_EQUAL"))]
    PowEqual,

    /**
     * Syntax: `print`
     * Reference: [`print`](https://www.php.net/manual/en/function.print.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_PRINT"))]
    Print,

    /**
     * Syntax: `private`
     * Reference: [`classes and objects`](https://www.php.net/manual/en/language.oop5.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_PRIVATE"))]
    Private,

    /**
     * Syntax: `protected`
     * Reference: [`classes and objects`](https://www.php.net/manual/en/language.oop5.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_PROTECTED"))]
    Protected,

    /**
     * Syntax: `public`
     * Reference: [`classes and objects`](https://www.php.net/manual/en/language.oop5.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_PUBLIC"))]
    Public,

    /**
     * Syntax: `readonly`
     * Reference: [`classes and objects`](https://www.php.net/manual/en/language.oop5.php) (available as of PHP 8.1.0) 
     */
    #[cfg(feature = "php_8_1")]
    #[cfg_attr(feature = "native", serde(rename = "T_READONLY"))]
    Readonly,

    /**
     * Syntax: `require`
     * Reference: [`require`](https://www.php.net/manual/en/function.require.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_REQUIRE"))]
    Require,

    /**
     * Syntax: `require_once`
     * Reference: [`require_once`](https://www.php.net/manual/en/function.require-once.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_REQUIRE_ONCE"))]
    RequireOnce,

    /**
     * Syntax: `return`
     * Reference: [`returning values`](https://www.php.net/manual/en/functions.returning-values.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_RETURN"))]
    Return,

    /**
     * Syntax: `<<`
     * Reference: [`bitwise operators`](https://www.php.net/manual/en/language.operators.bitwise.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_SL"))]
    Sl,

    /**
     * Syntax: `<<=`
     * Reference: [`assignment operators`](https://www.php.net/manual/en/language.operators.assignment.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_SL_EQUAL"))]
    SlEqual,

    /**
     * Syntax: `<=>`
     * Reference: [`comparison operators`](https://www.php.net/manual/en/language.operators.comparison.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_SPACESHIP"))]
    Spaceship,

    /**
     * Syntax: `>>`
     * Reference: [`bitwise operators`](https://www.php.net/manual/en/language.operators.bitwise.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_SR"))]
    Sr,

    /**
     * Syntax: `>>=`
     * Reference: [`assignment operators`](https://www.php.net/manual/en/language.operators.assignment.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_SR_EQUAL"))]
    SrEqual,

    /**
     * Syntax: `<<<`
     * Reference: [`heredoc syntax`](https://www.php.net/manual/en/language.types.string.php#language.types.string.syntax.heredoc) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_START_HEREDOC"))]
    StartHeredoc,

    /**
     * Syntax: `static`
     * Reference: [`variable scope`](https://www.php.net/manual/en/language.variables.scope.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_STATIC"))]
    Static,

    /**
     * Syntax: `parent, self, etc.`
     * Reference: identifiers, e.g. keywords like parent and self,
      function names, class names and more are matched.
      See also [`T_CONSTANT_ENCAPSED_STRING`](https://www.php.net/manual/en/tokens.php#constant.t-constant-encapsed-string). 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_STRING"))]
    String,

    /**
     * Syntax: `(string)`
     * Reference: [`type-casting`](https://www.php.net/manual/en/language.types.type-juggling.php#language.types.typecasting) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_STRING_CAST"))]
    StringCast,

    /**
     * Syntax: `"${a`
     * Reference: [`complex variable parsed syntax`](https://www.php.net/manual/en/language.types.string.php#language.types.string.parsing.complex) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_STRING_VARNAME"))]
    StringVarname,

    /**
     * Syntax: `switch`
     * Reference: [`switch`](https://www.php.net/manual/en/control-structures.switch.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_SWITCH"))]
    Switch,

    /**
     * Syntax: `throw`
     * Reference: [`Exceptions`](https://www.php.net/manual/en/language.exceptions.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_THROW"))]
    Throw,

    /**
     * Syntax: `trait`
     * Reference: [`Traits`](https://www.php.net/manual/en/language.oop5.traits.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_TRAIT"))]
    Trait,

    /**
     * Syntax: `__TRAIT__`
     * Reference: [`__TRAIT__`](https://www.php.net/manual/en/language.constants.magic.php#constant.trait) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_TRAIT_C"))]
    TraitC,

    /**
     * Syntax: `try`
     * Reference: [`Exceptions`](https://www.php.net/manual/en/language.exceptions.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_TRY"))]
    Try,

    /**
     * Syntax: `unset()`
     * Reference: [`unset()`](https://www.php.net/manual/en/function.unset.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_UNSET"))]
    Unset,

    /**
     * Syntax: `(unset)`
     * Reference: [`type-casting`](https://www.php.net/manual/en/language.types.type-juggling.php#language.types.typecasting) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_UNSET_CAST"))]
    UnsetCast,

    /**
     * Syntax: `use`
     * Reference: [`namespaces`](https://www.php.net/manual/en/language.namespaces.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_USE"))]
    Use,

    /**
     * Syntax: `var`
     * Reference: [`classes and objects`](https://www.php.net/manual/en/language.oop5.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_VAR"))]
    Var,

    /**
     * Syntax: `$foo`
     * Reference: [`variables`](https://www.php.net/manual/en/language.variables.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_VARIABLE"))]
    Variable,

    /**
     * Syntax: `while`
     * Reference: [`while`](https://www.php.net/manual/en/control-structures.while.php), [`do..while`](https://www.php.net/manual/en/control-structures.do.while.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_WHILE"))]
    While,

    /**
     * Syntax: `\t \r\n`
     * Reference:  
     */
    #[cfg_attr(feature = "native", serde(rename = "T_WHITESPACE"))]
    Whitespace,

    /**
     * Syntax: `^=`
     * Reference: [`assignment operators`](https://www.php.net/manual/en/language.operators.assignment.php) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_XOR_EQUAL"))]
    XorEqual,

    /**
     * Syntax: `yield`
     * Reference: [`generators`](https://www.php.net/manual/en/language.generators.syntax.php#control-structures.yield) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_YIELD"))]
    Yield,

    /**
     * Syntax: `yield from`
     * Reference: [`generators`](https://www.php.net/manual/en/language.generators.syntax.php#control-structures.yield.from) 
     */
    #[cfg_attr(feature = "native", serde(rename = "T_YIELD_FROM"))]
    YieldFrom,

}
