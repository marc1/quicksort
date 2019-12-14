pub fn qsort(v: &mut Vec<i32>) {
    if v.len() <= 1 {
        return;
    }

    let piv_i = 0;
    let piv = v[piv_i];

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for (i, x) in v.iter().enumerate() {
        if i == piv_i {
            continue;
        }

        if x <= &piv {
            left.push(*x);
        } else if x > &piv {
            right.push(*x);
        }
    }

    qsort(&mut left);
    qsort(&mut right);

    left.push(piv);
    left.append(&mut right);

    *v = left;
}

#[cfg(test)]
mod tests {
    use crate::qsort;

    #[test]
    fn sort() {
        let mut a = vec!(4, 3, 2, 5, 1);

        qsort(&mut a);

        assert_eq!(a, vec!(1, 2, 3, 4, 5));
    }
}
