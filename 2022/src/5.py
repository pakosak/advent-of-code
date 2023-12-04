#         "C" "B" "H"
# "W"     "D" "J" "Q" "B"
# "P" "F" "Z" "F" "B" "L"
# "G" "Z" "N" "P" "J" "S" "V"
# "Z" "C" "H" "Z" "G" "T" "Z"     "C"
# "V" "B" "M" "M" "C" "Q" "C" "G" "H"
# "S" "V" "L" "D" "F" "F" "G" "L" "F"
# "B" "J" "V" "L" "V" "G" "L" "N" "J"
#  1   2   3   4   5   6   7   8   9

# stacks = [
#     ["W","P","G","Z","V","S","B"],
#     ["F","Z","C","B","V","J"],
#     ["C","D","Z","N","H","M","L","V"],
#     ["B","J","F","P","Z","M","D","L"],
#     ["H","Q","B","J","G","C","F","V"],
#     ["B","L","S","T","Q","F","G"],
#     ["V","Z","C","G","L"],
#     ["G","L","N"],
#     ["C","H","F","J"]
# ]

import re

stacks = [
    ['B', 'S', 'V', 'Z', 'G', 'P', 'W'],
    ['J', 'V', 'B', 'C', 'Z', 'F'],
    ['V', 'L', 'M', 'H', 'N', 'Z', 'D', 'C'],
    ['L', 'D', 'M', 'Z', 'P', 'F', 'J', 'B'],
    ['V', 'F', 'C', 'G', 'J', 'B', 'Q', 'H'],
    ['G', 'F', 'Q', 'T', 'S', 'L', 'B'],
    ['L', 'G', 'C', 'Z', 'V'],
    ['N', 'L', 'G'],
    ['J', 'F', 'H', 'C']
]


commands = []
with open("5.input") as fin:
    for line in fin.readlines():
        s = re.search("move ([0-9]+) from ([0-9]+) to ([0-9]+)", line)
        gr = s.groups()
        commands.append((int(gr[0]), int(gr[1]) - 1, int(gr[2]) - 1))

print(commands)

for cnt, src, dst in commands:
    part = stacks[src][-cnt:]
    for _ in range(cnt):
        stacks[src].pop()
        # stacks[dst].append(stacks[src].pop())

    stacks[dst] += part

for s in stacks:
    print(s)

for s in stacks:
    print(s.pop(),end="")
