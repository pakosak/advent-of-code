with open("4.input") as fin:
    lines = [x.strip() for x in fin.readlines()]

# print(lines)

def contains(a, b):
    return a[0] <= b[0] and a[1] >= b[1] or a[0] >= b[0] and a[1] <= b[1]

def overlaps(a, b):
    return a[1] >= b[0] and a[0] <= b[1] or b[1] >= a[0] and b[0] <= a[1]

ranges = [
    [
        [int(x) for x in r.split("-")]
        for r in line.split(",")
    ]
    for line in lines
]

def pt1():
    fully_contains = sum(contains(a,b) for a,b in ranges)

    for fst, snd in ranges:
        if contains(fst, snd):
            print(fst, snd)

    print(fully_contains)

def pt2():
    some_overlap = sum(overlaps(a,b) for a,b in ranges)

    for fst, snd in ranges:
        if not overlaps(fst, snd):
            print(fst, snd)

    print(some_overlap)

# pt1()
pt2()
