

with open("1.input") as fin:
    data = sorted(int(x) for x in fin.readlines())

print(data)

start = 0
end = len(data) - 1


def find_inner(start, end, base):
    while start < end:
        sum_ = data[base] + data[start] + data[end]
        if sum_ == 2020:
            return (start, end)
        elif sum_ > 2020:
            end -= 1
        elif sum_ < 2020:
            start += 1
    return None

while start < end:
    res = find_inner(start+1, end, start)
    if res:
        middle, end = res
        break
    start += 1

print(data[start], data[middle], data[end], data[start] * data[middle] *data[end])
