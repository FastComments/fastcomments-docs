Określanie, jak wielu użytkowników wchodzi w interakcję oraz testowanie tego, jest skomplikowane. Oto specyfikacja, której przestrzegamy w kontroli dostępu, której możesz użyć do przetestowania swojej implementacji:

    Strona z null group ids, użytkownik z null group ids - powinien mieć dostęp.
    Strona z null group ids, użytkownik z group ids - powinien mieć dostęp.
    Strona z group ids, użytkownik z null group ids - powinien mieć dostęp.
    Strona z group ids, użytkownik z pustą listą - NIE powinien mieć dostępu.
    Strona z group ids, użytkownik z group ids - istnieje przecięcie - powinien mieć dostęp.
    Strona z group ids, użytkownik z group ids - przecięcie nie istnieje - NIE powinien mieć dostępu.
    Strona z pustą listą group ids (nikt nie ma dostępu), użytkownik z null - NIE powinien mieć dostępu.
    
    SSO User A = No group ids defined (null = pełny dostęp).
    SSO User B = No group ids defined (null = pełny dostęp).
    A może @B.
    
    SSO User A = No group ids defined (null = pełny dostęp).
    SSO User B = Group ids defined.
    A może @B.
    
    SSO User A = Group ids defined.
    SSO User B = No group ids defined (null = pełny dostęp).
    A może @B.
    
    SSO User A = Group ids = [a].
    SSO User B = Group ids = [b].
    A NIE może @B.
    
    SSO User A = Group ids = [a].
    SSO User B = Group ids = [a, b].
    A może @B.