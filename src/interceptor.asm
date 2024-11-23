.data
EXTERN g_map_address: qword
EXTERN g_map_back_address: qword
EXTERN g_pos_address: qword
EXTERN g_pos_back_address: qword
EXTERN g_pause_address: qword
EXTERN g_pause_back_address: qword

.code
asm_map_hook PROC
    mov [g_map_address], rsi
    jmp [g_map_back_address]
asm_map_hook ENDP

asm_pos_hook PROC
    mov [g_pos_address], rax
    jmp [g_pos_back_address]
asm_pos_hook ENDP

asm_pause_hook PROC
    mov [g_pause_address], rax
    jmp [g_pause_back_address]
asm_pause_hook ENDP

END