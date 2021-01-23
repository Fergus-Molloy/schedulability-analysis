use std::fmt;
/// Struct for modelling a resource assumes that the access time is static
/// (is not different for different tasks)
#[derive(Clone)]
#[allow(non_snake_case)]
pub struct Resource {
    pub name: String,
    pub P: Option<u32>, //priority for use with Priority Ceiling Protocols
    pub A: Option<u32>, // access time assumed 0 if none
}

impl fmt::Debug for Resource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

/// Struct for critical section of a task
/// Must have a resource and an integer time that the resource is accessed for
#[derive(Clone)]
pub struct CriticalSection(Resource, u32);

impl fmt::Debug for CriticalSection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Resource {:?} for {} time units", self.0, self.1)
    }
}
