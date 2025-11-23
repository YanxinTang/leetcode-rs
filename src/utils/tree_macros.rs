// 辅助宏：处理单个元素
// #[macro_export] 使其全局可用
#[macro_export]
#[doc(hidden)] // 这是一个辅助宏，通常用此属性隐藏它，避免污染文档
macro_rules! bfs_elem {
    (null) => {
        None
    };
    ($x:expr) => {
        Some($x as i32)
    }; // 明确转为 i32 类型
}

// 主宏：用于构建 Vec<Option<i32>>
#[macro_export]
macro_rules! bfs_vec {
    // 接受逗号分隔的元素列表
    ( $( $x:tt ),* ) => {
        {
            // 确保我们在当前作用域中导入了需要的类型
            use std::option::Option::{self, Some};

            let mut temp_vec: Vec<Option<i32>> = Vec::new();
            $(
                temp_vec.push($crate::bfs_elem!($x));
            )*
            temp_vec
        }
    };
}
