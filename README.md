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
let c = a.min(b);
```