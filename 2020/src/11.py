import copy

with open("11.input") as fin:
    seats = [list(l.strip()) for l in fin.readlines()]

neighbors = {}

def compute_neighbors():
    for row_i, row in enumerate(seats):
        for col_i, seat in enumerate(row):
            if seat != ".":
                nearest = []
                directions = [
                    (lambda x,y: (x-1,y)),
                    (lambda x,y: (x,y-1)),
                    (lambda x,y: (x+1,y)),
                    (lambda x,y: (x,y+1)),
                    (lambda x,y: (x-1,y-1)),
                    (lambda x,y: (x-1,y+1)),
                    (lambda x,y: (x+1,y-1)),
                    (lambda x,y: (x+1,y+1)),
                ]
                for fun in directions:
                    row, col = fun(row_i, col_i)
                    while 0 <= row < len(seats) and 0 <= col < len(seats[row]):
                        if seats[row][col] in ("#", "L"):
                            nearest.append((row,col))
                            break
                        row, col = fun(row, col)
                neighbors[(row_i, col_i)] = nearest

def do_generation():
    def get_seat_type(row, col):
        if 0 <= row < len(seats) and 0 <= col < len(seats[row]):
            return seats[row][col]
        else:
            return ""

    def adjacent_seats(row, col, which):
        neighbors_vals = [
            get_seat_type(*coords)
            for coords in neighbors[(row,col)]
        ]
        return neighbors_vals.count(which)

    def apply(row, col, seat):
        if seat == ".":
            return "."
        else:
            occupied = adjacent_seats(row, col, "#")
            if seat == "L" and occupied == 0:
                return "#"
            elif seat == "#" and occupied >= 5:
                return "L"
            else:
                return seat

    new_seats = copy.deepcopy(seats)
    for row_i, row in enumerate(seats):
        for col_i, seat in enumerate(row):
            new_seats[row_i][col_i] = apply(row_i, col_i, seat)

    return new_seats

def plot():
    print("\n".join("".join(row) for row in seats))
    print()

compute_neighbors()

plot()
prev_gen = ""
gen_i = 0
while prev_gen != seats:
    prev_gen = copy.deepcopy(seats)
    seats = do_generation()
    gen_i += 1
    print(gen_i)
    # plot()
    # break


print(sum(col.count("#") for row in seats for col in row))
print(gen_i)

