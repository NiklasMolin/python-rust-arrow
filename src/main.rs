use structopt::StructOpt;
#[link(name = "c_test", kind = "static")]
extern "C" {
    pub fn no_arg() -> *mut u64;
}
extern "C" {
    pub fn no_arg_struct() -> TestStruct;
}
extern "C" {
    pub fn no_arg_struct_p() -> *const TestStruct;
}

#[repr(C)]
#[derive(Debug)]
pub struct TestStruct {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Debug, StructOpt)]
#[structopt(about = "I am a program and I work, just pass `-h`")]
enum PartToRun {
    #[structopt(about = "A simple example that gets an int pointer from a c function")]
    get_int,
    #[structopt(about = "Get a simple three int struct from c and let rust handle the conversion")]
    get_struct,
    #[structopt(
        about = "Get the memory address of a simple three int struct an read it up in a struct"
    )]
    get_struct_pointer,
}

fn main() {
    let opt = PartToRun::from_args();
    match opt {
        PartToRun::get_int => test_get_simple_c_example(),
        PartToRun::get_struct => test_get_simple_c_struct(),
        PartToRun::get_struct_pointer => test_get_struct_pointer(),
    }
}

fn test_get_simple_c_example() {
    //we need to make this unsafe since we are dereferncing a raw pointer
    unsafe {
        let d = no_arg();
        println!(
            "this is the value we recived from the c-function (it should be 4): {}",
            (*d) as i32
        );
    }
}
const PRINT_DESCRIPTION: &str =
    "Getting a simple struct pointer with three ints x,y,z from c, the values should be 666,7727,2";
fn test_get_simple_c_struct() {
    //we need to make this unsafe since we are dereferncing a raw pointer
    unsafe {
        println!("{}", PRINT_DESCRIPTION);
        let res = no_arg_struct();
        println!("The result: {:?} ", res);
    }
}
fn test_get_struct_pointer() {
    //we need to make this unsafe since we are dereferncing a raw pointer
    unsafe {
        println!("{}", PRINT_DESCRIPTION);
        let pointer = no_arg_struct_p();
        //We have can't really do two uses of the pointer, it will move the value
        //println!("The address of the c struct is: {:?}", pointer);
        let res = pointer.read();
        println!("The result: {:?} ", res);
    }
}
