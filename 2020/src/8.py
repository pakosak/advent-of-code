import copy

instrs = []
with open("8.input") as fin:
    for line in fin.readlines():
        instr, op = line.split()
        instrs.append([instr, int(op)])

def do_round(instr_set):
    acc = 0
    ip = 0
    cunt = 0
    done = set()

    def do(ip):
        nonlocal acc
        if ip >= len(instr_set):
            return None
        what, op = instr_set[ip]
        if what == "acc":
            acc += op
            return ip + 1
        elif what == "nop":
            return ip + 1
        elif what == "jmp":
            return ip + op
        else:
            raise RuntimeError(f"unknown instr {what}")

    while True:
        cunt += 1
        done.add(ip)
        ip = do(ip)
        if ip is None:
            break
        if ip in done:
            # print(cunt)
            return False
    return acc

changed_idx = 0
while changed_idx < len(instrs):
    instr_set = copy.deepcopy(instrs)
    while instr_set[changed_idx][0] not in ["nop", "jmp"]:
        changed_idx += 1
    instr_set[changed_idx][0] = "nop" if instr_set[changed_idx][0] == "jmp" else "jmp"

    acc_val = do_round(instr_set)
    if acc_val:
        print(acc_val)
        break

    changed_idx += 1



