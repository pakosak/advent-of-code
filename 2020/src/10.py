with open("10.input") as fin:
    adapters = [int(line) for line in fin.readlines()]

adapters.append(max(adapters) + 3)
adapters.append(0)

adapters.sort()

print(adapters)



diffs = {1:0, 3:0}
for idx, x in enumerate(adapters):
    if idx != len(adapters) - 1:
        diffs[adapters[idx+1]-x] += 1

print(diffs)

print(diffs[1]*diffs[3])

# part two

combs = {}

def cunt_combs(idx):
    if idx in combs:
        return combs[idx]

    def gimme_possible_next():
        neighbors = []
        i = idx + 1
        while i != len(adapters):
            if adapters[i] - adapters[idx] <= 3:
                neighbors.append(i)
            else:
                break
            i += 1
        return neighbors

    if idx == len(adapters)-1:
        return 1
    else:
        neighbors = gimme_possible_next()
        return sum(cunt_combs(n) for n in neighbors)

for idx in reversed(range(len(adapters))):
    combs[idx] = cunt_combs(idx)

transformed = {}
for idx, adapter in enumerate(adapters):
    transformed[adapter] = combs[idx]


print(combs)
print(transformed)
