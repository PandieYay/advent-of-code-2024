import re

with open("d3.txt", "r") as file:
    contents = file.read()

mul_pattern = re.compile(r"mul\(\d{1,3},\d{1,3}\)")
do_pattern = re.compile(r"do\(\)")
dont_pattern = re.compile(r"don't\(\)")

total = 0
index = 0
mul_enabled = True

while index < len(contents):
    if do_pattern.match(contents, index):
        mul_enabled = True
        index += len("do()")
        continue

    if dont_pattern.match(contents, index):
        mul_enabled = False
        index += len("don't()")
        continue

    mul_match = mul_pattern.match(contents, index)
    if mul_match and mul_enabled:
        numbers = [int(number) for number in re.findall(r"\d+", mul_match.group())]
        total += numbers[0] * numbers[1]
        index += len(mul_match.group())
    else:
        index += 1

print("Total:", total)