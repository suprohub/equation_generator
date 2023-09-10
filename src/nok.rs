
fn nok(nums: &Vec<u32>, lim: u32) -> Option<u32> {
    let mut divider: u32 = 1;
    let mut divided: bool;
    loop {
        divider += 1;
        divided = true;
        for num in nums {
            if *num % divider != 0 {
                divided = false;
                break;
            }
        }
        if divided {
            return Some(divider);
        }
        if divider == lim {
            return None;
        }
    }
}