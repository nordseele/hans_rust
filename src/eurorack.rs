pub mod er301;
pub mod txo;

#[non_exhaustive]
pub struct Cmd;

#[derive(Debug)]
pub struct Command {
   pub command_number: u8,
   pub args: Vec<Arg>
}

#[derive(Debug)]
pub struct Arg {
    name: String,
    argtype: Bufsize
}
#[derive(Debug)]
pub enum Bufsize {
    U8, 
    S16, 
    S16V,
}
