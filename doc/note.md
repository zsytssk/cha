- @ques 语句的类型
  - 运算 left + right ...
  - 我怎么通过运算符 拿到左边的值呢
  - 这是一个不断变化的过程, 从左往右
  - 真正的运算确实从右往左...
  ***
  - `let a = 1;`
  - let 判断这是一个值定义
  - a -> let 定义的值
  - = -> 创建运算符 将已经创建的 `let a` 放在左边 去找右边...
  - 1 -> 一个值
  - ; -> 结束

* @ques match next ... 这个能不能做成公用的

* @ques 计算语句 Node left right;

* @ques rust match if statement
  `slice if Some(x) == Keyword::from_str(slice) =>`

* @ques lex 函数可以抽离出来...

* @ques rust print char \t

* @ques rust mod relative path

* @ques 如何将 list 变成 node

* @ques rust 怎么匹配 换行符... 空格...

  - 能不能匹配多个字符
  - match a = ' ' || '\n' || 'r' || 'n'

* @ques rust 怎么匹配 换行符...

* @ques rust 怎么 判断是 keyWord 还是 字符

* @ques rust auto complete mode

* @ques rust 字符串 as type

  - ts`type A = "a" | "b"`

* @ques rust lifetime 能不能设置为 self...

* @ques rust 能不能想 js 一样 export 另一个文件的内容...
  `pub use token_data::TokenData;`

* @ques 怎么把字符合并在一起

  - 正则匹配字符
  - \w
  - 标点 | 字符

* @ques 乱起八糟的小类型放在集合文件里面挺好的

  - 文件叫什么名字??/

* @ques 怎么把 string 转换为 enum

- 下面这个报错怎么处理

```rs
pub fn parse(source: String) {
    let chars = (&source as &str).chars().peekable();
    let parser = Parser { source, chars };
}
```

- 开始一定要精简

- 字符的类型

  - 字符块 :> chars_block

    - 关键字
    - 变量
    - 值

  - 标点 :> punc

- @ques `let a = 1;` a 和 1 要不要区分

  - name value

- @ques 怎么一个个的解析

* 语法解析器生成 Node, Node 之间存在对应的关系...
  - node 之间存在各种关系...

- @ques 类型 常量 + 标点 这些常量用什么英文表示

- @ques dbg! 是做什么的

- @ques 假设所有字符都是通过 空格 分割的

  - 这个玩意其实可以多线程去做的...

- @ques 怎么一个个的遍历字符串...

- @ques 怎么把一个链式的结构转化为父子树的结构

  - 如何快速的定位到其中的一个节点...

- @ques parseErr 常量
  - 语法错误...

* @ques 能不能将项目变成两部分 parser + runner

  - swc parse

* @thk 如果功能能方便的扩展, 就像 webpack plugin 一样 那不就好了

* @ques 怎么处理继承的
  - 只要能关联父子类之间的关系 就可以了...

- @ques get_field_slice 怎么处理...

* get_field
