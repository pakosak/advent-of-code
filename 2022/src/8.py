grid = []
with open("8.input") as fin:
    for line in fin.readlines():
        grid.append(line.strip())


row_len = len(grid[0])
col_len = len(grid)

print(row_len, col_len)


def is_visible(tree, row, col):
    def u():
        for row_i in reversed(range(row)):
            if grid[row_i][col] >= tree:
                return False
        return True
    def r():
        for col_i in range(col+1, row_len):
            if grid[row][col_i] >= tree:
                return False
        return True
    def d():
        for row_i in range(row+1, col_len):
            if grid[row_i][col] >= tree:
                return False
        return True
    def l():
        for col_i in reversed(range(col)):
            if grid[row][col_i] >= tree:
                return False
        return True

    return any((u(), r(), d(), l()))

def scenic_score(tree, row, col):
    def u():
        score = 0
        for row_i in reversed(range(row)):
            score += 1
            if grid[row_i][col] >= tree:
                break
        return score
    def r():
        score = 0
        for col_i in range(col+1, row_len):
            score += 1
            if grid[row][col_i] >= tree:
                break
        return score
    def d():
        score = 0
        for row_i in range(row+1, col_len):
            score += 1
            if grid[row_i][col] >= tree:
                break
        return score
    def l():
        score = 0
        for col_i in reversed(range(col)):
            score += 1
            if grid[row][col_i] >= tree:
                break
        return score

    print(row, col, tree, "\t", u(), r(), d(), l())
    return u() * r() * d() * l()

def pt1():
    visible = 0

    for row_i, row in enumerate(grid):
        for col_i, tree in enumerate(row):
            if is_visible(tree, row_i, col_i):
                visible += 1

    print(visible)

def pt2():
    best = 0
    for row_i, row in enumerate(grid):
        for col_i, tree in enumerate(row):
            best = max(best, scenic_score(tree, row_i, col_i))

    print(best)

pt2()

