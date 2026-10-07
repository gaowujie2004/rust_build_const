// main.rs —— 消费第4层的产物
// include! 把 build.rs 生成的源码直接嵌入本 crate，
// 于是 WORDS / BUILD_TAG 变成普通的编译期常量。

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

fn main() {
    // 运行时零计算：WORDS 是编译进二进制的静态常量表
    println!("tag = {BUILD_TAG}");
    for (word, n) in WORDS {
        println!("{word}: {n}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn computed_at_build_time() {
        // main() 里没有任何"统计逻辑"，结果早已烙进二进制
        let total: usize = WORDS.iter().map(|(_, n)| n).sum();
        assert_eq!(total, 6); // data.txt 共 6 行
        assert_eq!(WORDS.len(), 3); // 去重后 3 个词
    }
}
