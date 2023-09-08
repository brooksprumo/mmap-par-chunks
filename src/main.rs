use bytemuck;
use memmap2::MmapMut;
use rand::Rng;
use rayon::prelude::*;

fn main() {
    type Item = u32;
    const ITEM_SIZE: usize = std::mem::size_of::<Item>();

    // create a file-backed buffer
    let mut mmap = MmapMut::map_anon(10 * ITEM_SIZE).unwrap();
    println!("initial: {:02x?}", mmap.as_ref());

    // get a mutable slice to the buffer, but as the underlying Item type
    let buffer: &mut [Item] = bytemuck::cast_slice_mut(&mut mmap);
    println!("buffer: {:?}", buffer.as_ptr_range());

    // fill it in parallel chunks with random values
    buffer.par_chunks_mut(3).for_each(|chunk| {
        let mut rng = rand::thread_rng();
        rng.fill(chunk);
        println!(
            "chunk len {}, {:?}: {:08x?}",
            chunk.len(),
            chunk.as_ptr_range(),
            chunk
        );
    });
    println!("filled: {:02x?}", mmap.as_ref());

    // sum it all up - just to do something using the original byte view
    let sum: u64 = mmap.par_iter().map(|x| *x as u64).sum();

    println!("sum: {sum}");
}
