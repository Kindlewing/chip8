package main

import "core:fmt"
import "core:math"
import "core:mem"
import "core:os"
import rl "vendor:raylib"


MEMORY_SIZE :: 4096
PROGRAM_START :: 0x200
DSP_W :: 64
DSP_H :: 32
PX_SIZE :: 16

TARGET_FPS :: 60
INSTR_PER_SEC :: 600
INSTR_PER_FRAME :: INSTR_PER_SEC / TARGET_FPS

Chip8 :: struct {
	mem:     [MEMORY_SIZE]byte,
	v_reg:   [16]u8,
	i_reg:   u16,
	pc:      u16,
	delay_t: u8,
	sound_t: u8,
	screen:  [DSP_W * DSP_H]u8,
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
	fd, fd_err := os.open("./data/rom/1-chip8-logo.ch8", os.O_RDONLY)
	if fd_err != nil {
		fmt.eprintf("Error: %v\n", fd_err)
		return
	}


	rom, ok := os.read_entire_file_from_handle(fd)
	if !ok {
		fmt.eprintf("Read error")
		return
	}

	copy_slice(chip8.mem[PROGRAM_START:], rom)

	chip8.pc = PROGRAM_START

	rl.SetTargetFPS(TARGET_FPS)
	rl.InitWindow(DSP_W * PX_SIZE, DSP_H * PX_SIZE, "Chip8")
	defer rl.CloseWindow()


	for !rl.WindowShouldClose() {
		instr: u16 =
			(u16(chip8.mem[chip8.pc]) << 8) | u16(chip8.mem[chip8.pc + 1])
		chip8.pc += 2

		op := (instr & 0xF000) >> 12
		x := (instr & 0x0F00) >> 8
		y := (instr & 0x00F0) >> 4
		n := (instr & 0x000F) >> 0
		kk := (instr & 0x00FF) >> 0
		nnn := (instr & 0x0FFF) >> 0

		rl.BeginDrawing()
		for i in 0 ..< INSTR_PER_FRAME {
			switch op {
			case 0x0:
				if nnn == 0x00E0 {
					// CLS
					fmt.printf("(00E0) CLS: Clear screen\n")
				}
			case 0x1:
				// JMP
				chip8.pc = nnn
				fmt.printf("(1NNN) JMP\n")
			case 0x6:
				// set reg
				chip8.v_reg[x] = u8(kk)
				fmt.printf("(6XKK) Set Vx\n")
			case 0x7:
				chip8.v_reg[x] += u8(kk)
				fmt.printf("(7XNN) Add NN to Vx\n")
			case 0xA:
				chip8.i_reg = nnn
			case 0xD:
				fmt.printf("(DXYN) DISPLAY\n")
			}
		}
		for x in 0 ..< DSP_W {
			for y in 0 ..< DSP_H {
				if cast(u8)chip8.screen[x + y * DSP_W] > 0 {
					rl.DrawRectangle(
						cast(i32)x * PX_SIZE,
						cast(i32)y * PX_SIZE,
						PX_SIZE,
						PX_SIZE,
						0,
					)
				}
			}
		}
		rl.EndDrawing()
	}
}
