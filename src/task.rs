use std::cmp::Ordering;
use std::fmt;

#[derive(Clone)]
pub struct Task {
    pub name: String,
    pub t: u32,
    pub c: u32,
    pub p: u32,
    pub u: f64,
    pub r: u32,
}

impl std::fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Task {}\nt: {t}\nc: {c}\np: {p}\nu: {u}\nr: {r}",
            self.name,
            t = self.t,
            c = self.c,
            p = self.p,
            u = self.u,
            r = self.r
        )
    }
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
impl Eq for Task {}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        self.p.cmp(&other.p)
    }
}
