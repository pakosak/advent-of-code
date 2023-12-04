cunts = []
with open("6.input") as fin:
    group = set()
    new = True
    for line in fin.readlines():
        line = line.strip()
        if line == "":
            cunts.append(len(group))
            print(group)
            group = set()
            new = True
        else:
            if new:
                group = set(line)
                new = False
            else:
                group &= set(line)
    cunts.append(len(group))
print(cunts)
print(sum(cunts))


