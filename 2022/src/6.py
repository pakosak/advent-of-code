with open("6.input") as fin:
    code = fin.read()



i = 0
while True:
    if len(set(code[i:i+14])) == 14:
        print(i)
        break
    i += 1



