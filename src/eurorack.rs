pub mod er301;
pub mod txo;

#[derive(Debug)]
pub struct Command {
   pub command_number: u8,
   pub args: &'static[Arg],
}

#[derive(Debug)]
pub struct Arg {
    pub name: &'static str,
    pub argtype: Bufsize,
}
#[derive(Debug)]
pub enum Bufsize {
    U8, 
    S16, 
    S16V,
}
