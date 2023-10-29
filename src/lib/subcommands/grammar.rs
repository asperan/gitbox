use clap::Args;

// This is a phony command used to print a big help message for the grammar of Triggers
#[derive(Args, Debug)]
#[command(
    hide(true),
    help_template = "{before-help}",
    before_long_help = r#"
NOTES:
    Below there is the grammar of Triggers used in various subcommands.

    The symbols "WHITESPACE", "SOI", "EOI", "NEWLINE" and "ASCII_ALPHA_LOWER" are defined by pest crate.

    This language allows to parse expressions like:
    ```
        scope IN [core-deps, frontend] AND (type IN [ test, feat ] OR breaking)
    ```

    The "AND" operator has the precedence over the "OR" operator, so removing the parenthesis from the expression above is equivalent to (ipothetically, it is not permitted by the grammar) associate the predicates like so:
    ```
        (scope IN [core-deps, frontend] AND type IN [ test, feat ]) OR breaking
    ```
    and the equivalent permitted by the grammar is:
    ```
        scope IN [core-deps, frontend] AND type IN [ test, feat ] OR breaking
    ```

    Below the grammar there are some examples, and a link to the grammar specs on Grammophone.

GRAMMAR:

    WHITESPACE   =  _{ " " | "\t" | NEWLINE }

    START = _{ SOI ~ (OR_STMT | AND_STMT | STMT) ~ EOI}

    // Statements

    AND_STMT = { (PAR_STMT | STMT) ~ "AND" ~ (PAR_STMT | AND_STMT | STMT) }

    PAR_STMT = { "(" ~ OR_STMT ~ ")" }

    OR_STMT = { (AND_STMT | STMT) ~ "OR" ~ (AND_STMT | OR_STMT | STMT) }

    STMT = { BREAKING_STMT | ARRAY_STMT }

    ARRAY_STMT = { OBJECT ~ "IN" ~ ARRAY }

    BREAKING_STMT = { "breaking" }

    // Basics
    ARRAY = { "[" ~ LITERAL ~ ("," ~ LITERAL)* ~ "]" }

    OBJECT = { TYPE_OBJECT | SCOPE_OBJECT }

    TYPE_OBJECT = { "type" }

    SCOPE_OBJECT = { "scope" }

    LITERAL = @{ ASCII_ALPHA_LOWER ~ (ASCII_ALPHA_LOWER | "-")* }

EXAMPLES:

    * Triggers on commits with the type equal to 'chore' and the scope equal to 'core-deps':
        type IN [ chore ] AND scope IN [ core-deps ]

    * Triggers on commits with the type equal to 'test' and the scope equal to 'lib' or 'backend':
        type IN [ test ] AND scope IN [ lib, backend ]

EXTERNAL RESOURCES:
    * Grammophone: https://mdaines.github.io/grammophone/?s=U1RBUlQgLT4gT1JfU1RNVCB8IEFORF9TVE1UIHwgU1RNVCAuCgpBTkRfU1RNVCAtPiBGSVJTVF9BTkRfVkFMVUUgIkFORCIgU0VDT05EX0FORF9WQUxVRSAuCgpGSVJTVF9BTkRfVkFMVUUgLT4gUEFSX1NUTVQgfCBTVE1UIC4KU0VDT05EX0FORF9WQUxVRSAtPiBQQVJfU1RNVCB8IEFORF9TVE1UIHwgU1RNVCAuCgpQQVJfU1RNVCAtPiAiKCIgT1JfU1RNVCAiKSIgLgoKT1JfU1RNVCAtPiBGSVJTVF9PUl9TVE1UICJPUiIgU0VDT05EX09SX1NUTVQgLgoKRklSU1RfT1JfU1RNVCAtPiBBTkRfU1RNVCB8IFNUTVQgLgpTRUNPTkRfT1JfU1RNVCAtPiBBTkRfU1RNVCB8IE9SX1NUTVQgfCBTVE1UIC4KClNUTVQgLT4gT0JKRUNUICJJTiIgQVJSQVkgfCAiYnJlYWtpbmciIC4KCgpBUlJBWSAtPiAiWyIgQVJSQVlfRUxFTUVOVCAiXSIgLgoKQVJSQVlfRUxFTUVOVCAtPiBMSVRFUkFMIHwgTElURVJBTCAiLCIgQVJSQVlfRUxFTUVOVCAuCgpPQkpFQ1QgLT4gInR5cGUiIHwgInNjb3BlIiAuCgojIExpdGVyYWxzIGRvIG5vdCBjb250YWluIHNwYWNlcwpMSVRFUkFMIC0+IExFVFRFUiB8IExFVFRFUiBSRVNUIC4KUkVTVCAtPiBDSEFSQUNURVIgfCBDSEFSQUNURVIgUkVTVCAuCgpDSEFSQUNURVIgLT4gTEVUVEVSIHwgIi0iIC4KCkxFVFRFUiAtPiBhIC4gI3wgYiB8IGMgfCBkIHwgZSB8IGYgfCBnIHwgaCB8IGkgfCBqIHwgayB8IGwgfCBtIHwgbiB8IG8gfCBwIHwgcSB8IHIgfCBzIHwgdCB8IHUgfCB2IHwgdyB8IHggfCB5IHwgeiAuCg==

"#
)]
pub struct GrammarSubCommand {}
