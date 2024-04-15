pub fn dual_pivot_quicksort<T: Ord>(array: &mut [T]) -> (usize, usize) {
    let len = array.len();
    if len <= 1 {
        return (0, 0);
    }
    let (mut comps, mut swaps) = (0, 0);

    let ((p1, p2), out) = dual_pivot_partition(array);
    swaps += out.1;
    comps += out.0;

    let out = dual_pivot_quicksort(&mut array[..p1]);
    swaps += out.1;
    comps += out.0;

    let out = dual_pivot_quicksort(&mut array[p1 + 1..p2]);
    swaps += out.1;
    comps += out.0;

    let out = dual_pivot_quicksort(&mut array[p2 + 1..]);
    swaps += out.1;
    comps += out.0;

    (comps, swaps)
}

fn dual_pivot_partition<T: Ord>(array: &mut [T]) -> ((usize, usize), (usize, usize)) {
    let mut comps = 1;
    let mut swaps = 0;
    let mut smaller = 0;
    let mut bigger = 0;
    let (left_pivot, right_pivot) = (0, array.len() - 1);

    if array[left_pivot] > array[right_pivot] {
        swaps += 1;
        array.swap(left_pivot, right_pivot);
    }

    let mut left = left_pivot + 1;
    let mut right = right_pivot - 1;
    let mut iter = left;

    while iter <= right {
        if bigger > smaller {
            if {
                comps += 1;
                array[iter] > array[right_pivot]
            } {
                array.swap(iter, right);
                swaps += 1;

                right -= 1;
                bigger += 1;
            } else if {
                comps += 1;
                array[iter] < array[left_pivot]
            } {
                array.swap(iter, left);
                swaps += 1;

                left += 1;
                iter += 1;
                smaller += 1
            } else {
                iter += 1;
            }
        } else {
            if {
                comps += 1;
                array[iter] < array[left_pivot]
            } {
                array.swap(iter, left);
                swaps += 1;

                left += 1;
                iter += 1;
                smaller += 1;
            } else if {
                comps += 1;
                array[iter] > array[right_pivot]
            } {
                array.swap(iter, right);
                swaps += 1;

                right -= 1;
                bigger += 1;
            } else {
                iter += 1;
            }
        }
    }

    left -= 1;
    right += 1;
    array.swap(left_pivot, left);
    array.swap(right_pivot, right);

    ((left, right), (comps, swaps))
}