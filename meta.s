	ADR PROGRAM 
OUT1 
	TST '*1' 
	BF  A0 
	CL  'GN1' 
	OUT 
A0 
	BT  A1 
	TST '*2' 
	BF  A2 
	CL  'GN2' 
	OUT 
A2 
	BT  A1 
	TST '*' 
	BF  A3 
	CL  'CI' 
	OUT 
A3 
	BT  A1 
	SR 
	BF  A4 
	CL  'CL ' 
	CI 
	OUT 
A4 
A1 
	R 
OUTPUT 
	TST '.OUT' 
	BF  A5 
	TST '(' 
	BE 
A6 
	CLL OUT1 
	BT  A6 
	SET 
	BE 
	TST ')' 
	BE 
A5 
	BT  A7 
	TST '.LABEL' 
	BF  A8 
	CL  'LB' 
	OUT 
	CLL OUT1 
	BE 
A8 
A7 
	BF  A9 
	CL  'OUT' 
	OUT 
A9 
A10 
	R 
EX3 
	ID 
	BF  A11 
	CL  'CLL' 
	CI 
	OUT 
A11 
	BT  A12 
	SR 
	BF  A13 
	CL  'TST' 
	CI 
	OUT 
A13 
	BT  A12 
	TST '.ID' 
	BF  A14 
	CL  'ID' 
	OUT 
A14 
	BT  A12 
	TST '.NUMBER' 
	BF  A15 
	CL  'NUM' 
	OUT 
A15 
	BT  A12 
	TST '.STRING' 
	BF  A16 
	CL  'SR' 
	OUT 
A16 
	BT  A12 
	TST '(' 
	BF  A17 
	CLL EX1 
	BE 
	TST ')' 
	BE 
A17 
	BT  A12 
	TST '.EMPTY' 
	BF  A18 
	CL  'SET' 
	OUT 
A18 
	BT  A12 
	TST '$' 
	BF  A19 
	LB 
	GN1 
	OUT 
	CLL EX3 
	BE 
	CL  'BT ' 
	GN1 
	OUT 
	CL  'SET' 
	OUT 
A19 
A12 
	R 
EX2 
	CLL EX3 
	BF  A20 
	CL  'BF ' 
	GN1 
	OUT 
A20 
	BT  A21 
	CLL OUTPUT 
	BF  A22 
A22 
A21 
	BF  A23 
A24 
	CLL EX3 
	BF  A25 
	CL  'BE' 
	OUT 
A25 
	BT  A26 
	CLL OUTPUT 
	BF  A27 
A27 
A26 
	BT  A24 
	SET 
	BE 
	LB 
	GN1 
	OUT 
A23 
A28 
	R 
EX1 
	CLL EX2 
	BF  A29 
A30 
	TST '/' 
	BF  A31 
	CL  'BT ' 
	GN1 
	OUT 
	CLL EX2 
	BE 
A31 
A32 
	BT  A30 
	SET 
	BE 
	LB 
	GN1 
	OUT 
A29 
A33 
	R 
ST 
	ID 
	BF  A34 
	LB 
	CI 
	OUT 
	TST '=' 
	BE 
	CLL EX1 
	BE 
	TST '.,' 
	BE 
	CL  'R' 
	OUT 
A34 
A35 
	R 
PROGRAM 
	TST '.SYNTAX' 
	BF  A36 
	ID 
	BE 
	CL  'ADR' 
	CI 
	OUT 
A37 
	CLL ST 
	BT  A37 
	SET 
	BE 
	TST '.END' 
	BE 
	CL  'END' 
	OUT 
A36 
A38 
	R 
	END 
