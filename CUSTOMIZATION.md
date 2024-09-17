# Customization

There isn't a lot of customization in `hbd`.

Tho, you can still create a `~/.config/hbd/config.ron` file to add some customization to `hbd`.

Here is the complete list of things that you can customize with it:

```rs
ToolConfig(
    format: ConfigFormat(
        /// Person's birthdays name when using get with `--separator`
        /// %s: The name of the person
        separator_happy_birthday: Some("%s"),
        /// Person's birthdays name with age when using get with `--separator`
        /// %s: The name of the person
        /// %d: The age of the person
        separator_happy_birthday_age: Some("%s (%dy)"),
        /// Person's birthdays name when using get
        /// %s: The name of the person
        happy_birthday: Some("\x1B[1;33mToday is the birthday of %s !!!\x1B[0m"),
        /// Person's birthdays name with age when using get
        /// %s: The name of the person
        /// %d: The age of the person
        happy_birthday_age: Some("\x1B[1;33mToday is the birthday of %s (%d years old) !!!\x1B[0m"),
        /// Person's birthdays name when using list
        /// %s: The name of the person
        /// %d: `will_be`
        birthday_of: Some("Birthday of %s !! %d"),
        /// Person's birthdays age when using list
        /// %d: The age of the person
        will_be: Some("(Will be %d years old)"),
        /// In how many days is it the persons birthday
        /// %d: The number of days
        /// %s: if days != 1 { "s" } else { "" }
        in_x_days: Some("\x1B[1mIn %d day%s:\x1B[0m"),
    )
)
```

_Note: you can also see it [here](./config.ron)_

As you may have seen from the example, the formatting supports ANSI escape, so you can color them as you wish!
