<div align="center">

# Commands

</div>



## add

`add` - as it name applies - will "add" a person's name with it's birthday to our storage.

This is the command that you will use to remember birthdays

### Example

```sh
$ hbd add John 1984-12-20
```

### Help

```sh
$ hbd add --help
Usage: hbd add <NAME> <BIRTHDAY>

Arguments:
  <NAME>      The name of the person that you want to add
  <BIRTHDAY>  The birthday (YYYY-MM-DD, if there is a year, else MM-DD)

Options:
  -h, --help  Print help
```



## get

`get` - as it name applies - will "get" the name of the people that have their birthdays being today.

### Example

```sh
$ date
Tue Dec 20 00:00:00 AM UTC 2024
$ hbd add John 1984-12-20
$ hbd get
Today is the birthday of John (40 years old) !!!
```

### Help

```sh
$ hbd get --help
Usage: hbd get [OPTIONS]

Options:
  -s, --separator <SEPARATOR>  Use a separator between names
  -h, --help                   Print help
```



## list

`list` - as it name applies - will "list" the birthdays of all people

### Example

```sh
$ date
Tue Dec 19 00:00:00 AM UTC 2024
$ hbd add John 1984-12-20
$ hbd add Bob 1984-12-21
$ hbd list
In 1 day:
Birthday of John !!

In 2 day:
Birthday of Bob !!
```
