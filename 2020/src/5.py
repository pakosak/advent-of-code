with open("5.input") as fin:
    boarding_passes = fin.readlines()


def bisect(indices, begin, end):
    for idx in range(len(indices)):
        step = (end - begin + 1)/2
        if indices[idx] in ("F", "L"):
            end -= step
        elif indices[idx] in ("B", "R"):
            begin += step
    assert begin == end
    return begin

def get_id(bp):
    row = bisect(bp[:7], 0, 127)
    col = bisect(bp[7:], 0, 7)
    return row * 8 + col

ids = []
for bp in boarding_passes:
    ids.append(get_id(bp))
ids = sorted(ids)

for idx in range(len(ids)):
    if ids[idx+1] - ids[idx] != 1:
        print(ids[idx])
