file_path = "src/tokens.rs"

with open(file_path, 'r') as file:
    # Use the print() function to write to the file
    file_content = file.read()

for l in range(len(file_content) - 1):
    if file_content[l] == "T":
        identifier = ""
        while not file_content[l].isspace():
            identifier += file_content[l]
            print(identifier)
            l += 1
        for i in range(9):
            print(file_content[l + i], end="")