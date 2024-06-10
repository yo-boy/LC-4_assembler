	;; program to print the first 12 character of the alphabet
	.ORIG x3000	      	;set origin

	LDA R0, MESSAGE
	PUTS
	
	HALT

MESSAGE	.STRINGZ "hello world"
	
	.END

	
