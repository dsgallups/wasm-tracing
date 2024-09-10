struct StringRecorder {
    display: String,
    is_following_args: bool,
    show_fields: bool,
    fields: HashMap<String, String>,
}
impl StringRecorder {
    fn new(show_fields: bool) -> Self {
        StringRecorder {
            show_fields,
            ..Default::default()
        }
    }
}

impl Visit for StringRecorder {
    fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
        if field.name() == "message" {
            if !self.display.is_empty() {
                self.display = format!("{:?}\n{}", value, self.display)
            } else {
                self.display = format!("{:?}", value)
            }
        } else if self.show_fields {
            if self.is_following_args {
                // following args
                writeln!(self.display).unwrap();
            } else {
                // first arg
                write!(self.display, " ").unwrap();
                self.is_following_args = true;
            }
            write!(self.display, "{} = {:?};", field.name(), value).unwrap();
            self.fields
                .insert(field.name().to_owned(), format!("{:?}", value));
        }
    }
}

impl core::fmt::Display for StringRecorder {
    fn fmt(&self, mut f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if !self.display.is_empty() {
            write!(&mut f, " {}", self.display)
        } else {
            Ok(())
        }
    }
}

impl core::default::Default for StringRecorder {
    fn default() -> Self {
        Self {
            display: String::new(),
            is_following_args: false,
            show_fields: true,
            fields: HashMap::new(),
        }
    }
}
