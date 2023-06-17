### myCPU
myCPU is a custom-designed cpu emulated in Rust. This repository contains a basic assembler, runner, and cpu library.

### Assembler
```
assembler -p [FilePath] -o [OutputPath]
```

**Assembly Language Example:**
```arm
data:
  u8 end 5
  u8 count 0
  u8 increment 1

main:
  loada count
  loadb end
  jneq loop
  exit 0

loop:
  loada count
  loadb increment
  add
  storec count
  jump main
```

**Supported operations:**
```arm
#
# A [label] is either a function label ("main") or a variable label:
# e.g. ("u8 [label] [u8]" or "u8 myvar 0" - myvar being the label)
#
# A [code] is the exit code (u8).
#
nop
exit        [code]
add
subtract
multiply
loada       [label]
loadb       [label]
loadc       [label]
storea      [label]
storeb      [label]
storec      [label]
jump        [label]
jeq         [label]
jneq        [label]

#
# Data can only be stored in the data: label. 
# To define a label with a value aka a variable, use one of the following: 
#
u8 [label] [u8]

#
# Every program must contain a data and a main label, and an exit operation:
#
data:
  u8 mydata 0

main:
  exit 0

#
# Comments are supported with "//":
# This assembly adds two numbers
#
data:
  u8 first 10
  u8 second 5
  u8 result 0
main:
  // load value into reg a
  loada first
  // load value into reg b
  loadb second
  // add reg a & b
  add
  // store the result at the result label
  storec result
  // exit with code 0
  exit 0
 ```

### Runner
```
runner -p [FilePath] -r [optional: read address]
```