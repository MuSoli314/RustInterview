// 问题

// 如何用 Rust 实现多种排序算法？

// 解答

// 以下是几种经典排序算法的 Rust 实现，包括冒泡排序、选择排序、插入排序、快速排序和归并排序。

// 冒泡排序 (Bubble Sort)
fn bubble_sort(arr: &mut [i32]) {
    let len = arr.len();
    for i in 0..len {
        for j in 0..len - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

// 选择排序 (Selection Sort)
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

// 插入排序 (Insertion Sort)
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

// 快速排序 (Quick Sort)
fn quick_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }
    let pivot_index = partition(arr);
    quick_sort(&mut arr[0..pivot_index]);
    quick_sort(&mut arr[pivot_index + 1..]);
}

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

// 归并排序 (Merge Sort)
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

// 测试代码
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