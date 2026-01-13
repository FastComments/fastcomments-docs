Definir como vários usuários interagem e testá-lo é complicado. Aqui está a especificação que seguimos para controle de acesso,
que você pode usar para testar sua implementação:

    Página com IDs de grupo null, usuário com IDs de grupo null - deve ter acesso.
    Página com IDs de grupo null, usuário com IDs de grupo definidos - deve ter acesso.
    Página com IDs de grupo, usuário com IDs de grupo null - deve ter acesso.
    Página com IDs de grupo, usuário com lista vazia - NÃO deve ter acesso.
    Página com IDs de grupo, usuário com IDs de grupo - interseção existe - deve ter acesso.
    Página com IDs de grupo, usuário com IDs de grupo - interseção não existe - NÃO deve ter acesso.
    Página com lista vazia de IDs de grupo (ninguém tem acesso), usuário com null - NÃO deve ter acesso.
    
    SSO User A = Nenhum ID de grupo definido (null = acesso total).
    SSO User B = Nenhum ID de grupo definido (null = acesso total).
    A pode @B.
    
    SSO User A = Nenhum ID de grupo definido (null = acesso total).
    SSO User B = IDs de grupo definidos.
    A pode @B.
    
    SSO User A = IDs de grupo definidos.
    SSO User B = Nenhum ID de grupo definido (null = acesso total).
    A pode @B.
    
    SSO User A = IDs de grupo = [a].
    SSO User B = IDs de grupo = [b].
    A NÃO pode @B.
    
    SSO User A = IDs de grupo = [a].
    SSO User B = IDs de grupo = [a, b].
    A pode @B.