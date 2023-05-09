
fn median(d: &Vec<f32>) -> Option<f32> {
    if d.is_empty() == false {
        let mut d1 = d.clone();
        d1.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mid_point: usize = d1.len() / 2;

        if d1.len() % 2 == 0 {
            // Even
            let median_value: f32 = (d1[mid_point] + d1[mid_point - 1]) / 2.0;

            return Some(median_value);
        } else {
            // Odd
            return Some(d1[mid_point]);
        }
    }

    None
}

fn main() {
    let data: Vec<f32> = vec![0.0, 1.0];

    println!("Median = {}", median(&data).expect("None") );
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_empty() {
        let data: Vec<f32> = Vec::new();

        assert_eq!(median(&data),None);
    }

    #[test]
    fn test_odd_number_elements() {
        let data: Vec<f32> = vec![6.0, 4.0, 9.0];

        assert_eq!(median(&data), Some(6.0));
    }

    #[test]
    fn test_even_number_elements() {
        let data: Vec<f32> = vec![8.0, -1.0, 4.0, 23.0, 12.0, 5.0];

        assert_eq!(median(&data), Some(6.5));
    }

}
