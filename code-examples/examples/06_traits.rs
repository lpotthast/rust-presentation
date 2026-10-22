trait Notifier {
    fn recipient(&self) -> &str;

    // default implementation
    fn notify(&self, message: &str) {
        println!("To {}: {message}", self.recipient());
    }
}

struct Email {
    address: String,
}

impl Notifier for Email {
    fn recipient(&self) -> &str {
        &self.address
    }
}

struct Console;

impl Notifier for Console {
    fn recipient(&self) -> &str {
        "stdout"
    }

    // override the default implementation
    fn notify(&self, message: &str) {
        println!("[console] {message}");
    }
}

// the slide shows both variants under the same name `alert`
mod static_dispatch {
    use super::Notifier;

    pub fn alert(notifier: &impl Notifier) {
        notifier.notify("disk almost full");
    }

    // same thing in generic syntax
    pub fn alert_generic<T: Notifier>(notifier: &T) {
        notifier.notify("disk almost full");
    }
}

mod dynamic_dispatch {
    use super::Notifier;

    pub fn alert(notifier: &dyn Notifier) {
        notifier.notify("disk almost full");
    }
}

fn main() {
    let email = Email { address: String::from("ops@shop.de") };

    static_dispatch::alert(&Console);
    static_dispatch::alert(&email);
    static_dispatch::alert_generic(&email);

    dynamic_dispatch::alert(&Console);
    dynamic_dispatch::alert(&email);

    // different types in one list
    let notifiers: Vec<&dyn Notifier> = vec![&Console, &email];
    for notifier in notifiers {
        dynamic_dispatch::alert(notifier);
    }
}
