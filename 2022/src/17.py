import itertools


class Point:
    def __init__(self, x, y):
        self.x = x
        self.y = y

    def move(self, where):
        if where == "<":
            return Point(self.x - 1, self.y)
        elif where == ">":
            return Point(self.x + 1, self.y)
        elif where == "v":
            return Point(self.x, self.y - 1)

class Shape:
    def __init__(self, pts):
        self.points = pts

    def move(self, where, world):
        moved = False
        new_pts = [p.move(where) for p in self.points]
        if not world.collides(Shape(new_pts)):
            self.points = new_pts
            moved = True
        return moved

    def highest(self):
        return max(p.y for p in self.points)

class Line(Shape):
    def __init__(self, x_base, y_base):
        self.points = [
            Point(0 + x_base, 0 + y_base),
            Point(1 + x_base, 0 + y_base),
            Point(2 + x_base, 0 + y_base),
            Point(3 + x_base, 0 + y_base),
        ]

    def next_one(self):
        return Cross

class Cross(Shape):
    def __init__(self, x_base, y_base):
        self.points = [
            Point(1 + x_base, 0 + y_base),
            Point(0 + x_base, 1 + y_base),
            Point(1 + x_base, 1 + y_base),
            Point(2 + x_base, 1 + y_base),
            Point(1 + x_base, 2 + y_base),
        ]

    def next_one(self):
        return L

class Cross(Shape):
    def __init__(self, x_base, y_base):
        self.points = [
            Point(1 + x_base, 0 + y_base),
            Point(0 + x_base, 1 + y_base),
            Point(1 + x_base, 1 + y_base),
            Point(2 + x_base, 1 + y_base),
            Point(1 + x_base, 2 + y_base),
        ]

    def next_one(self):
        return L

class L(Shape):
    def __init__(self, x_base, y_base):
        self.points = [
            Point(0 + x_base, 0 + y_base),
            Point(1 + x_base, 0 + y_base),
            Point(2 + x_base, 0 + y_base),
            Point(2 + x_base, 1 + y_base),
            Point(2 + x_base, 2 + y_base),
        ]

    def next_one(self):
        return I

class I(Shape):
    def __init__(self, x_base, y_base):
        self.points = [
            Point(0 + x_base, 0 + y_base),
            Point(0 + x_base, 1 + y_base),
            Point(0 + x_base, 2 + y_base),
            Point(0 + x_base, 3 + y_base),
        ]

    def next_one(self):
        return Square

class Square(Shape):
    def __init__(self, x_base, y_base):
        self.points = [
            Point(0 + x_base, 0 + y_base),
            Point(0 + x_base, 1 + y_base),
            Point(1 + x_base, 0 + y_base),
            Point(1 + x_base, 1 + y_base),
        ]

    def next_one(self):
        return Line

WIDTH = 7
class World:
    def __init__(self):
        self.rows = []
        self.top = 0
        self.count = 0

        self.active = Line(2, self.top + 3)
        self.active_moves = 0

        self.add_row()

    def new_shape(self):
        self.active = self.active.next_one()(2, self.top + 3)

    def move(self, where):
        m = self.active.move(where, self)
        self.active_moves += 1

    def down(self):
        if not self.active.move("v", self):
            self.top = max(self.top, self.active.highest() + 1)
            self.add(self.active)
            self.new_shape()
            self.active_moves = 0
            return True
        return False

    def add_row(self):
        self.rows.append([0 for _ in range(WIDTH)])

    def add(self, shape):
        for p in shape.points:
            while p.y >= len(self.rows):
                self.add_row()
            self.rows[p.y][p.x] = 1
        self.count += 1
        # self.print()

    def collides(self, shape):
        for p in shape.points:
            if not ((0 <= p.x < WIDTH) and p.y >= 0):
                return True

            if p.y >= len(self.rows):
                continue

            if self.rows[p.y][p.x]:
                return True
        return False

    def print(self):
        for row in reversed(self.rows):
            for val in row:
                print("#" if val else ".", end="")
            print()
        print()

    def ff(self, top_bonus, count_bonus):
        self.top += top_bonus
        self.count += count_bonus

with open("17.input") as fin:
    arrows = fin.read().strip()

world = World()
i = 0
cycle = 0

past = []
repetition = []
rep_steps = []
prev_top = 0

while world.count < 1_000_000_000_000:
    if i % len(arrows) == 0:
        key = (world.active.__class__.__name__, world.active_moves)
        # print(key)
        # print(world.top, "\t", world.count)
        if repetition and key == repetition[0]:
            top_bonus = world.top - start_rep_top
            count_bonus = world.count - start_rep_count
            print(repetition, top_bonus, count_bonus)
            print(rep_steps)

            ff_count = (1_000_000_000_000 - world.count) // count_bonus

            world.ff(ff_count * top_bonus, ff_count * count_bonus)
            print(f"ff to {world.top} {world.count}")

            for i in range(1_000_000_000_000 - world.count):
                world.top += rep_steps[i]
                world.count += 1

            repetition.clear()

        if key in past:
            if not repetition:
                start_rep_top = world.top
                start_rep_count = world.count
                prev_top = world.top
            repetition.append(key)
        else:
            repetition.clear()
            rep_steps.clear()
        past.append(key)
        cycle += 1


    arrow = arrows[i % len(arrows)]
    world.move(arrow)
    if world.down() and repetition:
        rep_steps.append(world.top - prev_top)
        prev_top = world.top

    i += 1

print(world.top)
