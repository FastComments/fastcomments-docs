SAML (Security Assertion Markup Language) je XML-baziran otvoreni standard za razmenu podataka o autentifikaciji i autorizaciji između strana, posebno između provajdera identiteta (IdP) i provajdera usluge (SP).

### Kako SAML funkcioniše

SAML omogućava jedinstvenu prijavu (SSO) tako što korisnicima dozvoljava da se prijave jednom kod svog provajdera identiteta, a potom pristupe više aplikacija bez ponovnog unošenja kredencijala. Kada korisnik pokuša da pristupi FastComments:

1. **Zahtev za autentifikaciju**: FastComments preusmerava korisnika na vašeg provajdera identiteta
2. **Autentifikacija korisnika**: Korisnik se autentifikuje kod vašeg IdP-a (npr. Active Directory, Okta, Azure AD)
3. **SAML odgovor**: IdP šalje potpisanu SAML tvrdnju nazad FastComments-u
4. **Pristup korisnika**: FastComments validira tvrdnju i odobrava pristup autentifikovanom korisniku

### Prednosti SAML-a

- **Poboljšana bezbednost**: Centralizovana autentifikacija smanjuje rizike vezane za lozinke
- **Poboljšano korisničko iskustvo**: Korisnici se prijave jednom i besprekorno pristupaju više aplikacija
- **Usklađenost**: Pomaže u ispunjavanju regulatornih zahteva za kontrolu pristupa i evidenciju revizije
- **Administrativna kontrola**: IT administratori održavaju centralizovano upravljanje korisnicima

### Podrška za SAML 2.0

FastComments implementira SAML 2.0, najrašireniju verziju SAML standarda. Naša implementacija podržava:

- HTTP-POST i HTTP-Redirect vezanja
- Potpisani SAML odgovori i tvrdnje
- Šifrovane tvrdnje (opciono)
- Više algoritama za potpis i sažetak (digest)
- Različiti formati identifikatora imena