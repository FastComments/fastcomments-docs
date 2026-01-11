Opredelitev, kako več uporabnikov medsebojno deluje, in testiranje tega je zapleteno. Tukaj je naslednja specifikacija, ki jo upoštevamo za nadzor dostopa, ki jo lahko uporabite za testiranje vaše implementacije:

    Page with null group ids, user with null group ids - bi smel imeti dostop.
    Page with null group ids, user with group ids - bi smel imeti dostop.
    Page with group ids, user with null group ids - bi smel imeti dostop.
    Page with group ids, user with empty list - NE bi smel imeti dostopa.
    Page with group ids, user with group ids - intersection exists - bi smel imeti dostop.
    Page with group ids, user with group ids - intersection does not exist - NE bi smel imeti dostopa.
    Page with empty list of group ids (nihče nima dostopa), user with null - NE bi smel imeti dostopa.
    
    SSO User A = Ni definiranih group ids (null = polni dostop).
    SSO User B = Ni definiranih group ids (null = polni dostop).
    A lahko @B.
    
    SSO User A = Ni definiranih group ids (null = polni dostop).
    SSO User B = Group ids definirani.
    A lahko @B.
    
    SSO User A = Group ids definirani.
    SSO User B = Ni definiranih group ids (null = polni dostop).
    A lahko @B.
    
    SSO User A = Group ids = [a].
    SSO User B = Group ids = [b].
    A NE more @B.
    
    SSO User A = Group ids = [a].
    SSO User B = Group ids = [a, b].
    A lahko @B.