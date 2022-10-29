//https://www.codewars.com/kata/526989a41034285187000de4/rust
//kyu 5

//My first solution
fn get_ip_value(ip: &str) -> u32 {
    let int_ip: Vec<u32> = ip
        .split(".")
        .into_iter()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut final_val: u32 = 0;
    for val in int_ip.into_iter().rev().enumerate() {
        final_val += val.1 * 256u32.pow(val.0 as u32);
    }
    final_val
}

//optimized
fn get_ip_value_2(ip: &str) -> u32 {
    let int_ip: u32 = ip
        .rsplit(".")
        .map(|x| x.parse().unwrap())
        .enumerate()
        .fold(0, |final_val: u32, val: (usize, u32)| {
            final_val + (val.1) * 256u32.pow(val.0 as u32)
        });

    int_ip
}

//favorite solution with bit operations
fn get_ip_value_3(ip: &str) -> u32 {
    ip.split('.')
        .map(|s| s.parse::<u8>().unwrap())
        .fold(0, |acc, byte| (acc << 8) + byte as u32)
}

fn ips_between(start: &str, end: &str) -> u32 {
    get_ip_value_3(end) - get_ip_value_2(start)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(ips_between("10.0.0.0", "10.0.0.50"), 50);
        assert_eq!(ips_between("20.0.0.10", "20.0.1.0"), 246);
    }
}
