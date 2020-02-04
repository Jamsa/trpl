# 包和crate

crate是二进制项或库。crate root是一个源文件，Rust编译器以它为起始点，构建成你的crate根模块。

包是提供一系列功能的一个或多个crate。一个包会包含一个Cargo.toml文件，描述如何构建这些crate。

包中的内容由几条规则来确定：
 - 一个包中至多只能包含一个库crate(library crate);
 - 包中可以包含任意多个二进制crate(binary crate);
 - 包中至少包含一个crate，无论是库还是二进制的。

Cargo遵循的约定：
 - src/main.rs就是一个与包同名的二进制crate的crate根。
 - Cargo知道如果包目录中包含src/lib.rs，则包带有与其同名的库crate，且src/lib.rs是crate根。
 - crate根文件将由Cargo传递给rustc来实际构建库或者二进制项目。

如果一个包同时包含了src/main.rs和src/lib.rs，则它有两个crate：一个库和一个二进制项，且名字都与包相同。
