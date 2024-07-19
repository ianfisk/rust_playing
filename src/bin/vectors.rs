mod maths {
    use std::collections::{HashMap, HashSet};

    fn count_values(v: &Vec<i32>) -> HashMap<&i32, i32> {
        let mut value_counts = HashMap::new();
        for val in v {
            let count = value_counts.entry(val).or_insert(0);
            *count += 1;
        }

        value_counts
    }

    pub fn mode_n_lgn(v: &Vec<i32>) -> Option<HashSet<i32>> {
        let mut value_counts: Vec<(i32, i32)> =
            count_values(v).iter().map(|(&&k, &v)| (k, v)).collect();
        value_counts.sort_by(|a, b| b.1.cmp(&a.1));

        let most_freq_item = value_counts.first()?;
        Some(HashSet::from_iter(
            value_counts
                .iter()
                .take_while(|x| x.1 == most_freq_item.1)
                .map(|&(item, _)| item),
        ))
    }

    pub fn mode_n(v: &Vec<i32>) -> Option<HashSet<i32>> {
        let value_counts = count_values(v);
        let (_, &most_freq_count) = value_counts.iter().max_by_key(|(_, &count)| count)?;

        Some(HashSet::from_iter(
            value_counts
                .iter()
                .filter(|(_, &count)| count == most_freq_count)
                .map(|(&&item, _)| item),
        ))
    }

    pub fn median(v: &Vec<i32>) -> Option<f32> {
        println!("Median of {v:?}");

        let mut v: Vec<i32> = v.clone();
        v.sort();

        match v.len() % 2 {
            0 => {
                println!("vec has even number of elements");

                // Using the (?) operator on Option<T> short circuits and propagates None if a value is not present.
                // https://doc.rust-lang.org/reference/expressions/operator-expr.html#the-question-mark-operator
                let upper_index = v.len() / 2;
                let upper_val = *v.get(upper_index)? as f32;
                let lower_val = *v.get(upper_index - 1)? as f32;

                println!("v.get(upper_index - 1) = {:?}", upper_val);
                println!("v.get(upper_index) = {:?}", lower_val);

                Some((upper_val + lower_val) / 2.0)
            }
            1 => Some(v[v.len() / 2] as f32), // Integer division
            _ => None,
        }
    }
}

fn main() {
    let v1 = vec![1, 2, 3, 3, 4, 2, 3];
    let v2 = vec![1, 2, 3, 3, 4, 2, 3, 4, 4];
    let v3 = vec![2, 5, 1, 4, 3];
    let v4 = vec![1, 2, 3, 4, 5, 6];
    let v5 = vec![10];
    let v6: Vec<i32> = vec![];
    let v7 = Vec::new();

    for v in [v1, v2, v3, v4, v5, v6, v7] {
        println!("Vec {v:?}");
        let mode = maths::mode_n_lgn(&v);
        println!("Mode (O(nlgn)) = {mode:?}");

        let mode = maths::mode_n(&v);
        println!("Mode (O(n)) = {mode:?}");

        let median = maths::median(&v);
        println!("Median = {median:?}");
        println!();
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn mode_n_works_one_mode() {
        let v = vec![1, -2, 3, 3, 4, -2, 3];

        assert_eq!(maths::mode_n(&v).unwrap(), HashSet::from([3]));
    }

    #[test]
    fn mode_n_works_many_modes() {
        let v = vec![1, 2, 3, 4, 5];

        assert_eq!(
            maths::mode_n(&v).unwrap(),
            HashSet::from_iter(v.into_iter())
        );
    }

    #[test]
    fn mode_n_returns_none_with_empty_vector() {
        assert_eq!(maths::mode_n(&Vec::new()), None);
    }

    #[test]
    fn median_works_with_odd_num_elements() {
        let v = vec![3, 2, 2, 2, 1];
        let expected = 2.0;

        assert_eq!(maths::median(&v).unwrap(), expected);
    }

    #[test]
    fn median_works_with_even_num_elements_same_middle_values() {
        let v = vec![3, 2, 2, 1];
        let expected = 2.0;

        assert_eq!(maths::median(&v).unwrap(), expected);
    }

    #[test]
    fn median_works_with_even_num_elements_different_middle_values() {
        let v = vec![4, 3, 2, 1];
        let expected = 2.5;

        assert_eq!(maths::median(&v).unwrap(), expected);
    }

    #[test]
    fn median_works_with_one_element_vector() {
        let v = vec![1];
        let expected = 1.0;

        assert_eq!(maths::median(&v).unwrap(), expected);
    }

    #[test]
    fn median_works_with_two_element_vector() {
        let v = vec![1, 10];
        let expected = 5.5;

        assert_eq!(maths::median(&v).unwrap(), expected);
    }

    #[test]
    fn median_returns_none_with_empty_vector() {
        assert_eq!(maths::median(&Vec::new()), None);
    }
}
