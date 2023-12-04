
lines = []
line_len = 0
with open("3.input") as fin:
    for line in fin.readlines():
        lines.append(line.strip())

row_len = len(lines[0])
row_cnt = len(lines)

def is_tree(row, col):
    return lines[row][col%row_len] == "#"


def move_slope(right, down):
    current_row = 0
    current_col = 0
    trees = 0
    while current_row < row_cnt:
        trees += is_tree(current_row, current_col)
        current_col += right
        current_row += down
    return trees

print(move_slope(1,1) * move_slope(3,1) * move_slope(5,1) * move_slope(7,1) * move_slope(1,2))




