import sys

with open (sys.argv[2], "r") as f:
    tree_grid = f.read().splitlines()

if sys.argv[1] == "1":
    # calculate number of visible trees on the edge
    visible_trees = len(tree_grid) * 2 + (len(tree_grid[0]) - 2) * 2
    for i in range(len(tree_grid) - 2):
        for j in range(len(tree_grid[0]) - 2):
            tree = tree_grid[i + 1][j + 1]
            left = list(tree_grid[i + 1][:j + 1])
            right = list(tree_grid[i + 1][j + 2:])
            up = []
            down = []
            for x in range(0, i + 1):
                up.append(tree_grid[x][j + 1])
            for x in range(i + 2, len(tree_grid)):
                down.append(tree_grid[x][j + 1])

            if max(left) < tree or max(right) < tree or max(up) < tree or max(down) < tree:
                visible_trees += 1

    print(f"visible trees: {visible_trees}")
elif sys.argv[1] == "2":

    def scenic_score(left:int, right:int , up:int, down: int) -> int:    
        return (left * right * up * down)
    
    max_current_score = 0

    rows_count = len(tree_grid)
    cols_count = len(tree_grid[0])

    for i in range(rows_count):
        for j in range(cols_count):
            tree = tree_grid[i][j]
            left = []
            right = []
            up = []
            down = []
            for x in range(i - 1, -1, -1):
                up.append(tree_grid[x][j])
                if tree_grid[x][j] >= tree:
                    break
            for x in range(i + 1, rows_count):
                    down.append(tree_grid[x][j])
                    if tree_grid[x][j] >= tree: 
                        break
            for x in range(j - 1, -1, -1):
                left.append(tree_grid[i][x])
                if tree_grid[i][x] >= tree:
                    break
            for x in range(j + 1, cols_count):
                    right.append(tree_grid[i][x])
                    if tree_grid[i][x] >= tree: 
                        break
            
            if max_current_score < scenic_score(len(left), len(right), len(up), len(down)):
                max_current_score = scenic_score(len(left), len(right), len(up), len(down))
    print(f"max scenic score: {max_current_score}")
else:
    print("Usage: python main.py <1|2> <input file>")
