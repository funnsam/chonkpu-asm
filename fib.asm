norira r15 0xF

norira r1 0xD
norira r2 0xF
norira r4 0xB
addi r4 3

// a = a + b
add r1 r2

// b = a - b
norira r3 0xF
add r3 r2
nor r3 r0
addi r3 1
norira r2 0xF
add r2 r1

bnsr r1 enter_loop
add r2 r3
stwr r1 1

halt:
	jmpr halt
	add r0 r0
	add r0 r0

enter_loop:
	jmpa r4
	add r0 r0
	add r0 r0
