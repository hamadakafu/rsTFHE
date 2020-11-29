use std::ffi::c_void;

extern "C" {
    /// void *new_fft_table(int32_t nn);
    pub fn new_fft_table(nn: i32) -> *mut c_void;
    /// double *fft_table_get_buffer(const void *tables);
    pub fn fft_table_get_buffer(tables: *const c_void) -> *mut f64;
    /// void *new_ifft_table(int32_t nn);
    pub fn new_ifft_table(nn: i32) -> *mut c_void;
    /// double *ifft_table_get_buffer(const void *tables);
    pub fn ifft_table_get_buffer(tables: *const c_void) -> *mut f64;
    /// void fft_model(const void *tables);
    pub fn fft_model(tables: *const c_void);
    /// void ifft_model(void *tables);
    pub fn ifft_model(tables: *const c_void);
    /// void fft(const void *tables, double *data);
    pub fn fft(tables: *const c_void, data: *mut f64);
    /// void ifft(const void *tables, double *data);
    pub fn ifft(tables: *const c_void, data: *mut f64);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spqlios_fft_ifft() {
        let nn = 32;
        unsafe {
            let tables = new_fft_table(nn);
            let itables = new_ifft_table(nn);
            let buf_fft = fft_table_get_buffer(tables);
            let buf_ifft = ifft_table_get_buffer(itables);
            let mut a = Vec::with_capacity(nn as usize);
            let mut a2 = Vec::with_capacity(nn as usize);
            let mut b = Vec::with_capacity(nn as usize);
            for i in 0..nn {
                a.push(i as f64);
                a2.push(i as f64);
                b.push(i as f64);
            }

            println!("before fft");

            for i in 0..nn {
                let p = buf_fft.offset(i as isize);
                *p = a[i as usize];
            }
            fft_model(tables);
            for i in 0..nn {
                let p = buf_fft.offset(i as isize);
                a[i as usize] = *p;
            }

            for i in 0..nn {
                let p = buf_fft.offset(i as isize);
                *p = a2[i as usize];
            }
            fft(tables, buf_fft);
            for i in 0..nn {
                let p = buf_fft.offset(i as isize);
                a2[i as usize] = *p;
            }

            println!("after fft");

            println!("before ifft");

            for i in 0..nn {
                let p = buf_ifft.offset(i as isize);
                *p = a[i as usize];
            }
            ifft_model(itables);
            for i in 0..nn {
                let p = buf_ifft.offset(i as isize);
                a[i as usize] = *p;
            }

            for i in 0..nn {
                let p = buf_ifft.offset(i as isize);
                *p = a2[i as usize];
            }
            ifft(itables, buf_ifft);
            for i in 0..nn {
                let p = buf_ifft.offset(i as isize);
                a2[i as usize] = *p;
            }

            println!("after ifft");
            for i in 0..nn {
                // println!("a: {}, a2: {}", a[i as usize], a2[i as usize]);
                debug_assert_eq!(
                    a[i as usize] as u32, a2[i as usize] as u32,
                    "test fft_model == fft"
                );
                debug_assert_eq!(
                    a[i as usize] as u32,
                    nn as u32 / 2 * b[i as usize] as u32,
                    "test fft_model == fft"
                );
            }
        }
    }
}
