// build.rs —— 第4层：任意程序层
// 它就是一个普通的 Rust 可执行程序，在编译主 crate 之前运行。
// 没有任何 const fn 的限制：可以用 Vec、HashMap、String、循环、递归……

use std::collections::HashMap;
use std::env;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=data.txt");

    // ===== 这一步 const fn 永远做不到：堆分配 + HashMap =====
    let mut freq: HashMap<String, usize> = HashMap::new();
    for line in std::fs::read_to_string("data.txt").unwrap().lines() {
        *freq.entry(line.to_string()).or_insert(0) += 1;
    }

    // ===== 这一步 const fn 也做不到：读取外部环境 =====
    let build_time = env::var("MY_TAG").unwrap_or_else(|_| "untagged".to_string());

    // 把计算结果"降级"成源码，写回给编译器
    let out = Path::new(&env::var("OUT_DIR").unwrap()).join("generated.rs");
    let mut code = String::new();
    code.push_str(&format!("pub const BUILD_TAG: &str = \"{build_time}\";\n"));
    code.push_str("pub const WORDS: &[(&str, usize)] = &[\n");
    for (word, n) in &freq {
        code.push_str(&format!("    (\"{word}\", {n}),\n"));
    }
    code.push_str("];\n");
    std::fs::write(out, code).unwrap();
}
