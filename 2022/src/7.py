from pprint import pprint

with open("7.input") as fin:
    lines = fin.readlines()


class Dir(dict):
    def __init__(self, prev):
        self.prev = prev
        self.size = 0

    def get_size(self):
        if self.size == 0:
            for sub in self.values():
                if type(sub) == int:
                    self.size += sub
                else:
                    self.size += sub.get_size()
        return self.size


pwd = ""
current = Dir(None)
current["/"] = Dir(current)
root = current
prev = None

for line in lines:
    if line.startswith("$"):
        s = line.split()
        if len(s) == 3:
            if s[2] == "..":
                current = current.prev
            else:
                current = current[s[2]]
    else:
        dir_or_size, name = line.split()
        if dir_or_size == "dir":
            current[name] = Dir(current)
        else:
            current[name] = int(dir_or_size)

pprint(root)
print(root.get_size())

def pt1():
    small = 0

    def print_size(dir_):
        global small
        for name, subdir in dir_.items():
            if type(subdir) != int:
                # print(name, subdir.size)
                if subdir.size < 100000:
                    small += subdir.size
                print_size(subdir)

    print_size(root)
    print(small)

def pt2():
    goal = 30000000 - (70000000 - root.get_size())
    print(f"goal: {goal}")

    def find_exact(name, dir_):
        if type(dir_) != int:
            if dir_.size > goal:
                print(name, dir_.size)
            # else:
            #     print(name, dir_.size)
            for subdir_name, subdir in dir_.items():
                find_exact(subdir_name, subdir)

    find_exact("/", root)

pt2()








