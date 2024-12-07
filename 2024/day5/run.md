# Sanitize the input
For the input:
```
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
```

Need to follow this steps:
- First replace the '|' with ' '
```regex
%s/|/  
```
- Put the number of ordering rules in the first line
- Put the number of updates above the first update

For example the input must be:
```
21
47 53
97 13
97 61
97 47
75 29
61 13
75 53
29 13
97 29
53 29
61 53
97 53
61 29
47 13
75 47
97 75
47 61
75 61
47 29
75 13
53 13
6
75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
```

# Run the code
```bash
g++ -o a day5.cpp && ./a < input.in
```
