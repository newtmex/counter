#![cfg_attr(not(feature = "std"), no_std, no_main)]
extern crate alloc;

mod counter;

#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

