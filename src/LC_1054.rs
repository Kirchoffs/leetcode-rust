use std::collections::HashMap;

#[derive(Debug)]
struct Solution {}

impl Solution {
    pub fn rearrange_barcodes(barcodes: Vec<i32>) -> Vec<i32> {
        let total = barcodes.len();
        
        let mut cnt: HashMap<i32, i32> = HashMap::new();
        let mut max_barcode = 0;
        let mut max_barcode_cnt = 0;
        for &barcode in barcodes.iter() {
            cnt.insert(barcode, if cnt.contains_key(&barcode) { cnt[&barcode] } else { 0 } + 1);
            if cnt[&barcode] > max_barcode_cnt {
                max_barcode_cnt = cnt[&barcode];
                max_barcode = barcode;
            }
        }
        
        let mut res = vec![0; total];
        let mut index = 0;
        let mut fill = |num, cnt| {
            for _ in 0..cnt {
                res[index] = num;
                index += 2;
                if index >= res.len() {
                    index = 1;
                }
            }
        };
        
        fill(max_barcode, max_barcode_cnt);
        for (&k, &v) in cnt.iter() {
            if k != max_barcode {
                fill(k, v);
            }
        }
        
        res
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn LC_1054_1() {
        let barcodes = vec![1, 1, 1, 2, 2, 2];
        println!("{:?}", Solution::rearrange_barcodes(barcodes));
    }

    #[test]
    fn LC_1054_2() {
        let barcodes = vec![1, 1, 1, 1, 2, 2, 3, 3];
        println!("{:?}", Solution::rearrange_barcodes(barcodes));
    }
}
