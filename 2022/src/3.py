with open("3.input") as fin:
    rucksacks = [x.strip() for x in fin.readlines()]

# print(rucksacks)

def prio(v):
    if v.isupper():
        return ord(v) - ord("A") + 26 + 1
    else:
        return ord(v) - ord("a") + 1

def pt1():
    sum_ = 0
    for ruck in rucksacks:
        size = len(ruck)//2
        a, b = set(ruck[:size]), set(ruck[size:])
        # print(a,b,ruck)

        inter = a.intersection(b).pop()
        val = prio(inter)
        print(inter, val)
        sum_ += val

    print(sum_)

def pt2():
    sum_ = 0

    groups = [rucksacks[n:n+3] for n in range(0, len(rucksacks), 3)]
    # print(groups)
    for i, group in enumerate(groups):
        print(i, group)
        badge = set.intersection(*[set(elf) for elf in group]).pop()
        print(badge, prio(badge))
        sum_ += prio(badge)

    print(sum_)

pt2()


