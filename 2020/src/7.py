
system = {}
with open("7.input") as fin:
    for line in fin.readlines():
        name, rest = line.split(" contain ")
        name = " ".join(name.split()[0:2])
        system[name] = []
        for bag in rest.split(","):
            bag_name = " ".join(bag.split()[1:3])
            if bag_name != "other bags.":
                bag_cnt = int(bag.split()[0])
                system[name].append((bag_name, bag_cnt))

print(system)


checked = set()
def have_gold(name):
    if name in checked:
        return False
    else:
        checked.add(name)
    contents = system[name]
    if "shiny gold" in [x[0] for x in contents]:
        return True
    else:
        for x in contents:
            if have_gold(x[0]):
                return True
    return False


cunt = 0
for bag in system:
    checked.clear()
    cunt += have_gold(bag)
print(cunt, len(system))

def count_content(name, cnt=1):
    return cnt + cnt*sum(count_content(x, cnt) for x, cnt in system[name])

print(count_content("shiny gold") - 1)

