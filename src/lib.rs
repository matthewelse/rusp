
extern crate num;
use num::complex::Complex;
use std::f32::consts::PI;

pub fn separate(values: &Vec<Complex<f32>>) -> (Vec<Complex<f32>>, Vec<Complex<f32>>) {
    // Split odd-indexed values into the left result, and even-indexed values into the right
    // result.
    let split_len = values.len() / 2;

    values
        .iter()
        .fold((Vec::with_capacity(split_len), Vec::with_capacity(split_len)),
              |acc, &x| {
                  let (mut l, r) = acc;
                  l.push(x);
                  (r, l)
              })
}

pub fn fft(values: Vec<Complex<f32>>) -> Vec<Complex<f32>> {
    println!("{:?}", values.len());
    if values.len() == 2 {
        values
    } else {
        let (l, r) = separate(&values);

        let lfft = fft(l);
        let rfft = fft(r);

        let (mut lresult, rresult) : (Vec<_>, Vec<_>) = lfft.iter()
            .zip(rfft.iter())
            .enumerate()
            .map(|elem| {
                     let (k, x) = elem;
                     let (l, r) = x;
                     let w = Complex::new(0.0, -2.0 * PI * (k as f32) / (values.len() as f32)).exp();

                     (l + w * r, l - w * r)
                 })
            .unzip();

        lresult.extend(rresult);
        lresult
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn separate_works() {
        let test_in = (0u32..32)
            .map(|x| x as f32)
            .map(|x| Complex::new(x, x))
            .collect::<Vec<_>>();

        let (l, r) = separate(&test_in);

        assert!(l.len() == 16);
        assert!(r.len() == 16);

        for i in 0..(test_in.len()) {
            if i % 2 == 0 {
                assert_eq!(test_in[i], l[i / 2]);
            } else {
                assert_eq!(test_in[i], r[i / 2]);
            }
        }
    }

    #[test]
    fn fft_works() {
        let val_in = (0u32..32)
            .map(|x| x as f32)
            .map(|x| Complex::new(x, x))
            .collect::<Vec<_>>();
        let val_out = fft(val_in);
        
        // println!("{:?}", val_out);
        //assert_eq!(val_out[0], Complex::new(0.0, 0.0));
    }
}
