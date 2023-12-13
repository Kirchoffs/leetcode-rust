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

#### Sort Vec of ints
```
let mut arr = vec![3, 2, 1];
arr.sort();
```

#### Convert String to Vec<char>
```
let chs = s.chars().collect::<Vec<char>>();
```

```
let chs: Vec<char> = s.chars().collect();
```

### Compare
#### Compare value of two int
```
let a = 4;
let b = 2;
let c = std::cmp::max(a, b);
```

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

### Anonymous Function / Lambda Function
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

### Constant
- Max value of i32: `std::i32::MAX`

### Priority Queue
```
let mut heap = BinaryHeap::new();
assert_eq!(heap.peek(), None);

heap.push(1);
heap.push(5);
heap.push(2);
assert_eq!(heap.peek(), Some(&5));
assert_eq!(heap.len(), 3);
for x in &heap {
    println!("{}", x);
}

assert_eq!(heap.pop(), Some(5));
assert_eq!(heap.pop(), Some(2));
assert_eq!(heap.pop(), Some(1));
assert_eq!(heap.pop(), None);

heap.clear();

assert!(heap.is_empty());
```

```
let mut heap: BinaryHeap<(i32, i32)> = BinaryHeap::new();
assert_eq!(heap.peek(), None);

heap.push((2, 3));
heap.push((2, 5));
heap.push((3, 5));

let x = heap.pop();
println!("{:?}", x.unwrap());

let x = heap.pop();
println!("{:?}", x);

let x = heap.pop();
println!("{:?}", x);
```

### Math
#### Power
```
let x = 4_i64.pow(2);
```
