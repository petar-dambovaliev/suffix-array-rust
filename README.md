# Suffix Array
This package provides common functionality associated with suffix arrays.
It uses the LCP array addition as well.

```
    let s = String::from("azaza");
    let sa = array::new(s.as_str());
    let subs = sa.distinct_sub();
    
    
    let s = String::from("azaza");
    let sa = array::new(s.as_str());
    assert_eq!(9, sa.distinct_sub_count());
    
    let s = String::from("azaza");
    let sa = array::new(s.as_str());
    assert_eq!(15, sa.sub_count());
    
    let s = String::from("abracadabra");
    let sa = array::new(s.as_str());
    let lrs = sa.longest_repeated_substr();

    assert_eq!(1, lrs.len());
    assert_eq!("abra", str::from_utf8(&lrs[0]).unwrap());
```
