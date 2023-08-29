use impl_derive::MyDeriveMacro;
use my_trait::MyTrait;

#[derive(MyDeriveMacro)] // 使用MyDeriveMacro宏后，就为Main实现了MyTrait
struct Main;

fn main() {
    Main::do_something(); // 因为实现了MyTrait，所以可以调用do_something函数
}
