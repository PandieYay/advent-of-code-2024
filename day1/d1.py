file = open("d1.txt", "r")

array = []
array2 = []

for line in file:
    biglist = line.split()
    array.append(int(biglist[0]))
    array2.append(int(biglist[1]))

array.sort()
array2.sort()

totalDistance = 0
simiiliarityScore = 0
for i in range(len(array)):
    totalDistance += abs(array[i] - array2[i])
    simiiliarityScore += array[i] * array2.count(array[i])

print("Total Distance:", totalDistance)
print("Similarity Score:", simiiliarityScore)
