fn main() {
    let mut greed = [1, 2, 3];
    let mut cookies = [1, 1];
    println!("{}", find_content_children(&mut greed, &mut cookies));
}

fn bubble_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    for i in 0..len {
        let mut swapped = false;
        for j in 0..len - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
}

fn find_content_children<T: Ord>(greed: &mut [T], cookies: &mut [T]) -> usize {
    bubble_sort(greed);
    bubble_sort(cookies);

    let (mut child, mut cookie) = (0, 0);
    while child < greed.len() && cookie < cookies.len() {
        if cookies[cookie] >= greed[child] {
            child += 1;
        }
        cookie += 1;
    }

    child
}

#[cfg(test)]
mod tests {
    use super::{bubble_sort, find_content_children};

    #[test]
    fn bubble_sort_orders_generic_values_and_accepts_empty_input() {
        let mut numbers = [5, 1, 4, 2, 8];
        bubble_sort(&mut numbers);
        assert_eq!(numbers, [1, 2, 4, 5, 8]);

        let mut words = ["pear", "apple", "orange"];
        bubble_sort(&mut words);
        assert_eq!(words, ["apple", "orange", "pear"]);

        let mut empty: [i32; 0] = [];
        bubble_sort(&mut empty);
        assert_eq!(empty, []);
    }

    #[test]
    fn find_content_children_maximizes_assignments() {
        let mut greed = [1, 2, 3];
        let mut cookies = [1, 1];
        assert_eq!(find_content_children(&mut greed, &mut cookies), 1);

        let mut greed = [1, 2];
        let mut cookies = [1, 2, 3];
        assert_eq!(find_content_children(&mut greed, &mut cookies), 2);
    }

    #[test]
    fn find_content_children_accepts_empty_input() {
        let mut no_children: [u32; 0] = [];
        let mut cookies = [1];
        assert_eq!(find_content_children(&mut no_children, &mut cookies), 0);

        let mut children = [1];
        let mut no_cookies: [u32; 0] = [];
        assert_eq!(find_content_children(&mut children, &mut no_cookies), 0);
    }
}
