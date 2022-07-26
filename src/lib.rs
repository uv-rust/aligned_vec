//-----------------------------------------------------------------------------
pub fn aligned_vec<T: Copy>(size: usize, capacity: usize, align: usize, touch: Option<T>) -> Vec<T> {
    unsafe {
        if size == 0 {
            Vec::<T>::new()
        } else {
            let size = size * std::mem::size_of::<T>();
            let capacity = (capacity * std::mem::size_of::<T>()).max(size);

            let layout = std::alloc::Layout::from_size_align_unchecked(size, align);
            let raw_ptr = std::alloc::alloc(layout) as *mut T;
            if let Some(x) = touch {
                let mut v = Vec::from_raw_parts(raw_ptr, size, capacity);
                for i in (0..size).step_by(page_size::get()) {
                    v[i] = x;
                }
                v
            } else {
                //SLOW!
                Vec::from_raw_parts(raw_ptr, size, capacity)
            }
        }
    }
}
//-----------------------------------------------------------------------------
pub fn init_aligned_vec<T: Copy>(size: usize, capacity: usize, align: usize, x: T) -> Vec<T> {
    unsafe {
        if size == 0 {
            Vec::<T>::new()
        } else {
            let size = size * std::mem::size_of::<T>();
            let capacity = (capacity * std::mem::size_of::<T>()).max(size);

            let layout = std::alloc::Layout::from_size_align_unchecked(size, align);
            let raw_ptr = std::alloc::alloc(layout) as *mut T;
            
            let mut v = Vec::from_raw_parts(raw_ptr, size, capacity);
            v.fill(x);
            v
        }
    }
}
//-----------------------------------------------------------------------------
pub fn page_aligned_vec<T: Copy>(size: usize, capacity: usize, touch: Option<T>, page_locked: bool) -> Vec<T> {
    let v = aligned_vec::<T>(size, capacity, page_size::get(), touch);
    if page_locked {
       unsafe {
           nix::sys::mman::mlock(v.as_ptr() as *const std::ffi::c_void, size).unwrap();
       }
    }
    v
}

//=============================================================================
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn page_aligned() {
        let ps = page_size::get();
        let len = 5 * ps;
        let capacity = 2 * len;
        let init_value = 42;
        let v = page_aligned_vec::<u8>(len, capacity, Some(init_value), false);
        assert_eq!(v.as_ptr() as usize % ps, 0);
        assert_eq!(v.len(), len);
        assert_eq!(v.capacity(), capacity);
        assert_eq!(v[ps], init_value);
    }
}
