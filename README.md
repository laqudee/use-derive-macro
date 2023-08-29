# use-derive-macro

- 声明宏
    - macro_rules!定义
    
    ```rust
    // 定义一个声明宏my_vec
    macro_rules! my_vec {
    	( $( $x:expr ), * ) => {
    		{
    			let mut temp_vec = Vec:new();
    			$(
    				temp_vec.push($x);
    			)*
    			temp_vec
    		}
    	};
    }
    
    fn main() {
        let v = my_vec![1, 2, 3];  // 使用声明宏 my_vec！
        println!("v: = {:?}", v);
    }
    ```
    
- 过程宏
    - 接收Rust代码作为输入，在这些代码上进行操作，然后产生另一些代码作为输出
    - 声明宏则是匹配对应模式然后以另一部分代码替换当前代码
- 过程宏分类：
    - **自定义derive宏**，在结构体、enum等类型上通过#[derive()]指定derive属性，添加代码；
    - **类属性宏**，定义可用于任意项的自定义属性
    - **类函数宏**，看起来像函数但是作用于作为参数传递的Token
- 使用自定义derive宏涉及到三部分代码：
    - 要生成的目标代码
    - 为类型生成目标代码的代码
    - 在类型上标注的#[derive(xx属性)]
 
- [notion 宏笔记](https://www.notion.so/Learn-Rust-Easy-chapter3-3-22-3-23-f44427273f6c4b9fa16aaef3f4223f6f?pvs=4)
