Definiranje načina kako više korisnika međusobno djeluje i njegovo testiranje je komplicirano. Ovo je sljedeća specifikacija koju slijedimo za kontrolu pristupa,
koju možete koristiti za testiranje vaše implementacije:

    Stranica s null group ids, korisnik s null group ids - treba imati pristup.
    Stranica s null group ids, korisnik s group ids - treba imati pristup.
    Stranica s group ids, korisnik s null group ids - treba imati pristup.
    Stranica s group ids, korisnik s praznom listom - ne bi trebao imati pristup.
    Stranica s group ids, korisnik s group ids - presjek postoji - treba imati pristup.
    Stranica s group ids, korisnik s group ids - presjek ne postoji - ne bi trebao imati pristup.
    Stranica s praznom listom group ids (nitko nema pristup), korisnik s null - ne bi trebao imati pristup.
    
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
    A ne može @B.
    
    SSO User A = Group ids = [a].
    SSO User B = Group ids = [a, b].
    A može @B.