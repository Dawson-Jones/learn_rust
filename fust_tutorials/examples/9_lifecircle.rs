/// 1. 每一个**引用参数**都会获得独自的生命周期
/// 2. 函数参数中只有一个引用类型，所有返回值的生命周期都等于该输入生命周期
///     很好理解, 因为在函数内产生的对象会被销毁, 进而出现悬垂引用
/// 3. 若存在多个输入生命周期，且其中一个是 &self 或 &mut self，则 &self 的生命周期被赋给所有的输出生命周期


struct Test<'a> {
    part: &'a str,
}

impl<'a: 'b, 'b> Test<'a> {
    // 默认第三规则, 返回值的生命周期等于 self 的生命周期
    fn multi_args(&self, announcement: &str) -> &str {
        println!("Attention: {}", announcement);
        self.part
    }

    // 强制要求返回值的生命周期和 announcement 一样
    fn multi_args2(&self, announcement: &'b str) -> &'b str {
        println!("Attention: {}", announcement);
        // 但是这里返回了 self 的生命周期
        // 隐含的意思是 self 的生命周期要比 'b 长, 所以应该写成 ‘a: 'b
        self.part
    }
}

fn main() {
    let hw = String::from("hello, world");
    let t = Test { part:  &hw };
    let args = String::from("hello, rust");
    let refenrence = t.multi_args(&args);
    println!("{}", refenrence);
    let refenrence = t.multi_args2(&args);
    println!("{}", refenrence);

    // 一个复杂的生命周期的例子
    let mut list = List {
        // hello 有 ‘static 的生命周期
        manager: Manager { text: "hello" }
    };

    list.get_interface().noop();
    println!("Interface should be dropped here and the borrow released");

    use_list(&list);
}

fn use_list(list: &List) {
    println!("{}", list.manager.text);
}


struct Interface<'b, 'a: 'b> {  // 'a 的生命周期比 'b 长
    // manager 用 'b 的生命周期
    manager: &'b mut Manager<'a>,
}

struct Manager<'a> {
    // text 有 ’a 的生命周期, 被引用的对象至少要拥有 ‘a 的生命周期
    text: &'a str,
}

struct List<'a> {
    manager: Manager<'a>,
}

impl<'b, 'a: 'b> Interface<'b, 'a> {
    fn noop(self) {
        println!("interface consumed");
    }
}

impl<'a> List<'a> {
    fn get_interface<'b>(&'b mut self) -> Interface<'b, 'a> {
        // 返回的 Interface 生命周期要比 List 短
        Interface {
            manager: &mut self.manager,
        }
    }
}

