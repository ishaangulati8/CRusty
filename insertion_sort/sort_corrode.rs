#[no_mangle]
pub unsafe extern fn insertion_sort(n : i32, p : *mut i32) {
    let mut i : i32 = 1i32;
    'loop1: loop {
        if !(i < n) {
            break;
        }
        let tmp : i32 = *p.offset(i as (isize));
        let mut j : i32 = i;
        'loop4: loop {
            if !(j > 0i32 && (*p.offset((j - 1i32) as (isize)) > tmp)) {
                break;
            }
            *p.offset(j as (isize)) = *p.offset((j - 1i32) as (isize));
            j = j - 1;
        }
        *p.offset(j as (isize)) = tmp;
        i = i + 1;
    }
}
