import re

with open("src/tokens.rs") as file:
    rust_enum = file.read()

# Use regular expression to find the entire enum block
enum_block = re.search(r'enum TokenType \{([\s\S]*?)\n\}', rust_enum).group(1)

# Split the enum block by lines and filter out the comments and curly braces
enum_entries = [entry.strip() for entry in enum_block.split('\n') if not entry.strip().startswith("//") and entry.strip() not in ["LCURLY,", "RCURLY,"]]

# Get the last enum entry
last_entry = enum_entries[-1]

print("Last Enum Entry:", last_entry)
