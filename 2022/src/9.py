
commands = []
with open("9.input") as fin:
    for line in fin.readlines():
        d, val = line.split()
        commands.append((d, int(val)))

class Point:
    def __init__(self):
        self.x = 0
        self.y = 0

    def is_far(self, other):
        return abs(other.x - self.x) > 1 or abs(other.y - self.y) > 1

    def move(self, where):
        if where == "D":
            self.y -= 1
        elif where == "L":
            self.x -= 1
        elif where == "U":
            self.y += 1
        elif where == "R":
            self.x += 1

    def __eq__(self, other):
        return self.x == other.x and self.y == other.y

    def pos(self):
        return (self.x, self.y)

    def move_closer(self, other):
        if self == other:
            print("bad")

        if self.x == other.x:
            if self.y < other.y:
                self.y += 1
            else:
                self.y -= 1
        elif self.y == other.y:
            if self.x < other.x:
                self.x += 1
            else:
                self.x -= 1
        else:
            if self.x < other.x and self.y < other.y:
                self.x += 1
                self.y += 1
            elif self.x < other.x and self.y > other.y:
                self.x += 1
                self.y -= 1
            elif self.x > other.x and self.y < other.y:
                self.x -= 1
                self.y += 1
            elif self.x > other.x and self.y > other.y:
                self.x -= 1
                self.y -= 1


def pt1():
    head = Point()
    tail = Point()

    positions = set()
    positions.add((0,0))

    for com, repeat in commands:
        for _ in range(repeat):
            head.move(com)
            if tail.is_far(head):
                tail.move_closer(head)
                positions.add(tail.pos())

    print(len(positions))

def pt2():
    rope = [Point() for _ in range(10)]

    positions = set()
    positions.add((0,0))

    for com, repeat in commands:
        for _ in range(repeat):
            rope[0].move(com)
            for idx in range(1, 10):
                if rope[idx].is_far(rope[idx-1]):
                    rope[idx].move_closer(rope[idx-1])
            positions.add(rope[-1].pos())

    print(positions)
    print(len(positions))

pt2()
