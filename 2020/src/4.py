

def validate_pp(pp):
    pp = {
        field.split(":")[0]: field.split(":")[1]
        for field in pp
    }
    def all_present():
        required_fields = {"byr","eyr","iyr","hgt","hcl","ecl","pid"}
        present_fields = set(pp)
        return required_fields & present_fields == required_fields

    def all_valid():
        return (
                1920 <= int(pp["byr"]) <= 2002
            and 2010 <= int(pp["iyr"]) <= 2020
            and 2020 <= int(pp["eyr"]) <= 2030
            and (
                ("cm" in pp["hgt"]
                and 150 <= int(pp["hgt"][:-2]) <= 193)
                or
                ("in" in pp["hgt"]
                and 59 <= int(pp["hgt"][:-2]) <= 76)
            )
            and pp["hcl"][0] == "#"
            and pp["hcl"][1:].isalnum()
            and pp["ecl"] in ("amb","blu","brn","gry","grn","hzl","oth")
            and len(pp["pid"]) == 9
        )

    return all_present() and all_valid()

valid = 0
with open("4.input") as fin:
    current_pp = []
    for line in fin.readlines():
        line = line.strip()
        if line == "":
            valid += validate_pp(current_pp)
            current_pp = []
        else:
            current_pp += line.split()

print(valid)
