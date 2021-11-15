ARR1		DCD		10, -8, 47, 81, 4, -6, -11, 7, 30, 11
DIM 		DCD 	10

			MOV 	R0, #ARR1
			MOV 	R1, #DIM
			LDR 	R1, [R1]

			MOV 	R4, R0
			MOV 	R5, R1
			MOV 	R6, #0

CICLO1		CMP 	R1, #1
			BEQ 	NEXTSTEP
			LDR 	R2, [R0]
			LDR 	R3, [R0, #4]
			CMP 	R2, R3
			BLE 	NOSWITCH
			STR 	R3, [R0]
			STR 	R2, [R0, #4]
			ADD 	R6, R6, #1

NOSWITCH	ADD 	R0, R0, #4
			SUB 	R1, R1, #1
			B 		CICLO1



NEXTSTEP