calFile = open("./input1.txt", 'r')
firstnumber = ""
secondnumber = ""
number = 0
total = 0
wordednumber = ""
wordtonum = ""

for x in calFile:
    for y in x:
        if y.isdigit():              
            if firstnumber == "":
                firstnumber = y
            else:
                secondnumber = y
        else:
            wordednumber = wordednumber + y
            if "one" in wordednumber:
                wordednumber = ""
                wordtonum = 1
            elif "two"  in wordednumber:
                wordednumber = ""
                wordtonum = 2
            elif "three"  in wordednumber:
                wordednumber = ""
                wordtonum = 3
            elif "four" in wordednumber:
                wordednumber = ""
                wordtonum = 4
            elif "five"  in wordednumber:
                wordednumber = ""
                wordtonum = 5
            elif "six"  in wordednumber:
                wordednumber = ""
                wordtonum = 6
            elif "seven"  in wordednumber:
                wordednumber = ""
                wordtonum = 7
            elif "eight"  in wordednumber:
                wordednumber = ""
                wordtonum = 8
            elif "nine"  in wordednumber:
                wordednumber = ""
                wordtonum = 9
            else:
                wordtonum = ""
            if firstnumber == "" and wordtonum != "":
                firstnumber = wordtonum
            elif wordtonum != "":
                secondnumber = wordtonum

    wordednumber = ""    
    if secondnumber == "":
        secondnumber = firstnumber
    number = str(firstnumber) + str(secondnumber)
    firstnumber = ""
    secondnumber = ""
    total = total + int(number)


print("The total is: ", total)
calFile.close()
input("Press Enter to continue...")

input = open("./input1.txt", 'r')
total=0
numbers=["one","two","three","four","five","six","seven","eight","nine"]
for line in input:
    firstnumber=-1
    secondnumber=-1
    for i in range(len(line)):
        c=line[i]
        if c.isdigit():              
            if firstnumber==-1:
                firstnumber=int(c)
            else:
                secondnumber=int(c)
        else:
            for j in range(len(numbers)):
                if line[i:].startswith(numbers[j]):
                    if firstnumber==-1:
                        firstnumber=int(j+1)
                    else:
                        secondnumber=int(j+1)
    total+=firstnumber*10
    if secondnumber!=-1:
        total+=secondnumber
    else:
        total+=firstnumber
print("total: ",total)
input.close()
