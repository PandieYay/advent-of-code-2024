file = open("d2.txt", "r")

safeReports = 0
for reports in file:
    levels = [int(level) for level in reports.split()]
    increasing = None
    for i in range(len(levels) - 1):
        if levels[i] - levels[i + 1] <= 3 and levels[i] - levels[i + 1] > 0:
            if increasing == False:
                break
            increasing = True
        elif levels[i] - levels[i + 1] >= -3 and levels[i] - levels[i + 1] < 0:
            if increasing == True:
                break
            increasing = False
        else:
             break
    else:
        safeReports += 1
print("Safe Reports:", safeReports)