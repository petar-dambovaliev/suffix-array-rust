#[cfg(test)]
use std::str;
mod array;

#[test]
fn distinct_sub_count() {
    let s = String::from("azaza");
    let sa = array::new(s.as_str());
    assert_eq!(9, sa.distinct_sub_count());
}

#[test]
fn distinct_sub() {
    let s = String::from("azaza");
    let sa = array::new(s.as_str());
    let subs = sa.distinct_sub();
    let real_subs = ["a", "az", "aza", "azaz", "azaza", "z", "za", "zaz", "zaza"];

    assert_eq!(real_subs.len(), subs.len());

    for i in 0..subs.len() {
        let rs = real_subs[i].as_bytes();

        for k in 0..subs[i].len() {
            assert_eq!(rs[k], subs[i][k]);
        }
    }
}

#[test]
fn sub_count() {
    let s = String::from("azaza");
    let sa = array::new(s.as_str());
    assert_eq!(15, sa.sub_count());
}

#[test]
fn longest_repeated_substr() {
    let s = String::from("abracadabra");
    let sa = array::new(s.as_str());
    let lrs = sa.longest_repeated_substr();

    assert_eq!(1, lrs.len());
    assert_eq!("abra", str::from_utf8(&lrs[0]).unwrap());
}
