
*"That's Numberlang!"*

# Numberlang is an esoteric programming language that uses numbers for sumbols (variable names, function names, etc.)

### **There are three types of expressions in Numberlang:**
- Numbers, for example `100`, `0` or `237816`.
- Tuples, for example `(1 2 3)` `()` or `((1 2 3) 4 (5 6))`
- Function calls, for example `10 < 1 2 3` (in general `[function symbol] < [argument] [argument] ...`)

### **Some common function symbols:**
- 0: Get variable `0 < [variable symbol]`
- 1: Set variable `1 < [variable symbol] [value]`
- 2: Sum `2 < [number] [number] ...` returns the sum of all the numbers

- 10: Print `10 < [value] [value] [value] ...` prints all the numbers inputted as their ascii equivalent (tuples get flattened)
- 11: Converts the value to a string (tuple of numbers) `11 < [value]`

- 30: Create function `30 < [function symbol] [arguments] [expression]`


