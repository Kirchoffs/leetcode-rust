# LeetcodeWithRust

## Test
To do the test, use `cargo test` commands.  

```  
To test the lc_1368_1 method in lc_1368:
>> cargo test lc_1368_1 -- --nocapture  

To test all the tests in lc_1644:  
>> cargo test lc_1644 -- --nocapture
```

## Rust Detail
### Sort Vec of floats
```
let mut arr = vec![0.618, 2.718, 0.367];
arr.sort_by(|a, b| b.partial_cmp(a).unwrap());
```

### Max value of two floats
```
let a: f64 = 2.718;
let b: f64 = 0.367;
let c = a.max(b);
```

### Convert String to Vec<char>
```
let chs = s.chars().collect::<Vec<char>>();
```
or
```
let chs: Vec<char> = s.chars().collect();
```

### Zip & Enumerate
In Python, we have:
```
for idx, (num1, num2) in enumerate(zip(nums1, nums2)):
    ...
```

In Rust, we have similar code:
```
for (idx, (&num1, &num2)) in nums1.iter().zip(nums2.iter()).enumerate() { 
    ...
}
```
