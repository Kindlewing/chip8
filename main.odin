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


Chip8 :: struct {
	mem:     [MEMORY_SIZE]byte,
	v_reg:   [16]u8,
	i_reg:   u16,
	pc:      int,
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

	chip8: Chip8
	fd, fd_err := os.open("data/rom/ibm_logo.ch8", os.O_RDONLY)
	if fd_err != nil {
		fmt.eprintf("Error: %v\n", fd_err)
		return
	}

	rom, ok := os.read_entire_file_from_handle(fd)
	if !ok {
		fmt.eprintf("Read error")
		return
	}

	fmt.printf("%v\n", rom)

	copy_slice(chip8.mem[PROGRAM_START:], rom)

	for {
		pc := chip8.pc
		ins_low: u8 = chip8.mem[pc]
		ins_hi: u8 = chip8.mem[pc + 1]
		chip8.pc += 2

		instr: u16 = (u16(ins_low) << 8) | u16(ins_hi)
		fmt.printf("instr: %u\n", instr)
	}
}
