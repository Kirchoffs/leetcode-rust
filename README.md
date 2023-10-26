# LeetcodeWithRust

## Test
To do the test, use `cargo test` commands.  

```  
To test the lc_1368_1 method in lc_1368:
>> cargo test lc_1368_1 -- --nocapture  

To test all the tests in lc_1644:  
>> cargo test lc_1644 -- --nocapture
```

## Problems Index
### Sort
#### Quick Select
- LC-347

## Rust Detail
### HashMap
```
let mut map = HashMap::new();

// insert / insert
map.insert(1, 2);

// Get
let val = map.get(&1).unwrap();

// remove
map.remove(&1);

// Get all the keys
let keys = map.keys().map(|&x| x).collect::<Vec<i32>>();

// Get or insert default
let val = map.entry(1).or_insert(-1);
```

### Vec
#### Convert Vec to HashSet
```
let set = vec.into_iter().collect::<HashSet<i32>>();
```

```
let set = vec.iter().cloned().collect::<HashSet<i32>>();
```

#### Sort Vec of floats
```
let mut arr = vec![0.618, 2.718, 0.367];
arr.sort_by(|a, b| b.partial_cmp(a).unwrap());
```

#### Convert String to Vec<char>
```
let chs = s.chars().collect::<Vec<char>>();
```

```
let chs: Vec<char> = s.chars().collect();
```

### Compare

#### Max value of two floats
```
let a: f64 = 2.718;
let b: f64 = 0.367;
let c = a.max(b);
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

### Anonymous Function
```
let count_one = |mut mask: usize| -> usize {
    let mut res = 0;
    while mask != 0 {
        res += mask % 2;
        mask /= 2;
    }
    res
};
```

```
let manhattan_dist = |worker_idx: usize, bike_idx: usize| -> i32 {
    return 
        (workers[worker_idx][0] - bikes[bike_idx][0]).abs() + 
        (workers[worker_idx][1] - bikes[bike_idx][1]).abs();
};
```

### Conversion
#### Convert char to digit
```
let ch = '1';
let digit = ch as i32 - '0' as i32;
```

```
let ch = '1';
let digit = ch.to_digit(10).unwrap() as i32;
```

#### Convert digit to char
```
let digit = 1;
let ch = char::from_digit(digit as u32, 10).unwrap();
```
