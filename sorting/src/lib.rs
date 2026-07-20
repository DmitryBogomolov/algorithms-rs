mod insertion;
mod shell;
mod merge;
mod quick;
mod heap;

pub use insertion::sort as insertion;
pub use shell::sort as shell;
pub use merge::sort as merge;
pub use quick::sort as quick;
pub use heap::sort as heap;

#[cfg(test)]
mod test_data;
