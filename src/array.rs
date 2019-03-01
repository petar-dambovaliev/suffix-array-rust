use std::cmp::Ordering;

#[derive(Debug)]
pub struct Array {
    txt: Vec<u8>,
    sa: Vec<usize>,
    lcp: Vec<usize>,
}

#[derive(Debug, Clone)]
struct Suffix {
    index: usize,
    rank: [i8; 2],
}

#[allow(dead_code)]
impl Array {
    pub fn distinct_sub_count(&self) -> usize {
        let mut dup: usize = 0;

        for i in &self.lcp {
            dup += i
        }

        return self.sub_count() - dup;
    }

    pub fn sub_count(&self) -> usize {
        return (self.txt.len() * (self.txt.len() + 1)) / 2;
    }

    pub fn longest_repeated_substr(&self) -> Vec<Vec<u8>> {
        let mut max = 0;
        let mut key = 0;
        let mut lrs: Vec<Vec<u8>> = Vec::new();

        for k in 0..self.lcp.len() {
            if self.lcp[k] > max {
                max = self.lcp[k];
                key = k;
            }
        }

        lrs.push(self.txt[self.sa[key]..self.sa[key] + max].to_vec());
        let mut i = key + 1;

        while i < self.lcp.len() {
            if self.lcp[i] == max {
                lrs.push(self.txt[self.sa[i]..self.sa[i] + max].to_vec());
            }
            i += 1;
        }
        return lrs;
    }

    pub fn distinct_sub(&self) -> Vec<Vec<u8>> {
        let mut dist: Vec<Vec<u8>> = Vec::new();
        let mut x;

        for i in 0..self.sa.len() {
            x = self.lcp[i] + self.sa[i];

            while x < self.sa.len() {
                dist.push(self.txt[self.sa[i]..x + 1].to_vec());
                x += 1
            }
        }

        return dist;
    }
}

#[allow(dead_code)]
pub fn new(txt: &str) -> Array {
    let txt = txt.as_bytes().to_vec();
    let sa = new_array(&txt);
    let lcp = new_lcp(&txt, &sa);

    return Array { txt, sa, lcp };
}

fn new_lcp(txt: &Vec<u8>, sa: &Vec<usize>) -> Vec<usize> {
    let n = sa.len();
    let mut lcp = vec![0; n];
    let (mut common, mut s1, mut s2): (usize, usize, usize);
    common = 0;

    for i in 1..n {
        s1 = sa[i - 1];
        s2 = sa[i];

        while s1 < txt.len() && s2 < txt.len() && txt[s1] == txt[s2] {
            common += 1;
            s1 += 1;
            s2 += 1;
        }
        lcp[i] = common;
        common = 0
    }

    return lcp;
}

fn new_array(txt: &Vec<u8>) -> Vec<usize> {
    let n = txt.len();
    let mut suffixes = vec![
        Suffix {
            index: 0,
            rank: [0, 0]
        };
        n
    ];

    for i in 0..n {
        suffixes[i].index = i;
        suffixes[i].rank[0] = txt[i] as i8 - 'a' as i8;

        if i + 1 < n {
            suffixes[i].rank[1] = txt[i + 1] as i8 - 'a' as i8;
        } else {
            suffixes[i].rank[1] = -1;
        }
    }

    let sort_func = |i: &Suffix, j: &Suffix| -> Ordering {
        if i.rank[0] == j.rank[0] {
            if i.rank[1] < j.rank[1] {
                return Ordering::Less;
            }
            return Ordering::Greater;
        }

        if i.rank[0] < j.rank[0] {
            return Ordering::Less;
        }
        return Ordering::Greater;
    };

    suffixes.sort_by(sort_func);

    let mut ind = vec![0; n];
    let mut k = 4;

    while k < 2 * n {
        let mut rank = 0;
        let mut prev_rank = suffixes[0].rank[0];

        suffixes[0].rank[0] = rank;
        ind[suffixes[0].index] = 0;

        for i in 1..n {
            if suffixes[i].rank[0] == prev_rank && suffixes[i].rank[1] == suffixes[i - 1].rank[1] {
                prev_rank = suffixes[i].rank[0];
                suffixes[i].rank[0] = rank
            } else {
                prev_rank = suffixes[i].rank[0];
                rank += 1;
                suffixes[i].rank[0] = rank;
            }
            ind[suffixes[i].index] = i
        }

        for i in 0..n {
            let nextindex = suffixes[i].index + k / 2;
            if nextindex < n {
                suffixes[i].rank[1] = suffixes[ind[nextindex]].rank[0];
            } else {
                suffixes[i].rank[1] = -1;
            }
        }

        suffixes.sort_by(sort_func);
        k *= 2;
    }

    let mut suffix_arr = vec![0; n];

    for i in 0..n {
        suffix_arr[i] = suffixes[i].index;
    }

    return suffix_arr;
}
