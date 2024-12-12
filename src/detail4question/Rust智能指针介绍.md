# Rust 中的智能指针介绍

Rust 中的智能指针是一种不仅仅包含数据地址的指针，还具有额外功能的数据结构。它们通常实现了 `Deref` 和 `Drop` 两个特质，提供指针操作和生命周期管理能力。以下是 Rust 中常见的智能指针及其特点：

---

## 1. Box<T>
- **用途**：
  - 将数据存储在堆上，而非栈上。
  - 用于递归类型（如树结构）。
- **特点**：
  - 数据的所有权被 `Box` 所持有。
  - 数据在堆上分配，访问时需通过解引用。
- **示例**：
    ```rust
    let b = Box::new(5);
    println!("b = {}", b); // 输出 b = 5
    ```

---

## 2. Rc<T>
- **用途**：
  - 在单线程环境下，实现多所有权（引用计数）。
  - 用于需要共享数据的场景。
- **特点**：
  - 引用计数递增时，克隆 `Rc` 不会复制数据。
  - 只支持不可变引用。
- **示例**：
    ```rust
    use std::rc::Rc;

    let a = Rc::new(5);
    let b = Rc::clone(&a);
    println!("a = {}, b = {}", a, b); // 输出 a = 5, b = 5
    ```

---

## 3. Arc<T>
- **用途**：
  - 在多线程环境下，实现多所有权（线程安全的引用计数）。
- **特点**：
  - 内部实现了线程安全机制（如原子操作）。
  - 性能比 `Rc` 略低，但可以在线程间安全共享。
- **示例**：
    ```rust
    use std::sync::Arc;
    use std::thread;

    let a = Arc::new(5);
    let a_clone = Arc::clone(&a);

    let handle = thread::spawn(move || {
        println!("From thread: {}", a_clone); // 输出 From thread: 5
    });

    handle.join().unwrap();
    ```

---

## 4. RefCell<T>
- **用途**：
  - 在单线程环境下实现运行时的可变性。
  - 允许在不可变上下文中对数据进行可变借用。
- **特点**：
  - 通过运行时检查（借用检查）确保可变性。
  - 如果违反借用规则，会在运行时报错。
- **示例**：
    ```rust
    use std::cell::RefCell;

    let data = RefCell::new(5);
    *data.borrow_mut() += 1;
    println!("{}", data.borrow()); // 输出 6
    ```

---

## 5. Mutex<T>
- **用途**：
  - 用于多线程环境下的互斥锁，确保数据在同一时间只能被一个线程访问。
- **特点**：
  - 提供内部可变性。
  - 阻塞线程，直到锁被释放。
- **示例**：
    ```rust
    use std::sync::Mutex;

    let m = Mutex::new(5);

    {
        let mut data = m.lock().unwrap();
        *data += 1;
    }

    println!("{}", m.lock().unwrap()); // 输出 6
    ```

---

## 6. RwLock<T>
- **用途**：
  - 类似于 `Mutex`，但允许多个读者或一个写者。
- **特点**：
  - 提供读写锁，优化读多写少场景。
- **示例**：
    ```rust
    use std::sync::RwLock;

    let lock = RwLock::new(5);

    // 读锁
    {
        let r = lock.read().unwrap();
        println!("Read lock: {}", r); // 输出 Read lock: 5
    }

    // 写锁
    {
        let mut w = lock.write().unwrap();
        *w += 1;
    }

    println!("Updated value: {}", lock.read().unwrap()); // 输出 Updated value: 6
    ```

---

## 7. 内部可变性与线程安全
- **RefCell 和 Mutex/RwLock 的对比**：
  - `RefCell`：单线程环境下提供运行时的内部可变性。
  - `Mutex/RwLock`：多线程环境下提供线程安全的内部可变性。
- **注意**：
  - `RefCell` 和 `Mutex` 都会在不遵循规则时引发运行时错误。
  - 使用 `Mutex` 或 `RwLock` 时要注意避免死锁。

---

## 总结
- Rust 的智能指针提供了不同的功能和适用场景：
  - `Box`：堆分配和递归类型。
  - `Rc` 和 `Arc`：多所有权。
  - `RefCell`：运行时可变性。
  - `Mutex` 和 `RwLock`：线程安全的同步工具。
- 根据具体需求选择合适的智能指针可以充分利用 Rust 的内存安全和性能优势。
