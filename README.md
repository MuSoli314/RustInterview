# 这是一些rust面试题


## https://course.rs/practice/interview.html

- Bitcoin和Ethereum如何防止双花攻击

        Bitcoin和Ethereum通过各自的共识机制防止双花攻击。Bitcoin使用工作量证明（PoW）来确保交易的顺序和真实性，通过矿工竞争解决复杂的数学问题来验证交易。而Ethereum虽然最初也采用PoW，但逐渐过渡到权益证明（PoS），通过质押代币验证交易。这些机制都使得伪造或重复使用相同的交易变得极为困难，从而防止双花攻击。

// ￼rust（异步）线程池结构体，以及工作方式

// ￼rust实现排序算法: 快速排序/冒泡排序

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

# 数据库索引命中规则

数据库索引的命中规则是指查询操作是否能够利用索引来提高查询效率。理解这些规则有助于设计高效的查询和合理的索引结构。以下是常见的数据库索引命中规则和注意事项：

---

## 1. 基本索引命中规则

### 1.1 全值匹配
- 当查询条件完全匹配索引字段时，索引可以被命中。
- **示例**：假设表中有索引 `idx_name`，覆盖字段为 `name`。
    ```sql
    SELECT * FROM users WHERE name = 'John';
    ```
  - 查询条件与索引字段完全匹配，索引被命中。

### 1.2 最左前缀匹配
- 对于多列联合索引，只有查询条件匹配索引的**最左字段**开始时，索引才能被命中。
- **示例**：联合索引 `idx_name_age (name, age)`。
    ```sql
    SELECT * FROM users WHERE name = 'John'; -- 索引命中
    SELECT * FROM users WHERE name = 'John' AND age = 25; -- 索引命中
    SELECT * FROM users WHERE age = 25; -- 索引不命中（没有用到最左字段）
    ```

### 1.3 范围查询规则
- 如果查询中使用了范围查询（如 `<`, `>`, `BETWEEN`, `LIKE 'abc%'`），索引可以命中，但范围字段后面的索引列将无法使用。
- **示例**：联合索引 `idx_name_age (name, age)`。
    ```sql
    SELECT * FROM users WHERE name = 'John' AND age > 25; -- 部分索引命中（只使用 name 部分）
    ```

### 1.4 精确匹配 + 范围匹配
- 精确匹配的字段依然可以使用索引，但范围匹配的字段后续索引列无效。
- **示例**：联合索引 `idx_a_b_c (a, b, c)`。
    ```sql
    SELECT * FROM table WHERE a = 1 AND b > 2 AND c = 3;
    -- 索引命中 a 和 b，无法利用 c。
    ```

---

## 2. 查询条件影响索引命中

### 2.1 函数操作会导致索引失效
- 如果在查询条件中对索引字段使用了函数或表达式，索引无法命中。
- **示例**：
    ```sql
    SELECT * FROM users WHERE UPPER(name) = 'JOHN'; -- 索引不命中
    ```

### 2.2 数据类型不一致
- 查询条件中的值类型必须与索引字段的数据类型一致，否则索引可能无法命中。
- **示例**：
    ```sql
    SELECT * FROM users WHERE name = 123; -- 如果 name 是字符串类型，索引无法命中
    ```

### 2.3 隐式转换
- 如果查询条件触发隐式类型转换，索引会失效。
- **示例**：
    ```sql
    SELECT * FROM users WHERE name = '123'; -- 如果 name 是整数类型，索引可能失效
    ```

### 2.4 NULL 值查询
- 对于某些数据库（如 MySQL），如果索引字段中包含 NULL 值，则 `IS NULL` 和 `IS NOT NULL` 查询可能无法利用索引。

---

## 3. 特殊查询模式

### 3.1 LIKE 模式
- 使用 `LIKE` 时：
  - 如果模式以常量开头（如 `LIKE 'abc%'`），索引可以命中。
  - 如果模式以通配符开头（如 `LIKE '%abc'`），索引无法命中。
- **示例**：
    ```sql
    SELECT * FROM users WHERE name LIKE 'John%'; -- 索引命中
    SELECT * FROM users WHERE name LIKE '%John'; -- 索引不命中
    ```

### 3.2 OR 条件
- 如果 `OR` 条件中的所有字段都能使用相同的索引，则索引可以命中；否则，索引失效。
- **示例**：
    ```sql
    SELECT * FROM users WHERE name = 'John' OR age = 25; -- 索引可能不命中
    ```

### 3.3 IN 子句
- `IN` 条件通常可以命中索引，但如果 `IN` 中的字段太多，性能可能受限。
- **示例**：
    ```sql
    SELECT * FROM users WHERE name IN ('John', 'Alice', 'Bob'); -- 索引命中
    ```

---

## 4. 查询优化的影响

### 4.1 覆盖索引
- 如果查询的所有字段都包含在索引中，查询可以直接通过索引返回结果（称为覆盖索引），提高效率。
- **示例**：
    ```sql
    SELECT name, age FROM users WHERE name = 'John'; -- 如果有索引 (name, age)，可利用覆盖索引
    ```

### 4.2 分组和排序
- 如果 `GROUP BY` 或 `ORDER BY` 使用的字段在索引中，且排序方式与索引顺序一致，则索引可以加速分组或排序。
- **示例**：
    ```sql
    SELECT * FROM users ORDER BY name; -- 索引命中
    SELECT * FROM users ORDER BY age; -- 索引不命中（如果索引不包含 age）
    ```

---

## 5. 索引失效的其他原因

### 5.1 表扫描
- 如果查询需要访问的数据量较大（超过一定比例），数据库可能会选择全表扫描而不是使用索引。

### 5.2 数据分布不均
- 如果索引字段的值分布高度集中，数据库可能认为使用索引的效率不高，进而选择全表扫描。

### 5.3 SQL 查询优化器
- 数据库的查询优化器可能根据数据统计信息动态选择是否使用索引。

---

## 最佳实践
1. 尽量避免对索引字段使用函数、表达式或隐式类型转换。
2. 确保查询条件中的字段顺序与索引的最左前缀顺序一致。
3. 使用覆盖索引来减少回表查询的开销。
4. 定期分析表的统计信息，确保数据库优化器能够正确选择索引。
5. 避免频繁更新的字段作为索引，减少索引维护成本。

通过遵守这些规则，可以最大化索引的使用效率，从而提高数据库查询性能。