Definire come più utenti interagiscono e testarlo è complicato. Ecco la seguente specifica che seguiamo per il controllo degli accessi, che puoi usare per testare la tua implementazione:

    Pagina con null group ids, utente con null group ids - dovrebbe avere accesso.
    Pagina con null group ids, utente con group ids - dovrebbe avere accesso.
    Pagina con group ids, utente con null group ids - dovrebbe avere accesso.
    Pagina con group ids, utente con empty list - NON dovrebbe avere accesso.
    Pagina con group ids, utente con group ids - esiste intersezione - dovrebbe avere accesso.
    Pagina con group ids, utente con group ids - non esiste intersezione - NON dovrebbe avere accesso.
    Pagina con empty list di group ids (nessuno ha accesso), utente con null - NON dovrebbe avere accesso.
    
    SSO User A = No group ids defined (null = full access).
    SSO User B = No group ids defined (null = full access).
    A può @B.
    
    SSO User A = No group ids defined (null = full access).
    SSO User B = Group ids defined.
    A può @B.
    
    SSO User A = Group ids defined.
    SSO User B = No group ids defined (null = full access).
    A può @B.
    
    SSO User A = Group ids = [a].
    SSO User B = Group ids = [b].
    A NON può @B.
    
    SSO User A = Group ids = [a].
    SSO User B = Group ids = [a, b].
    A può @B.

---