	;; program to print numbers from 0 to 12
	.ORIG x3000	      	;set origin

	LD R1, #12		; load the loop counter into R1


LOOP	ADD R0, R0, #1		; ADD one to the output variable
	PUTS			; print the output
	.FILL xFFFF
	ADDe R1, R2, x-fff
	ADD R1, R1, #-4		; remove one from the loop variable
	BRp LOOP		; loop until R1 becomes zero or negative
	
	HALT

	
	
NUMBER	.FILL x12

MESSAGE	.STRINGZ "hello"
	
	.END

	
