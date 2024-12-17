file = open("d2.txt", "r")

safeReports = 0
for reports in file:
    levels = [int(level) for level in reports.split()]

    for x in range(len(levels) + 1):
        tempLevels = levels.copy()
        # Checks all possible combinations by removing one level from the list
        if (x > 0):
            tempLevels.pop(x - 1)
        increasing = None
        for i in range(len(tempLevels) - 1):
            if tempLevels[i] - tempLevels[i + 1] <= 3 and tempLevels[i] - tempLevels[i + 1] > 0:
                if increasing == False:
                    break
                increasing = True
            elif tempLevels[i] - tempLevels[i + 1] >= -3 and tempLevels[i] - tempLevels[i + 1] < 0:
                if increasing == True:
                    break
                increasing = False
            else:
                break
        else:
            safeReports += 1
            break
print("Safe Reports:", safeReports)