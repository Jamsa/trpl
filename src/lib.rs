pub mod client;                     // rust默认只知道src/lib.rs中的内容。通过这个文件中定义的模块去查找对应的模块名.rs文件

pub mod network;

// 如果foo模块没有子模块，则应该将其代码放在foo.rs中，并在lib.rs中添加 mod foo;
// 如果foo模块有子模块，则应该将其代码放在foo/mod.rs中，并在lib.rs中添加 mod foo;
// 如果bar是foo的子模块，则应该将其它代码放在foo/bar.rs中，并在foo/mod.rs中添加mod bar;
