import re

rust_enum = """
#[derive(Debug)]
pub enum TokenType {
    // Keywords
    VAR,
    CONST,
    FUNC,
    FOR,
    WHILE,
    IF,
    ELSE,
    WHEN,
    USE,


    // Other keywords
    

    // Special character
    LCURLY,       // {
    RCURLY,       // }
    LPARENT,     // (
    RPARENT,     // )
    LSQUAREBRAC, // [
    RSQUAREBRAC, // ]
    COMMENT,     // //
    QUOTMARK,   // "
    EXCLAMMARK,
    COMMA,       // ,
    COLON,       // :

    // Identifier, literals
    IDENT,
    NUMBER,
    FALSE,
    TRUE,

    // Other
    ILLEGAL, // illeagl expression
    EOL,     // End of line

    // Arithmetic operators and alike
    PLUS,        // +
    MINUS,       // -
    DIVIDE,      // /
    MULTIPLY,    // *
    ASSIGN,      // =

    EQUAL,       // ==
    NOTEQUAL,    // !=
    GREATERTHAN, // >
    LESSERTHAN,    // <
    GREATEROREQUALTHAN, // >=
    LESSEROREQUALTHAN,    // <=

    AND,
    OR,
    IN,
    ARROW, // ->
    OTHER,

    // Structures
    STRUCT,
    ENUM,
}
"""

# Use regular expression to find the entire enum block
enum_block = re.search(r'enum TokenType \{([\s\S]*?)\n\}', rust_enum).group(1)

# Split the enum block by lines and filter out the comments and curly braces
enum_entries = [entry.strip() for entry in enum_block.split('\n') if not entry.strip().startswith("//") and entry.strip() not in ["LCURLY,", "RCURLY,"]]

# Get the last enum entry
last_entry = enum_entries[-1]

print("Last Enum Entry:", last_entry)
