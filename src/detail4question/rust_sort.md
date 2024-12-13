# Rust实现多种排序算法？
- 以下是几种经典排序算法的Rust实现，包括冒泡排序、选择排序、插入排序、快速排序和归并排序，算法步骤和动图演示来源：https://www.runoob.com/w3cnote/ten-sorting-algorithm.html, 代码来自ChatGpt。

## 冒泡排序 (Bubble Sort)
- 逻辑：通过多轮相邻元素比较和交换，将最大（或最小）元素逐步移动到数组的一端。

- 算法步骤
    1. 比较相邻的元素。如果第一个比第二个大，就交换他们两个。
    2. 对每一对相邻元素作同样的工作，从开始第一对到结尾的最后一对。这步做完后，最后的元素会是最大的数。
    3. 针对所有的元素重复以上的步骤，除了最后一个。
    4. 持续每次对越来越少的元素重复上面的步骤，直到没有任何一对数字需要比较。

- 动图演示
![冒泡排序动图演示](https://www.runoob.com/wp-content/uploads/2019/03/bubbleSort.gif)

- 代码实现
```rust
fn bubble_sort(arr: &mut [i32]) {
    let len = arr.len();
    for i in 0..len {
        let mut swapped = false; // 优化
        for j in 0..len - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                swapped = true; // 优化 
            }
        }
        if !swapped { // 优化
            break;
        }
    }
}
```

## 选择排序 (Selection Sort)
- 逻辑：每轮从未排序部分选择最小（或最大）元素，与当前轮起始位置交换。

- 算法步骤
    1. 首先在未排序序列中找到最小（大）元素，存放到排序序列的起始位置。
    2. 再从剩余未排序元素中继续寻找最小（大）元素，然后放到已排序序列的末尾。
    3. 重复第二步，直到所有元素均排序完毕。

- 动图演示
![选择排序动图演示](https://www.runoob.com/wp-content/uploads/2019/03/selectionSort.gif)

- 代码实现
```rust
fn selection_sort(arr: &mut [i32]) {
    let len = arr.len();
    for i in 0..len {
        let mut min_index = i;
        for j in i + 1..len {
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }
        if min_index != i {
            arr.swap(i, min_index);
        }
    }
}
```

## 插入排序 (Insertion Sort)
- 逻辑:将未排序元素逐个插入到已排序部分的正确位置。

- 算法步骤
    1. 将第一待排序序列第一个元素看做一个有序序列，把第二个元素到最后一个元素当成是未排序序列。
    2. 从头到尾依次扫描未排序序列，将扫描到的每个元素插入有序序列的适当位置。（如果待插入的元素与有序序列中的某个元素相等，则将待插入元素插入到相等元素的后面。

- 动图演示
![插入排序动图演示](https://www.runoob.com/wp-content/uploads/2019/03/insertionSort.gif)

- 代码实现
```rust
fn insertion_sort(arr: &mut [i32]) {
    let len = arr.len();
    for i in 1..len {
        let mut j = i;
        while j > 0 && arr[j] < arr[j - 1] {
            arr.swap(j, j - 1);
            j -= 1;
        }
    }
}
```

## 快速排序 (Quick Sort)
- 逻辑：选择一个基准，将数组划分为小于基准和大于基准的两部分，递归排序。

- 算法步骤
    1. 从数列中挑出一个元素，称为 "基准"（pivot）。
    2. 重新排序数列，所有元素比基准值小的摆放在基准前面，所有元素比基准值大的摆在基准的后面（相同的数可以到任一边）。在这个分区退出之后，该基准就处于数列的中间位置。这个称为分区（partition）操作。
    3. 递归地（recursive）把小于基准值元素的子数列和大于基准值元素的子数列排序。

- 动图演示
![快速排序动图演示](https://www.runoob.com/wp-content/uploads/2019/03/quickSort.gif)

- 代码实现
```rust
fn quick_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }
    let pivot_index = partition(arr);
    quick_sort(&mut arr[0..pivot_index]);
    quick_sort(&mut arr[pivot_index + 1..]);
}
// 以最后一个元素为基准
fn partition(arr: &mut [i32]) -> usize {
    let pivot = arr[arr.len() - 1];
    let mut i = 0;
    for j in 0..arr.len() - 1 {
        if arr[j] <= pivot {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, arr.len() - 1);
    i
}
```

## 归并排序 (Merge Sort)
- 逻辑：递归地将数组分成两半，分别排序后合并成有序数组。

- 算法步
    1. 骤申请空间，使其大小为两个已经排序序列之和，该空间用来存放合并后的序列。
    2. 设定两个指针，最初位置分别为两个已经排序序列的起始位置。
    3. 比较两个指针所指向的元素，选择相对小的元素放入到合并空间，并移动指针到下一位置。
    4. 重复步骤 3 直到某一指针达到序列尾。
    5. 将另一序列剩下的所有元素直接复制到合并序列尾。

- 动图演示
![归并排序动图演示](https://www.runoob.com/wp-content/uploads/2019/03/mergeSort.gif)

- 代码实现
```rust
fn merge_sort(arr: &mut [i32]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }
    let mid = len / 2;
    let mut left = arr[0..mid].to_vec();
    let mut right = arr[mid..].to_vec();

    merge_sort(&mut left);
    merge_sort(&mut right);
    merge(arr, &left, &right);
}
fn merge(arr: &mut [i32], left: &[i32], right: &[i32]) {
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            arr[k] = left[i];
            i += 1;
        } else {
            arr[k] = right[j];
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        arr[k] = left[i];
        i += 1;
        k += 1;
    }

    while j < right.len() {
        arr[k] = right[j];
        j += 1;
        k += 1;
    }
}
```

## 测试代码
```rust
#[test]
fn test() {
    let mut arr = [64, 34, 25, 12, 22, 11, 90];
    
    // 调用不同的排序算法
    bubble_sort(&mut arr);
    println!("Bubble Sorted: {:?}", arr);

    let mut arr = [64, 34, 25, 12, 22, 11, 90];
    selection_sort(&mut arr);
    println!("Selection Sorted: {:?}", arr);

    let mut arr = [64, 34, 25, 12, 22, 11, 90];
    insertion_sort(&mut arr);
    println!("Insertion Sorted: {:?}", arr);

    let mut arr = [64, 34, 25, 12, 22, 11, 90];
    quick_sort(&mut arr);
    println!("Quick Sorted: {:?}", arr);

    let mut arr = [64, 34, 25, 12, 22, 11, 90];
    merge_sort(&mut arr);
    println!("Merge Sorted: {:?}", arr);
}
```