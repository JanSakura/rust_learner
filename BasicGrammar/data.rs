/*variable,变量
in Rust, variable default immutable!
 */
fn variable_test()
{
    /*
    use 'let' to declare variable number;
    and Rust can derive type, such as i32
     */
    let var1 = 1;
    // Specify the type explicitly,显式声明变量类型
    let var2 : i64 = 2;
    /*
    The variable naming convention is Snake Case naming,
    // var_name_type;
    while enumeration and structure naming are Pascal naming.
    // structNameType;

    If the variable is not used, add an underline before the name
    // _var_not_use;
     */

    // Use as for variable casting,使用as进行强制类型转换
    let var3 = var1 as i64;
    // use 'let mut' to make variable mutable
    let mut var_mut_1: i32 = 4;
    var_mut_1 = 5;
    println!("mutable variable number: {var_mut_1}");

    /*
    Shadowing Variable
    This is not reassigning values to variables!
    这只是隐藏了变量，所以可以改变类型，可变性,值
    This just hides the variables,
    so you can change the type, mutability, and value
     */
    let mut var1 = "1";
    let var1 = 0.222;
}