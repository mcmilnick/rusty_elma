/// # Remarks
/// The lib collects the following modules as they are used for elma. They are compiled into a library file.
/// This elma library can then be included elsewhere such as it is in the test /test directory.

pub mod process;
pub mod channel;
pub mod manager;
pub mod stopwatch;
pub mod sender_proc;
pub mod reciever_proc;
pub mod basic_process;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
