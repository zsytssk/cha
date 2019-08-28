- @q

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
