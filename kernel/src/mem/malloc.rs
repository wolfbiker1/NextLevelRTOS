const JOURNAL_BASE: u32 = 0x2000_0000;
const JOURNAL_NUM_OF_ELEMENTS: u32 = JOURNAL_BASE;
const JOURNAL_START: u32 = JOURNAL_BASE + 0x04;

const MEM_BLOCK_START: u32 = 0x2000_0048;
const ADDR_OF_HIGHEST_FREE_BLOCK: u32 = MEM_BLOCK_START + 0x04;
const WORD: u32 = 0x4;
use core::intrinsics::{volatile_load, volatile_store};
use core::ptr::swap;
pub unsafe fn init() {
    volatile_store(MEM_BLOCK_START as *mut u32, ADDR_OF_HIGHEST_FREE_BLOCK);
    volatile_store(JOURNAL_NUM_OF_ELEMENTS as *mut u32, 0x0000_0000);
}

pub unsafe fn get_mem(mut requested_size: u32) -> u32 {

    // let m = volatile_load(MEM_BLOCK_START as *const u32);
    while requested_size % 4 != 0 {
        requested_size += 1
    }
    // if m >= 0x20000300 {
    //     // let x = 3;
    //     return 0;
    // }
    let mut no_gap_found = true;
    if volatile_load(JOURNAL_NUM_OF_ELEMENTS as *const u32) != 0 {
        // first fit algorithm
        let entries = volatile_load(JOURNAL_NUM_OF_ELEMENTS as *const u32);
        // if entries > 15 {
        //     let b = 12;
        // }
        for journal_entry in 0..entries {
            let journal_entry_addr = JOURNAL_START + journal_entry * WORD;
            let free_entry = volatile_load(journal_entry_addr as *const u32);
            // filter dirty bit?
            let size_available = volatile_load(free_entry as *const u32);
            if requested_size <= size_available {
                volatile_store(journal_entry_addr as *mut u32, 0x0000_0000);

                let journal_size = volatile_load(JOURNAL_NUM_OF_ELEMENTS as *const u32);
                let end_adress = JOURNAL_BASE + journal_size * WORD;

                if journal_size == 1 {
                    volatile_store(journal_entry_addr as *mut u32, 0x0000_0000);
                } else {
                    swap(end_adress as *mut u32, journal_entry_addr as *mut u32);
                }
                volatile_store(
                    JOURNAL_NUM_OF_ELEMENTS as *mut u32,
                    volatile_load(JOURNAL_NUM_OF_ELEMENTS as *const u32) - 1,
                );
                volatile_store(free_entry as *mut u32, requested_size | 0x8000_0000);
                return free_entry + WORD;
            }
        }
        no_gap_found = true;
    }

    if volatile_load(JOURNAL_NUM_OF_ELEMENTS as *const u32) == 0 || no_gap_found {
        // CRASH
        let meta_of_new_chunk = volatile_load(MEM_BLOCK_START as *const u32);
        // let ahf = core::ptr::read_volatile(MEM_BLOCK_START as *const u32);

        // first 4 byte containing size of the chunk
        let chunk_start = meta_of_new_chunk + WORD;

        // 0x8000_0000 ...set dirty bit
        volatile_store(meta_of_new_chunk as *mut u32, requested_size | 0x8000_0000);

        // save adress of next free region 
        volatile_store(MEM_BLOCK_START as *mut u32, chunk_start + requested_size);
        return chunk_start;
    }
    0
}

pub fn free(addr: u32) {
    unsafe {
        // on free, one more gap arises
        let num_of_journal_entries = volatile_load(JOURNAL_NUM_OF_ELEMENTS as *mut u32);

        // stores size & dirty bit
        let mut chunk_meta = volatile_load((addr - WORD) as *mut u32);
        chunk_meta &= !(0x8000_0000);

        volatile_store(
            JOURNAL_NUM_OF_ELEMENTS as *mut u32,
            num_of_journal_entries + 1,
        );
        volatile_store(
            (JOURNAL_START + num_of_journal_entries * WORD) as *mut u32,
            addr - WORD,
        );
        let a: u32 = 0;
        volatile_store(
            (addr - WORD) as *mut u32,
            chunk_meta,
        );
        let b: u32 = 0;
    }
}
