	.file	"hi.c"
	.text
	.globl	ret_1
	.type	ret_1, @function
ret_1:
;.LFB0:
;	.cfi_startproc
	pushq	%rbp
	;.cfi_def_cfa_offset 16
	;.cfi_offset 6, -16
	movq	%rsp, %rbp
	;.cfi_def_cfa_register 6
	movl	$1331, %eax
	popq	%rbp
	;.cfi_def_cfa 7, 8
	ret
;	.cfi_endproc
;.LFE0:
;	.size	ret_1, .-ret_1
;	.section	.rodata
.LC0:
	.string	"i'm saying hi !"
	.text
	.globl	say_hi
	.type	say_hi, @function
say_hi:
.LFB1:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register 6
	movl	$.LC0, %edi
	call	puts
	popq	%rbp
	.cfi_def_cfa 7, 8
	ret
	.cfi_endproc
.LFE1:
	.size	say_hi, .-say_hi
	.section	.rodata
.LC1:
	.string	"shit son %d\n"
	.text
	.globl	main
	.type	main, @function
main:
.LFB2:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register 6
	movl	$0, %eax
	call	say_hi
	movl	$0, %eax
	call	ret_1
	movl	%eax, %edx
	movl	$.LC1, %eax
	movl	%edx, %esi
	movq	%rax, %rdi
	movl	$0, %eax
	call	printf
	movl	$0, %eax
	popq	%rbp
	.cfi_def_cfa 7, 8
	ret
	.cfi_endproc
.LFE2:
	.size	main, .-main
	.ident	"GCC: (Ubuntu/Linaro 4.6.3-1ubuntu5) 4.6.3"
	.section	.note.GNU-stack,"",@progbits
