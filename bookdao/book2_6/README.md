var、&mut var 不能出现在一个作用域内

```
fn main()
{
  // mut可变绑定
  let mut x = 5; 
  println!("x = {}", x);

  // &mut 获取可变类型的内存地址
  let ptr = &mut x; 

  // error: cannot borrow `x` as immutable because it is also borrowed as mutable
  println!("x = {}", x); 
}
```