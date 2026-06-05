# Claude 风格 Markdown 查看器

这是一个使用 `mdview` 渲染的示例文档，展示了各种 Markdown 元素的 Claude 风格排版效果。

## 文本格式化

这是一段普通文本。你可以使用 **粗体**、*斜体* 或 ***粗斜体*** 来强调内容。

> 这是一个引用块，用于展示重要信息或引用他人的话语。
> 
> — Claude

## 代码展示

### 行内代码

使用 `console.log()` 来调试 JavaScript 代码，或者用 `std::println!()` 在 Rust 中输出。

### 代码块

```python
def fibonacci(n: int) -> list[int]:
    """生成斐波那契数列"""
    if n <= 0:
        return []
    
    fib = [0, 1]
    for i in range(2, n):
        fib.append(fib[i-1] + fib[i-2])
    
    return fib[:n]

# 使用示例
result = fibonacci(10)
print(f"斐波那契数列前10项: {result}")
```

```cpp
#include <iostream>
#include <vector>
#include <algorithm>

template<typename T>
auto sort_and_print(std::vector<T> vec) -> std::vector<T> {
    std::sort(vec.begin(), vec.end());
    
    std::cout << "排序结果: ";
    for (const auto& item : vec) {
        std::cout << item << " ";
    }
    std::cout << std::endl;
    
    return vec;
}

int main() {
    std::vector<int> numbers = {5, 2, 8, 1, 9, 3};
    auto sorted = sort_and_print(numbers);
    return 0;
}
```

```rust
use std::collections::HashMap;

fn main() {
    let mut scores: HashMap<&str, i32> = HashMap::new();
    
    scores.insert("Alice", 95);
    scores.insert("Bob", 87);
    scores.insert("Charlie", 92);
    
    for (name, score) in &scores {
        println!("{name}: {score}");
    }
}
```

## 数学公式

### 行内公式

质数的定义：一个大于 1 的自然数 $p$，如果只有 1 和 $p$ 本身两个因数，则称 $p$ 为质数。

著名的欧拉公式：$e^{i\pi} + 1 = 0$

### 块级公式

二次方程的求根公式：

$$x = \frac{-b \pm \sqrt{b^2 - 4ac}}{2a}$$

积分的基本定理：

$$\int_a^b f(x) dx = F(b) - F(a)$$

其中 $F'(x) = f(x)$。

矩阵表示：

$$\begin{pmatrix} a & b \\ c & d \end{pmatrix} \begin{pmatrix} x \\ y \end{pmatrix} = \begin{pmatrix} ax + by \\ cx + dy \end{pmatrix}$$

## 列表

### 无序列表

- 第一项
- 第二项
  - 子项 A
  - 子项 B
- 第三项

### 有序列表

1. 准备开发环境
2. 安装依赖
3. 编写代码
4. 运行测试

## 表格

| 语言 | 类型系统 | 特点 |
|------|----------|------|
| Rust | 静态、强类型 | 内存安全、零成本抽象 |
| Python | 动态、强类型 | 简洁易读、生态丰富 |
| C++ | 静态、弱类型 | 高性能、底层控制 |

## 链接与图片

访问 [Rust 官方网站](https://www.rust-lang.org) 了解更多。

---

## 水平分割线

上面是一条分割线，用于分隔不同的内容区域。

---

*文档由 mdview 渲染 - Claude 风格排版*
