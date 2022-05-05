rust + pest

https://pest.rs/book/parser_api.html
https://arzg.github.io/lang/7/

## 2022-05-04 10:40:46

- @ques 应该给自己一点拓展的时间, 瞎搞瞎搞

## 2022-05-02 16:01:21

- @ques 如何匹配一个个独特的语法

  - 最好是非常容易扩展的写法
  - 最好能在一个地方写 match pattern
  - `let @name = @expr` 这种形式 那就方便多了
  - 我能想到的就是, 一个个的去匹配

- @todo 比较不熟悉的地方

- @ques map_else or_else

- @ques Result 的返回值处理

## 2022-05-01 14:38:46

- @save test

```
cargo test -- tests::parse_one_plus_two --nocapture
```

https://pest.rs/book/examples/jlang.html

- parser(token<[]>) -> lexer(Node<tree>) -> executer

- cargo test -- --show-output

- @ques 理解一些基本概念

## 2021-04-13 09:53:01

- @ques 那么多的类型该怎么处理 ?

  - rust 无法做类型继承
  - enum TokenData

- @ques if else 怎么放在 TokenData 中

- @ques BinaryExpression

  - 赋值语句

- @todo Expression ｜ Statement

- @ques 怎么运行代码

  - 怎么写 executer

- VariableDeclaration FunctionDeclaration

- lexer 每一个语句记录他的位置 会有很多的地方用到

  - location: start: position {line, column}, end: position {line, column}
  - pos: number

- @ques test 换成什么语句合适

```
test = {value | operate}
define_key = {"let"}
define = { define_key ~ name ~ "=" ~ test ~ ";" }
```

- @todo

```ts
{
  let c = 1 + 2;
}
```

## 2021-04-12 09:38:51

- @todo

  - 基本类型
    - array
  - 基本语句
    - 声明
    - 赋值
  - 加减 乘除
  - if else
  - function
  - for loop

- @todo 基本参照 rust dart ts 语法

- @ques pest 能不能跨文件

## 2021-04-02 09:54:10

- @ques 字符串"true"是怎么变成 true 的

- @ques `inner = @{ char* }` 是什么意思

  - 不容许存在 WHITESPACE ` ${ char* }` 相同且匹配 char\*

- @ques `inner = ${ char* }` 是什么意思

  - 不会忽略 WHITESPACE

- @ques 不知道 rust 本身的解析器是通过什么来写的

- @ques 字母怎么表示

- @ques 数字怎么表示
  - 十六进制...
  - 八进制

## 2021-03-31 09:31:51

感觉我这一开始设计太复杂了，我什么都不懂就搭一个这么大的架子, 将会耗费太多的精力在上面
可以设立一个个小目标；

解析器 -> 变量 -> 函数 -> pattern match -> ...
在这同时一个个的设计...

有没有一种方式 像 hook 一样的 组织代码， 非常的简单 非常的易于使用
rust 的语法如果没有生命周期那也是很简单的

dart 组合方式

脚本语言 -> ui 语言 -> 易于扩展

## 2021-03-31 09:56:18

- component base language
  - interface + implement
  - 集合提供外接接口

## save

提供语法原理, 方便扩展新的语法

cha 是语法类似 rust 的脚本语言, 用来实现一些 zsy 常用脚本的任务

类型推断

基本语法很简单， 如果要用高级功能， 那么就需要满足特定的条件。。。

很容易表达人的想法 -> 最好是思维型
(将特例分离)

interface 继承只能用来整理数据

await

很容易用来做 ui

- 语法解析器生成 Node, Node 之间存在对应的关系...
  - node 之间存在各种关系...

最好能直接调用 rust 代码

像 python 一样的语法, 易于理解...

能不能参考中国古代数学的逻辑, 来组织语法
<九章数学>

有没有办法将 rust 里面 box option ... 去掉... - 更干净 清晰

快速的监听属性的改变...

## 2021-03-31 09:30:33

https://medium.com/clevyio/using-rust-and-nom-to-create-an-open-source-programming-language-for-chatbots-12fe67582af5

https://blog.cloudflare.com/building-fast-interpreters-in-rust/

rust + pest | nom

easy use + speed

best lang for script | ui render ｜ speed

## 2019-10-10 09:29:46

- @todo print() 如何处理 这是一个方法的调用...

  - 函数调用 函数 + 参数...
  - 如何匹配是一个函数... `函数名()`
  - 能不能支持匹配模式 多个进行 item 就行合并 -> 组合匹配
  - 会有很多的地方放需要这种匹配, 没有这种方式, 很多东西很难做...
  - 其实这些语法都是固定的模式的...
  - 那个语法匹配高亮是怎么实现的...

- @note rust 的 match 写起来 还是比较麻烦...

  - 有什么方法来匹配类型吗...

- github 简历

## 2019-08-29 14:04:00

[Node { val: Scope(Scope { children: [Node { val: Statement(Statement { children: [Node { val: String("hello world"), position: NodePosition { uid: "", pos: Pos { x: 26, y: 1 } } }, Node { val: Variable(Variable { name: "a", variable_type: None }), position: NodePosition { uid: "", pos: Pos { x: 10, y: 1 } } }, Node { val: Define(Define { right: Some(Node { val: Assign(Assign { left: None, right: None }), position: NodePosition { uid: "", pos: Pos { x: 12, y: 1 } } }), is_mut: false }), position: NodePosition { uid: "", pos: Pos { x: 8, y: 1 } } }] }), position: NodePosition { uid: "", pos: Pos { x: 0, y: 0 } } }, Node { val: Statement(Statement { children: [Node { val: Variable(Variable { name: "a", variable_type: None }), position: NodePosition { uid:
"", pos: Pos { x: 12, y: 2 } } }, Node { val: Variable(Variable { name: "print", variable_type: None }), position: NodePosition { uid: "", pos: Pos { x: 10, y: 2 } } }] }), position: NodePosition { uid: "", pos: Pos { x: 0, y: 0 } } }] }), position: NodePosition { uid: "", pos: Pos { x: 1, y: 0 } } }]

- @ques `RUST_BACKTRACE=1`

  - `RUST_BACKTRACE=1 cargo run`

* @ques rust debug

define_set_right(define, var_node);

- @todo asiggn hello world 没有值...

- @ques rust for in remove 会不会漏掉...

- @ques rust RUST_BACKTRACE=1

- @ques `let a = 1 + 2 + 3;` 怎么处理 `1 + 2 + 3` 怎么合并起来...

* @todo Identifier("hello world") 变成 string

* @ques assign 如何设置 right...

  - 我还不知道右边有什么东西...
  - 如果右边是一个表达式怎么处理...
  - 一直到 `;` 结束...
  - 这我怎么处理???
  - 在一行结束之后去处理这个问题..., `=` 前面的 后面的 怎么处理...

* @todo `lexer\mod.rs` 有重复代码

- @todo 还有许多值没有做处理...

  - let identify

- @todo 双引号里面的字符串应该能匹配..
- @todo 连续的 space 要不要合并在一起

* @ques 语法关系,

  - scope -> statement -> node
  - 我怎么找到特定的 node
  - 可能 node 在运行的时候才有关系...
  - scope -> statement -> expression -> expression 就像一篇文章 -> 段落 -> 句子
  - scope -> node 就像里面的人物关系表 -> 只有在实际运行是才有关系...
  - ***
  - node 是跨越行的...

* expression 包裹 expression 一直到行结束
  - 怎么将东西组成一个 express
  - `let a = 1;`, (1 + (1 + 1)) 这如何组织关系...
  - 加本身组成一个 expression
  - 现在我可以不管...

## 2019-08-20 07:50:13

- macro
- @todo macro

- @note 方便的方式组织 struct...

- @ques preview_next None hello world 没有放在一起...

- @ques 数据有问题 多个 space --> EOL

## 2019-08-28 17:33:07

- @ques expect lifeTime 如何解决...

  - n
  - 现在没有办法解决...
  - 给 expect 添加这个参数....
  - 这样所有的都只在函数中有效...
  - expect_fn 的生命周期最多和 ExpectList 一样, 不可能超过 scope_tree, 这我怎么处理
  - 让 expect_fn 的生命周期小于这个函数...
  - 能不能 expect_fn 是一个引用, 保存在 Parser 里面
  - 这样生命周期的问题就解决了

- @Keyword

  - 能不能不直接使用变量
  - 返回一个 key, 通过这个 key 能找到我引用的变量

- @ques rust 有没有观察者 设计模式

- @ques 怎么定位到一个特定的 scope 中的 node...,

  - 应该有 id 的
  - id 的下面怎么找...
  - 可以在创建的时候不放在树中
  - 等到完成之后才放进去...

- @ques let a = 1; a 并不知道自己后面有没有 = 1;

- @ques 代码像一篇文章, 语句是他的单位...
  - 文件 块 句子 词组 字
  - fun scope, match scope if scope
  - 我怎么定位到 定义呢

## 2019-08-06 08:01:14

- @ques rust 能不能获取一个变量的 lifeTime
  - 或者然 struct 函数的参数的 lifetime 为 self

* @ques `let Expect{expect_fn,..} = self;expect_fn()`

  - expect_fn 不知道自己内部使用多少对象, 不知道他的 size

* @ques 两个 enum 怎么 match 相等

* @ques rust 如何传递函数给 struct

```
match (self_val_type, self_expect_type) {
    (val_type, expect_type) => true,
    _ => false,
}
```

- @ques rust 能用观察者吗

- @ques rust 一行就相当于一个语句

  - 检测换行符...

- @todo ExpectList<T: PartialEq, U>

## 2019-08-02 07:45:55

- @ques rust 怎么将 vec 中 item 拿出来放到其他地方...
  - 我就是要将 item 拿出来

```rs
for i in 0..node_list.len() {
    let item = node_list[i];
    scope.add_child(item)
}
```

## 2019-07-30 07:39:51

- @quest scope 的子集到底是什么类型...

  - 必须是一个, 也就是是
  - parse 并不需要计算, type: ..., val: `a`
  - assign left right 怎么解决...

- @ques NodeVal 可能是多种类型如何处理...

  - 最理想的情形是...
  - 如果用继承 我直接用基类就可以解决这个问题...

- @ques 用 enum 包裹类型显然不应该是 enum 的用法

- @ques 不同的类型这个图形绘制应该类似啊

  - circle... 怎么组织的

- 这些不同的类型不得不通过 enum 包裹起来, 不然他们都不是一个类型
  - 无法在一起使用

## expect

- @todo 这个格式是什么样子的

  - expectBefore expectAfter
  - expect 只要记录类型就可以了

- @todo expect 成功之后要执行函数啊

  - 匿名函数
  - 需要将满足条件的值传过来

- 强制 expect, 非强制 expect
- let a = 1;

  - `let a` 是一个表达式返回变量 a
  - `=` expect 前面是一个变量 后面是一个值

- @ques scope 要支持 findItemByName,
  - 获取变量函数...

## 2019-07-27 05:24:56

- @note 我现在还不知道我后面 execute 需要什么信息

  - 现在完成我自己的功能...

- scope '{' 能不能 expect 后面出现 '}' 这样就结束了
  - 如果很容易就能实现这个功能那就方便了
  - 如何去写 expect?

* @ques 如何把三个不同的类型合并成一个...

  - 这个问题一定要解决...

* @ques Node --> DefineNode; DefineNode --> Node

  - defined variable
  - 能不能将节点的信息设置为 DefineNode
  - val: DefineNode

* NodeVal 还是不能将其中的值抽离出来

  - 几个类型必须要合并成一个 trait

* @ques 能不能和 rust 保持同样的逻辑, 而不用使用 unwrap 这种形式...

* @ques errorScope

- @ques 一开始将所有的东西启动起来真的很麻烦

  - 而且如果以后要加什么东西 都必须每一个部分都需要添加
  - 能不能将这变得 plugin ...
  - 很方便的定义语法...

- @ques 有时候我需要在子节点去寻找父节点定义的 var fun

  - 这怎么处理
  - 我怎么才能找到这些值...
  - parent...
    - 父亲下面的值

- @ques 我 DefineNode 怎么组织自己的 left(node)

- @ques 其实并不是所有的节点都有 children 的

* @ques trait 能不能支持属性...

  - 公共的 属性 + 行为...
  - 能不能像 interface 一样呢

* 如何将链式 变成 父子树的形式

* @ques 赋值如何去组织
  - left right
  - 如何将链式 变成 父子树的形式
  - left 已经创建 right 还未创建...
  - 创建一个 scope 将左右各包起来

- @note 在 parse 阶段我只要将各种值的关系列好就可以了

  - 比方说一个变量的创建以及他的各种引用
  - 等到真正运行的时候, 才去真正的赋值...

- @ques 事实上每一个 node 类型他的数据结构都不一样, 无法用一个 stuct 表达所有的类型

  - 用什么呢? trait 保持同样的语法
  - node type 也就不需要了(最好还是要, 要有一个地方整理所有的结构)
  - rust 怎么...

- @ques 前面定义的变量, 后面使用 我怎么使用呢

  - 怎么找到, 怎么使用...

- @ques 所有的 enum 放在一起好, 还是放在他们使用的地方好呢???

- @ques NodeTrait 我应该如何去定义...

- @ques 语句的类型

- @ques match data ...

  - 这下面一个个的看着不清晰...

- @todo let 语句

* @ques 每一种的 nodeType 的格式其实都是不一样的

  - 有没有统一的格式...

* @ques let 语句如何去组织...

  - `let a = 1;`

* @ques struct 能不能使用空对象
  left 能不能可选 + 创建时候为空

```rs
Node {
    left: Box<Option>,
    ...
}
```

- @ques 怎么创建一个空的

* @ques Parser::parse 里面的 match 放在其他的地方会更有条理

- @ques 怎么把 node 放到其他的 node 下面...

  - scope_tree 结束之后才会放到 scope_tree 中 上一个 scope, 将自己去掉...
  - scope_tree 为空 才会放到 node_list 中

- @ques scope 的类型

  - function if ..

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
