file_path = "src/tokens.rs"

with open(file_path, 'r') as file:
    # Use the print() function to write to the file
    file_content = file.read()

identifier = ""
identifier2 = ""
for l in range(len(file_content) - 1):
    if file_content[l] == "T":
        for x in range(len("TokenType")):
            identifier += file_content[l + x]
    if identifier == "TokenType":
        for y in range(5):
            identifier2 += file_content[l - y - 1]
        break

print(identifier)
print(identifier2[::-1])