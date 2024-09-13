pub struct FormatWith<'a> {
    s: Option<&'a str>,
    d: Option<&'a str>,
}

impl<'a> FormatWith<'a> {
    pub fn new(s: &'a str, d: &'a str) -> FormatWith<'a> {
        FormatWith {
            s: Some(s),
            d: Some(d),
        }
    }

    pub fn d(d: &str) -> FormatWith {
        FormatWith {
            d: Some(d),
            s: None,
        }
    }

    pub fn s(s: &str) -> FormatWith {
        FormatWith {
            s: Some(s),
            d: None,
        }
    }
}

pub fn fmt_string(formatter_string: String, args: FormatWith) -> String {
    let formatter_string = if let Some(s) = args.s {
        formatter_string.replace("%s", s)
    } else {
        formatter_string
    };
    let formatter_string = if let Some(d) = args.d {
        formatter_string.replace("%d", d)
    } else {
        formatter_string
    };
    formatter_string
}
