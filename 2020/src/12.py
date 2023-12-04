with open("12.input") as fin:
    commands = [x.strip() for x in fin.readlines()]

print(commands)

forward_fun = {
    1: (lambda x,y,val: (y+val,y)),
    3: (lambda x,y,val: (x-val,y)),
    0: (lambda x,y,val: (x,y+val)),
    2: (lambda x,y,val: (x,y-val)),
}

facing = 1
x = 0
y = 0


directions = [0,1,2,3]
directions_transform = ["N","E","S","W"]

def do(command):
    global facing, x, y

    command, val = command[0], int(command[1:])
    print(command, val)
    if command == "R":
        facing = directions[(facing + val//90)%4]
    elif command == "L":
        facing = directions[(facing - val//90)%4]
    elif command == "F":
        x,y = forward_fun[facing](x,y,val)
    elif command == "N":
        y += val
    elif command == "E":
        x += val
    elif command == "S":
        y -= val
    elif command == "W":
        x -= val
    else:
        print(f"wrird command {command}")
    print(x,y,directions_transform[facing])

for command in commands:
    do(command)

print(x,y,facing)
print(abs(x)+ abs(y))
