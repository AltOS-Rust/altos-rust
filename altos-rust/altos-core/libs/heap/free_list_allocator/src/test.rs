/*
 * Copyright (C) 2017  AltOS-Rust Team
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

use std::vec::Vec;

use free_list;

// This produces a section of memory that we can use for testing by creating an array
// Returns the starting address of this memory
pub fn get_memory(heap_size: usize) -> *const u8 {
    let heap: Vec<u8> = vec![0; heap_size];
    &heap[0] as *const u8
}

// Gets an initialized free list with the requested memory for testing purposes
pub fn get_free_list_with_size(heap_size: usize) -> free_list::FreeList {
    let heap_start = get_memory(heap_size);

    let mut free_list = free_list::FreeList::new();
    free_list.init(heap_start as usize, heap_size);
    free_list
}
