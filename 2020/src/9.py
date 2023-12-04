with open("9.input") as fin:
    nums = [int(x) for x in fin.readlines()]

print(nums)


def am_i_sum(me):
    for i in range(me-25, me):
        for j in range(me-25, me):
            if i == j:
                continue
            if nums[i] + nums[j] == nums[me]:
                return True
    return False


i = 25
while i < len(nums):
    if not am_i_sum(i):
        break
    i += 1

print(i, nums[i])

weak = nums[i]

begin = 0
end = 0
suma = 0
while True:
    if suma > weak:
        suma -= nums[begin]
        begin += 1
    elif suma < weak:
        suma += nums[end]
        end += 1
    else:
        break

print(begin, end)
print(nums[begin:end], min(nums[begin:end]) +  max(nums[begin:end]))
