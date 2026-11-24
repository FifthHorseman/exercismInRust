pub fn collatz(n: u64) -> Option<u64> {
    let mut count : u64 = 0;
    let mut curr_num : u64 = n;
    if(n ==0) {
        return None;
    }
    while (curr_num!=1) {
        count +=1;
        if (curr_num %2 == 0){
            curr_num /= 2;
        } else {
            curr_num *= 3;
            curr_num += 1;
        }
    }
    Some(count)
}
