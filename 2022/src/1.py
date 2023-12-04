import itertools

with open("1.input") as fin:
    data = fin.readlines()

data = [int(x.strip()) if x != "\n" else None for x in data]


cals_by_elf = filter(lambda gr: gr[0], itertools.groupby(data, lambda line: line != None))
# for _, c in cals_by_elf:
#     print(list(c))

sorte = sorted((sum(cals) for _, cals in cals_by_elf))
print(sum(sorte[-3:]))



