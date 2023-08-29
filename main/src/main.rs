use impl_attr_macro::func_info;
use impl_derive::MyDeriveMacro;
use impl_fn_macro::make_answer;
use my_trait::MyTrait;

#[derive(MyDeriveMacro)] // 使用MyDeriveMacro宏后，就为Main实现了MyTrait
struct Main;

make_answer!(); // 调用函数宏生成answer函数

#[func_info] // 为foo函数添加属性
fn foo() {
    println!("hello foo!");
}

fn main() {
    Main::do_something(); // 因为实现了MyTrait，所以可以调用do_something函数

    println!("{}", answer());

    foo(); // 会自动加上属性宏中的内容
}
