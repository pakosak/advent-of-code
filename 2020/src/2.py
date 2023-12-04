

def is_ok(line):
    cnts, letter, passwd = line.split()
    min_, max_ = [int(x) for x in cnts.split("-")]
    letter = letter[:1]
    print(min_, max_, letter, passwd)

    occurence = passwd.count(letter)
    return min_ <= occurence <= max_

def is_ok2(line):
    cnts, letter, passwd = line.split()
    idx1, idx2 = [int(x)-1 for x in cnts.split("-")]
    letter = letter[:1]

    # print(idx1+1, idx2+1, letter, passwd, passwd[idx1] == letter or passwd[idx2] == letter)
    return (passwd[idx1] == letter) ^ (passwd[idx2] == letter)

ok_cnt = 0
with open("2.input") as fin:
    for line in fin.readlines():
        ok_cnt += is_ok2(line)

print(ok_cnt)

