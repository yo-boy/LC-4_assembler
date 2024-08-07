	;; program to print hello world
	.ORIG x3000	      	;set origin

	LDA R0, MESSAGE
	PUTS
	
	HALT

MESSAGE	.STRINGZ "hello world"
	
	.END

	
