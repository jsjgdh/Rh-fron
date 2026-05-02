import sys

file_path = "/home/kulratan/Rhexiom/Rh-fron/assets/main.css"

with open(file_path, "r") as f:
    lines = f.readlines()

new_lines = []
skip = False
for line in lines:
    if ".grid-line {" in line or ".pulse-line {" in line or "@keyframes pulse-v" in line or "@keyframes pulse-h" in line:
        skip = True
    
    if not skip:
        new_lines.append(line)
    
    if skip and line.strip() == "}":
        # Check if we are at the end of a block
        # keyframes have nested braces, this simple logic might need refinement if keyframes are complex
        # but for these simple ones it should work
        skip = False

with open(file_path, "w") as f:
    f.writelines(new_lines)
