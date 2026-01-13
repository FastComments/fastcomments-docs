FastComments nudi i SSO i SAML autentifikaciju. Razumevanje razlika pomaže vam da izaberete pravi pristup za vašu organizaciju.

### Simple/Secure SSO rešenja

FastComments nudi dva različita SSO toka za autentifikaciju u widgetu za komentare preko vašeg sajta.
Ovo se razlikuje od SAML-a i ne zahteva SAML. Umesto toga, Simple SSO jednostavno zahteva prosleđivanje objekta widgetu za komentare, dok Secure SSO radi to i dodatno hešira payload pomoću API ključa.

SAML, s druge strane, autentifikuje korisnika za ceo proizvod (na osnovu njihovih dozvola) *kao i* widget za komentare
(ako imaju omogućene third party kolačiće za naš domen).

### SAML autentifikacija

SAML je enterprise-nivo autentifikacioni protokol koji obezbeđuje robusniju sigurnost i mogućnosti integracije:

- **Implementation**: Zahteva konfiguraciju Identity Provider-a (IdP) i razmenu sertifikata
- **Security**: Koristi potpisane XML asercije i podržava enkripciju
- **Use Case**: Idealan za preduzeća sa postojećom SAML infrastrukturom (Active Directory, Okta, itd.)
- **Setup Complexity**: Više uključen - zahteva konfiguraciju IdP-a i upravljanje sertifikatima
- **Enterprise Features**: Napredno mapiranje uloga, centralizovano upravljanje korisnicima, audit tragovi

### Kada izabrati SAML

Razmotrite SAML autentifikaciju ako vaša organizacija:

- Već koristi SAML-kompatibilnog provajdera identiteta (Okta, Azure AD, ADFS, itd.)
- Zahteva sigurnost i usklađenost na nivou preduzeća
- Treba centralizovano upravljanje korisnicima i kontrolu pristupa
- Ima više aplikacija koje koriste SAML za autentifikaciju
- Zahteva detaljne audit tragove i izveštavanje o bezbednosti

### Kada izabrati Simple ili Secure SSO

Naša SSO rešenja fokusirana na widget mogu biti dovoljna ako vi:

- Imate prilagođeni sistem autentifikacije
- Trebate brz implementaciju uz minimalno podešavanje
- Ne zahtevate integraciju sa provajderom identiteta na nivou preduzeća
- Želite da kontrolišete podatke o korisnicima direktno iz vaše aplikacije
- Imate jednostavnije sigurnosne zahteve

Simple i Secure SSO se često koriste za onlajn portale, blogove itd., gde korisnik već ima nalog *preko vašeg sajta ili aplikacije*
ali ne koristi nužno SAML.