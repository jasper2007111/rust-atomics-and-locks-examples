# 《Rust并发编程实战》示例集

《Rust并发编程实战》一书的示例集，此书英文原版[《Rust atomics and locks》](https://www.oreilly.com/library/view/rust-atomics-and/9781098119430/?_gl=1*3mmke0*_ga*NTE1NDAzMDYuMTczNTIxNjI3NA..*_ga_092EL089CH*MTczNTIxOTI5MS4yLjEuMTczNTIxOTMwNC40Ny4wLjA.)。纯手工敲出来的，非官方示例，书中部分代码只显示局部，有些示例属于我自己的一些补全，可能也不对仅供参考，希望喜欢Rust一块学习讨论。
《Rust并发编程实战》这本书中文版出来后，我第一时间购买了。Rust学习的书籍里，并发编程大都只是简单的一章，而能成书的目前似乎只有这本书。Rust的一些特性如果不结合并发编程的场景总觉得理解不够彻底，初读这本书发现对原来的认知新的理解。

# 运行
我使用了工作空间（workspace）的管理方式，以第一章节的1-1项目为例：

```sh
cd 1-1 # 切换目录，直接在目录下执行以下的命令
cargo run -p example-1 # 执行具体的例子
cargo new example-2 --vcs none # 创建新的例子，--vcs none不创建git
```
