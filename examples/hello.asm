	;; program to print the first 12 character of the alphabet
	.ORIG x3000	      	;set origin

	LD R1, #12		; load the loop counter into R1
	LD R0, x60		; load the hex value for a in R0
	
LOOP	ADD R0, R0, #1		; ADD one to the output variable indexing to the next character
	OUT			; print the output variable
	ADD R1, R1, #-1		; remove one from the loop variable
	BRp LOOP		; loop until R1 becomes zero or negative

	LDA R0, MESSAGE
	PUTS
	
	HALT

MESSAGE	.STRINGZ "hello"
	
	.END

	
