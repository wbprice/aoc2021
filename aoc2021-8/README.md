# Entry

## Input:
be 
cgeb 
edb
cfbegad 
cbdgef 
fgaecd 
agebfd 
fdcge 
fecdb 
fabcd 

## Output:
fdgacbe cefdb cefbgd gcbe

# Parsing Input

## Freebies:
cfbegad - 8
cgeb - 4
edb - 7
be - 1

## Therefore:
- Top segment is 'd'
- Right two segments are: 'be'
- Middle segment is 'c'
- Lower right is 'e'
- Upper right is 'b'

### the 6 segment numbers (0, 6, 9)
cbdgef (_bcdefg)
- It has bceg (4), so it must be (9)

agebfd (ab_defg) 
- By process of elimination, (0)
- Middle segment is 'c'

fgaecd (a_cdefg) 
- It does not have 'be', so it must be (6)
- It has 'e', but not 'b'
- 'e' is lower right, 'b' is upper right

### the 5 segment numbers (2, 3, 5)
fdcge (__cdefg)
- It has 'e', so this must be (5)

fecdb (_bcdef_)
- It has 'edb', so this must be (3)

fabcd (abcd_f_)
- By process of elimination (2)

### The answer:
be - 1
edb - 7
cgeb - 4
fabcd - 2
fecdb - 3
fdcge - 5
fgaecd - 6
cbdgef - 9
agebfd - 0
cfbegad - 8

So `fdgacbe cefdb cefbgd gcbe` = 8394

# The algorithm

## In the input:

### Determine 8, 4, 7, 1 using length of token:
7 segments -> 8
4 segments -> 4
3 segments -> 7
2 segments -> 1

### Handle the 6 segment numbers (0, 6, 9)
1. 9 can fit a '4' in it
2. 6 can not fit '1' in it
    - it has one segment in common with '1', but not both.
    - The common segment is lower right
    - The not common segment is upper right
3. By process of elimination, the last 6 digit number is 0
    - The segment that isn't shared with 8 is the middle segment

### Handle the 5 segment numbers (2, 3, 5)
1. 3 can fit a '7' in it
2. 5 has the lower right segment 
3. By process of elimination, the last number is 2



