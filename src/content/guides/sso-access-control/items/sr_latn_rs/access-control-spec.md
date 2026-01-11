Definisanje načina na koji više korisnika međusobno interaguje i testiranje toga je komplikovano. Evo sledeće specifikacije koju primenjujemo za kontrolu pristupa,
koju možete koristiti za testiranje vaše implementacije:

    Page with null group ids, user with null group ids - treba da ima pristup.
    Page with null group ids, user with group ids - treba da ima pristup.
    Page with group ids, user with null group ids - treba da ima pristup.
    Page with group ids, user with empty list - NE bi trebalo da ima pristup.
    Page with group ids, user with group ids - postoji presek - treba da ima pristup.
    Page with group ids, user with group ids - presek ne postoji - NE bi trebalo da ima pristup.
    Page with empty list of group ids (nobody has access), user with null - NE bi trebalo da ima pristup.
    
    SSO User A = No group ids defined (null = puni pristup).
    SSO User B = No group ids defined (null = puni pristup).
    A može @B.
    
    SSO User A = No group ids defined (null = puni pristup).
    SSO User B = Group ids defined.
    A može @B.
    
    SSO User A = Group ids defined.
    SSO User B = No group ids defined (null = puni pristup).
    A može @B.
    
    SSO User A = Group ids = [a].
    SSO User B = Group ids = [b].
    A NE može @B.
    
    SSO User A = Group ids = [a].
    SSO User B = Group ids = [a, b].
    A može @B.