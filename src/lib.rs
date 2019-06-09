use std::fs::File;
use std::mem::size_of;
use std::io::Seek;
use std::io::SeekFrom;
use std::convert::TryInto;

use byteorder::{LittleEndian, ReadBytesExt};
use libc::{_SC_PAGESIZE, sysconf};

#[derive(Debug)]
pub enum Error {
    PageMap,
    Read,
    Unk,
}

pub fn virt_to_phys<T>(virt: *const T) -> Result<*const T, Error> {
    let ptr_val = virt as usize;
    let page_size = unsafe {
        sysconf(_SC_PAGESIZE) as usize
    };
    let mut file = File::open("/proc/self/pagemap").unwrap();
    let seek = (ptr_val / page_size * size_of::<*const T>()).try_into().unwrap();
    file.seek(SeekFrom::Start(seek)).unwrap();
    let entry = file.read_u64::<LittleEndian>().unwrap();

    Ok(((entry & 0x7fffffffffffffu64) * page_size as u64 + (ptr_val % page_size) as u64) as *const T)
}

#[cfg(test)]
mod tests {
    use crate::virt_to_phys;
    #[test]
    fn it_works() {
        let my_num: i32 = 10;
        let my_num_ptr: *const i32 = &my_num;
        println!("ptr virt: {:?} ptr phys: {:?}", my_num_ptr, virt_to_phys(my_num_ptr).unwrap());
    }
}
