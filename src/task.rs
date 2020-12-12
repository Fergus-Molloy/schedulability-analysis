use std::cmp::Ordering;
use std::fmt;

#[derive(Clone)]
#[allow(non_snake_case)]
pub struct Task {
    pub name: String,
    pub T: u32,
    pub D: u32,
    pub C: u32,
    pub P: u32,
    pub U: f64,
    pub R: u32,
}

impl std::fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Task {}\nT: {t}\nD: {d}\nC: {c}\nP: {p}\nU: {u}\nR: {r}",
            self.name,
            t = self.T,
            d = self.D,
            c = self.C,
            p = self.P,
            u = self.U,
            r = self.R
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
        self.P.cmp(&other.P)
    }
}
