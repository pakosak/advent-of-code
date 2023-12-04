rounds = []
with open("2.input") as fin:
    for line in fin.readlines():
        rounds.append(line.strip().split())

# print(rounds)


shape_points = {
    "R": 1,
    "P": 2,
    "S": 3
}

code_points = {
    "X": 0,
    "Y": 3,
    "Z": 6
}

shape_to_og = {
    "X": "R",
    "Y": "P",
    "Z": "S",
    "A": "R",
    "B": "P",
    "C": "S",
}

dom = {
    "R": "P",
    "P": "S",
    "S": "R"
}

nondom = {v: k for k, v in dom.items()}

def pt1():
    suma = 0
    for op, my in rounds:
        my_transformed = shape_to_og[my]
        op_transformed = shape_to_og[op]
        round_pts = 0
        round_pts += shape_points[my_transformed]
        if my_transformed == op_transformed:
            round_pts += 3
        elif op_transformed != dom[my_transformed]:
            round_pts += 6

        print(my, op, my_transformed, op_transformed, round_pts)
        suma += round_pts

    print(suma)

def pt2():
    suma = 0
    for op, code in rounds:
        op_transformed = shape_to_og[op]
        round_pts = 0
        round_pts += code_points[code]

        if code == "X":
            round_pts += shape_points[nondom[op_transformed]]
        elif code == "Y":
            round_pts += shape_points[op_transformed]
        else:
            round_pts += shape_points[dom[op_transformed]]

        # print(my, op, my_transformed, op_transformed, round_pts)
        suma += round_pts

    print(suma)

pt2()
