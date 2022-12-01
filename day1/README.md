# Day1

## Most Calories Elf

It was kinda funky to get the string read in from stdin and I did not want to
do any `cat << EOF` shenanigans, so I chose to use stdin and double enter to
confirm.

```sh
$ go run ./mostCalories.go --first

*paste in the stdin, enter*
*confirm by double-entering*

*answer*
```

To do the second part, repeat the first without the flag

```sh
$ go run ./mostCalories.go

*paste in the stdin, enter*
*confirm by double-entering*

*answer*
```
