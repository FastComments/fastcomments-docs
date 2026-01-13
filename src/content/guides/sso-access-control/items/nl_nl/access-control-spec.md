---
Het definiëren van hoe meerdere gebruikers met elkaar omgaan, en het testen daarvan, is gecompliceerd. Hier is de volgende specificatie die wij hanteren voor toegangscontrole,
die je kunt gebruiken om je implementatie te testen:

    Pagina met null group ids, gebruiker met null group ids - moet toegang hebben.
    Pagina met null group ids, gebruiker met group ids - moet toegang hebben.
    Pagina met group ids, gebruiker met null group ids - moet toegang hebben.
    Pagina met group ids, gebruiker met empty list - moet GEEN toegang hebben.
    Pagina met group ids, gebruiker met group ids - intersection exists - moet toegang hebben.
    Pagina met group ids, gebruiker met group ids - intersection does not exist - moet GEEN toegang hebben.
    Pagina met empty list of group ids (niemand heeft toegang), gebruiker met null - moet GEEN toegang hebben.
    
    SSO User A = Geen group ids gedefinieerd (null = volledige toegang).
    SSO User B = Geen group ids gedefinieerd (null = volledige toegang).
    A kan @B.
    
    SSO User A = Geen group ids gedefinieerd (null = volledige toegang).
    SSO User B = Group ids gedefinieerd.
    A kan @B.
    
    SSO User A = Group ids gedefinieerd.
    SSO User B = Geen group ids gedefinieerd (null = volledige toegang).
    A kan @B.
    
    SSO User A = Group ids = [a].
    SSO User B = Group ids = [b].
    A kan NIET @B.
    
    SSO User A = Group ids = [a].
    SSO User B = Group ids = [a, b].
    A kan @B.
---