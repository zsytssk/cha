rust 对应的脚本语言:> 快速开发 + 性能 + 安全...

## 参考文档

- @imp https://arzg.github.io/lang/3/
  https://github.com/arzg/eldiro

https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html

## 如何做 benchmark

## 语句的类型

- Statement ? Expression

- 赋值语句

## 赋值如何去组织

- left right
- left 已经创建, right 还未创建...
- 创建一个 scope 将左右各包起来

  - 分号就相当于结束
  - 这和 scope 不一样, 这是一个 statement

## 能不能将这变得 plugin

- 可任意添加语法...
