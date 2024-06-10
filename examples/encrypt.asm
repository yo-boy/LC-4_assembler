.orig x3000

	ADDe R0, R0, welcome
	puts
	XOR R0, R0, R0
	
	ADDe R0, R0, x7E30
	ADDe R1, R1, x6750
	ADDe R2, R2, x7C55
	LSD		;load key as seed
	XOR R0, R0, R0
	XOR R1, R1, R1
	XOR R2, R2, R2	;clean up
	
;;; main input loop
	LDa R6, Esc	;load escape to check if exit
loop	in
	XOR R4, R6, R0	;check if user pressed escape
	BRz exit	;exit
	XOR R1, R1, R1	;zero R1
	ADD R1, R0, #0	;move R0 to R1
	LPN		;load stream cypher into R0
	XOR R2, R1, R0	;encrypt character
	ANDa R2, R2, lower8
	
	XOR R3, R3, R3
	ADDe R3, R3, #8 ;load variable for loop	

print	ANDa R4, R2, mask1	;mask the eigth bit
	BRz zero		;jump to zero if zero
	LDa R0, oval 		;one
	out
	BRnzp next		;go past zero

zero	LDa R0, zval		;zero
	out

next	ADD R2, R2, R2 		;advance by one bit
	ADD R3, R3, #-1		;decrement loop
	BRp print		;repeat 8 times until all of the number is printed
	
	BRnzp loop

exit	XOR R0, R0, R0
	ADDe R0, R0, goodbye
	puts
	HALT

welcome	.STRINGZ "Welcome, the encryption key is hardcoded, input characters to encrypt, ESC to exit"
goodbye .STRINGZ "GGoodbye."
	
Esc	.FILL x1B
zval	.FILL x30
oval	.FILL x31

LF	.FILL x0A
CR	.FILL x0D
	
lower8	.FILL xFF
mask1	.FILL x80
mask0	.FILL x0

	


	;;; new line
NL	LDa R0, LF
	out
	LDa R0, CR
	out
	Ret
	
.END
