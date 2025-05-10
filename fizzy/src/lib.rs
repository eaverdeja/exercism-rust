use std::ops::Rem;

pub type MatchFn<T> = Box<dyn Fn(T) -> bool>;

pub struct Matcher<T> {
    func: MatchFn<T>,
    subs: String,
}

impl<T> Matcher<T> {
    pub fn new<F, S>(matcher: F, subs: S) -> Matcher<T>
    where
        F: Fn(T) -> bool + 'static,
        S: AsRef<str>,
    {
        Matcher {
            func: Box::new(matcher),
            subs: subs.as_ref().to_string(),
        }
    }
}

pub struct Fizzy<T>(Vec<Matcher<T>>);

impl<T> Fizzy<T>
where
    T: ToString + Copy,
{
    pub fn new() -> Self {
        Fizzy(Vec::new())
    }

    #[must_use]
    pub fn add_matcher(mut self, matcher: Matcher<T>) -> Self {
        self.0.push(matcher);
        self
    }

    pub fn apply<I>(self, iter: I) -> impl Iterator<Item = String>
    where
        I: Iterator<Item = T>,
    {
        iter.map(move |item| {
            let mut out = String::new();

            for matcher in &self.0 {
                if matcher.func.as_ref()(item) {
                    out += matcher.subs.as_str();
                }
            }

            if out.is_empty() {
                out = item.to_string();
            }

            out
        })
    }
}

pub fn fizz_buzz<T>() -> Fizzy<T>
where
    T: Copy + Default + From<u8> + PartialEq + Rem<Output = T> + 'static,
{
    let three: T = 3.into();
    let five: T = 5.into();

    Fizzy(vec![
        Matcher::new(move |n| n % three == T::default(), "fizz"),
        Matcher::new(move |n| n % five == T::default(), "buzz"),
    ])
}
