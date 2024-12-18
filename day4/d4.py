import re

file = open("d4.txt", "r")
xmasCounter = 0
wordSearch = file.read()
lengthOfOneLine = wordSearch.index("\n") + 1

for i in range(len(wordSearch)):
    if wordSearch[i] == "X":
        # Check forward
        if i < len(wordSearch) - 3:
            if (
                wordSearch[i + 1] == "M"
                and wordSearch[i + 2] == "A"
                and wordSearch[i + 3] == "S"
            ):
                print("Found forward XMAS")
                xmasCounter += 1
        # Check backward
        if i - 3 >= 0:
            if (
                wordSearch[i - 1] == "M"
                and wordSearch[i - 2] == "A"
                and wordSearch[i - 3] == "S"
            ):
                print("Found backward XMAS")
                xmasCounter += 1
        # Check down
        if (i + lengthOfOneLine * 3) < len(wordSearch):
            if (
                wordSearch[i + lengthOfOneLine] == "M"
                and wordSearch[i + (lengthOfOneLine * 2)] == "A"
                and wordSearch[i + (lengthOfOneLine * 3)] == "S"
            ):
                print("Found down XMAS")
                xmasCounter += 1
        # Check up
        if (i - lengthOfOneLine * 3) >= 0:
            if (
                wordSearch[i - lengthOfOneLine] == "M"
                and wordSearch[i - (lengthOfOneLine * 2)] == "A"
                and wordSearch[i - (lengthOfOneLine * 3)] == "S"
            ):
                print("Found up XMAS")
                xmasCounter += 1
        # Check diagonal down left
        if (i + lengthOfOneLine * 3 - 3) < len(wordSearch):
            if (
                wordSearch[i + lengthOfOneLine - 1] == "M"
                and wordSearch[i + (lengthOfOneLine * 2) - 2] == "A"
                and wordSearch[i + (lengthOfOneLine * 3) - 3] == "S"
            ):
                print("Found diagonal down left XMAS")
                xmasCounter += 1
        # Check diagonal down right
        if (i + lengthOfOneLine * 3 + 3) < len(wordSearch):
            if (
                wordSearch[i + lengthOfOneLine + 1] == "M"
                and wordSearch[i + (lengthOfOneLine * 2) + 2] == "A"
                and wordSearch[i + (lengthOfOneLine * 3) + 3] == "S"
            ):
                print("Found diagonal down right XMAS")
                xmasCounter += 1
        # Check diagonal up left
        if (i - lengthOfOneLine * 3 - 3) >= 0:
            if (
                wordSearch[i - lengthOfOneLine - 1] == "M"
                and wordSearch[i - (lengthOfOneLine * 2) - 2] == "A"
                and wordSearch[i - (lengthOfOneLine * 3) - 3] == "S"
            ):
                print("Found diagonal up left XMAS")
                xmasCounter += 1
        # Check diagonal up right
        if (i - lengthOfOneLine * 3 + 3) >= 0:
            if (
                wordSearch[i - lengthOfOneLine + 1] == "M"
                and wordSearch[i - (lengthOfOneLine * 2) + 2] == "A"
                and wordSearch[i - (lengthOfOneLine * 3) + 3] == "S"
            ):
                print("Found diagonal up right XMAS")
                xmasCounter += 1
print("Total XMAS:", xmasCounter)
