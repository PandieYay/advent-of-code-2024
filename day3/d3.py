import re

file = open("d3.txt", "r")
total = 0
for line in file:
    pattern = re.compile(r"mul\(\d{1,3},\d{1,3}\)")
    matches = pattern.findall(line)
    for match in matches:
        numbers = [int (number) for number in (re.findall(r"\d+", match))]
        total += numbers[0] * numbers[1]
print("Total:", total)