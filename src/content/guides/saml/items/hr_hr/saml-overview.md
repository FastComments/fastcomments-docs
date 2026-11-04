SAML (Security Assertion Markup Language) je XML-bazirani otvoreni standard za razmjenu podataka o autentifikaciji i autorizaciji između strana, 
posebno između davatelja identiteta (IdP) i davatelja usluga (SP).

### Kako SAML funkcionira

SAML omogućuje jedinstvenu prijavu (SSO) tako da korisnici mogu jednom autentificirati kod svog davatelja identiteta, a zatim pristupiti više aplikacija 
bez ponovnog unosa vjerodajnica. Kada korisnik pokuša pristupiti FastComments:

1. **Zahtjev za autentifikacijom**: FastComments preusmjerava korisnika na vašeg davatelja identiteta
2. **Autentifikacija korisnika**: Korisnik se autentificira kod vašeg IdP-a (npr. Active Directory, Okta, Azure AD)
3. **SAML odgovor**: IdP šalje potpisani SAML assertion natrag u FastComments
4. **Pristup korisnika**: FastComments provjerava assertion i odobrava pristup autentificiranom korisniku

### Prednosti SAML-a

- **Poboljšana sigurnost**: Centralizirana autentifikacija smanjuje sigurnosne rizike povezane s lozinkama
- **Poboljšano korisničko iskustvo**: Korisnici se prijave jednom i neprimjetno pristupaju više aplikacija
- **Usklađenost**: Pomaže u ispunjavanju regulatornih zahtjeva za kontrolu pristupa i revizijske tragove
- **Administrativna kontrola**: IT administratori održavaju centralizirano upravljanje korisnicima

### Podrška za SAML 2.0

FastComments implementira SAML 2.0, najšire prihvaćenu verziju SAML standarda. Naša implementacija podržava:

- HTTP-POST i HTTP-Redirect binding
- Potpisane SAML odgovore i assertion-e
- Šifrirane assertion-e (opcionalno)
- Više algoritama za potpis i digest
- Različite formate name identifier