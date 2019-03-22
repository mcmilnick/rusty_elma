pub mod process;
pub mod channel;
pub mod manager;
pub mod stopwatch;
pub mod sender_proc;
pub mod reciever_proc;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
