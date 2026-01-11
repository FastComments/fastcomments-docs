Definir cómo interactúan múltiples usuarios y probarlo es complicado. A continuación está la especificación que seguimos para el control de acceso, que puede usar para probar su implementación:

    Página con IDs de grupo nulos, usuario con IDs de grupo nulos - debería tener acceso.
    Página con IDs de grupo nulos, usuario con IDs de grupo - debería tener acceso.
    Página con IDs de grupo, usuario con IDs de grupo nulos - debería tener acceso.
    Página con IDs de grupo, usuario con lista vacía - NO debería tener acceso.
    Página con IDs de grupo, usuario con IDs de grupo - existe intersección - debería tener acceso.
    Página con IDs de grupo, usuario con IDs de grupo - no existe intersección - NO debería tener acceso.
    Página con lista vacía de IDs de grupo (nadie tiene acceso), usuario con null - NO debería tener acceso.
    
    SSO User A = No se han definido IDs de grupo (null = acceso total).
    SSO User B = No se han definido IDs de grupo (null = acceso total).
    A puede @B.
    
    SSO User A = No se han definido IDs de grupo (null = acceso total).
    SSO User B = IDs de grupo definidos.
    A puede @B.
    
    SSO User A = IDs de grupo definidos.
    SSO User B = No se han definido IDs de grupo (null = acceso total).
    A puede @B.
    
    SSO User A = IDs de grupo = [a].
    SSO User B = IDs de grupo = [b].
    A NO puede @B.
    
    SSO User A = IDs de grupo = [a].
    SSO User B = IDs de grupo = [a, b].
    A puede @B.