	.ORIG x3000 		;starting address

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

	XOR R4, R4, R4
	ADDe R4, R4, #9

	LDa R0, LF
	out
	LDa R0, CR
	out

	XOR R0, R0, R0
	ADDe R0, R0, prompt
	puts			;print prompt
	XOR R0, R0, R0
	
loop	LDa R5, ascii0
	LDa R6, ascii1
	ADD R4, R4, #-1
	BRz byte
	getc
	out
	XOR R5, R5, R0
	BRz zero
	XOR R6, R6, R0
	BRz one
	BRnzp Exit

zero 	ADD R1, R1, R1	;shift the character being constructed left, it implicityly has a zero
	BRnzp loop
one	ADD R1, R1, R1	;shift the character being constructed left
	ADD R1, R1, #1	;add one to the character
	BRnzp loop

	
byte	LDa R0, LF
	out
	LDa R0, CR
	out

	XOR R0, R0, R0
	ADDe R0, R0, message
	puts		
	XOR R0, R0, R0
	LPN			;load stream cypher
	XOR R0, R0, R1		;decrypt ascii character
	ANDa R0, R0, mask	;take only lower 8 bits
	;; ADD R0, R1, #0
	out

	LDa R0, LF
	out
	LDa R0, CR
	out

	XOR R0, R0, R0
	ADDe R0, R0, prompt
	puts			;print prompt

	XOR R0, R0, R0
	XOR R1, R1, R1
	XOR R2, R2, R2
	XOR R4, R4, R4
	ADDe R4, R4, #9
	BRnzp loop
	
	
Exit	HALT

welcome	.STRINGZ "Welcome, the decryption key is hardcoded, input bytes as 0s and 1s left to right to decrypt, any other character to exit"

prompt .STRINGZ "Enter a byte: "
	
message .STRINGZ "Decrypted character: "

Esc	.FILL x1B
ascii0	.FILL x30
ascii1	.FILL x31

LF	.FILL x0A
CR	.FILL x0D	
	
mask	.FILL xFF
	
	.END
