# 这是一些rust面试题


## https://course.rs/practice/interview.html

- Bitcoin和Ethereum如何防止双花攻击


// ￼rust（异步）线程池结构体，以及工作方式

// ￼rust实现排序算法: 快速排序/冒泡排序
| 排序算法 | 时间复杂度 | 空间复杂度 | 稳定性 | 适用场景 | 算法特点 |
|----------|------------|------------|--------|----------|----------|
| 冒泡排序 | O(n²) | O(1) | 稳定 | 小规模数据 | 简单直观，交换相邻元素 |
| 快速排序 | O(n log n) | O(log n) | 不稳定 | 大规模数据 | 分治法，性能优秀 |
| 归并排序 | O(n log n) | O(n) | 稳定 | 需要稳定排序 | 分治，额外空间开销 |
| 堆排序 | O(n log n) | O(1) | 不稳定 | 找topK元素 | 原地排序，堆结构 |
| 插入排序 | O(n²) | O(1) | 稳定 | 小规模有序 | 类似打扑克牌 |
| 选择排序 | O(n²) | O(1) | 不稳定 | 小规模数据 | 每轮选择最小元素 |
| 希尔排序 | O(n log n) | O(1) | 不稳定 | 中等规模 | 改进版插入排序 |
| 计数排序 | O(n+k) | O(k) | 稳定 | 整数范围小 | 非比较排序 |
| 基数排序 | O(nk) | O(n+k) | 稳定 | 整数排序 | 按位排序 |
| 桶排序 | O(n+k) | O(n+k) | 稳定 | 数据分布均匀 | 分桶排序 |


// ￼rust可变引用/不可变引用可以用多个吗


// ￼rust copy()和clone()的区别
// ￼Rust中的智能指针有三个，Box<T>、Rc<T>、Arc<T>。其中：
    
// ￼数据库索引命中规则(最左匹配)
// ￼Send, Sync 的问题/Send Sync ?Size分别解释
// ￼Box<dyn(Fn() + Send + 'static)>的表达式的意思
// ￼HTTP TCP/IP相关问题
// ￼解释一下Rust中Deref 、Drop、Clone、Copy、Any 这几个trait。
// ￼trait 和 trait object 的问题    
// ￼fn Fn FnMut FnOnce 的区别 https://mp.weixin.qq.com/s/-l00_EvV6v9pQrsr-QRAHQ
// ￼锁有几种/锁中毒