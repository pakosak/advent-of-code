
commands = []

with open("10.input") as fin:
    for line in fin.readlines():
        s = line.split()
        if len(s) == 1:
            commands.append((False, 0))
        else:
            commands.append((True, int(s[1])))

print(commands)

def pt1():

    signal = 1

    strength = 0

    def track(cycle, com, val):
        nonlocal strength

        if cycle % 40 == 19:
            print(cycle, com, val, signal, sep="\t")
            # print()
            strength += (cycle+1) * signal

    cycle = 0

    for idx, (com, val) in enumerate(commands):
        track(cycle, com, val)
        if com:
            cycle += 1
            track(cycle, com, val)
            cycle += 1
            signal += val
        else:
            cycle += 1

        if idx == len(commands) - 1:
            track(cycle, com, val)


    print(strength)

def pt2():
    def track(cycle, com, val):
        if cycle and cycle % 40 == 0:
            print()
        # print(cycle, com, signal, sep="\t")
        if abs((signal) - (cycle % 40)) <= 1:
            print("#", end="")
        else:
            print(".", end="")
        # print(i)

    cycle = 0
    signal = 1

    for idx, (com, val) in enumerate(commands):
        track(cycle, com, val)
        if com:
            cycle += 1
            track(cycle, com, val)
            cycle += 1
            signal += val
        else:
            cycle += 1

        if idx == len(commands) - 1:
            track(cycle, com, val)

pt2()
