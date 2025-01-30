package main

import "core:fmt"
import "core:math"
import "core:mem"
import "core:os"
import rl "vendor:raylib"


MEMORY_SIZE :: 4096
PROGRAM_START :: 0x200
DSP_X :: 64
DSP_Y :: 32
SPRITE_X :: 8
SPRITE_Y_MIN :: 1
SPRITE_Y_MAX :: 15


chip8 :: struct {
	mem:     [MEMORY_SIZE]byte,
	v_reg:   [16]u8,
	i_reg:   u16,
	delay_t: u8,
	sound_t: u8,
}

main :: proc() {
	when ODIN_DEBUG {
		track: mem.Tracking_Allocator
		mem.tracking_allocator_init(&track, context.allocator)
		context.allocator = mem.tracking_allocator(&track)

		defer {
			if len(track.allocation_map) > 0 {
				fmt.eprintf(
					"=== %v allocations not freed: ===\n",
					len(track.allocation_map),
				)
				for _, entry in track.allocation_map {
					fmt.eprintf(
						"- %v bytes @ %v\n",
						entry.size,
						entry.location,
					)
				}
			}
			if len(track.bad_free_array) > 0 {
				fmt.eprintf(
					"=== %v incorrect frees: ===\n",
					len(track.bad_free_array),
				)
				for entry in track.bad_free_array {
					fmt.eprintf("- %p @ %v\n", entry.memory, entry.location)
				}
			}
			mem.tracking_allocator_destroy(&track)
		}
	}
}
