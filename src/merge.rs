pub fn merge_sort<T: Ord + Clone>(array: &[T]) -> Vec<T> {
    let n = array.len();
    if n <= 1 {
        return array.to_vec();
    }
    let mid = n / 2;

    let (left, right) = array.split_at(mid);
    let left = merge_sort(left);
    let right = merge_sort(right);
    merge(&left, &right)
}

fn merge<T: Ord + Clone>(left: &[T], right: &[T]) -> Vec<T> {
    let mut result = Vec::new();
    let mut i = 0;
    let mut j = 0;
    while i < left.len() || j < right.len() {
        if j == right.len() || (i != left.len() && left[i] <= right[j]) {
            result.push(left[i].clone());
            i += 1;
        } else {
            result.push(right[j].clone());
            j += 1;
        }
    }
    result
}


//nie działa :(
fn quick_merge<T: Ord>(array: &mut [T], breakpoint: usize) {
    let size = array.len();
    let mut insert = 0;
    let mut left = 0;
    let mut right = breakpoint;
    while left >= insert && left < right {
        if right != size && array[right] < array[left] {
            array.swap(insert, right);
            right += 1;
        } else {
            array.swap(insert, left);
            if right != size {
                left += 1;
            }
        }

        if left == insert {
            left = right - 1;
        } else if left == right {
            left = right - 1;
        }

        insert += 1;
    }
}
