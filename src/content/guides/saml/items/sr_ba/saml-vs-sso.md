FastComments nudi i SSO i SAML autentikaciju. Razumijevanje razlika pomaže vam da odaberete pravi pristup za vašu organizaciju.

### Simple/Secure SSO Implementacije

FastComments nudi dva različita SSO toka za autentikaciju u widgetu za komentare preko vaše stranice. Ovo se razlikuje od SAML-a i ne zahtijeva SAML. Umjesto toga, Simple SSO jednostavno zahtijeva prosljeđivanje objekta widgetu za komentare, dok Secure SSO radi isto plus hashiranje payloada pomoću API key.

SAML, s druge strane, autentificira korisnika ka cijelom proizvodu (na osnovu njihovih dozvola) *kao i* widgetu za komentare (ako imaju omogućene third party cookies za naš domen).

### SAML Autentikacija

SAML je enterprise razine protokol za autentikaciju koji pruža robusnije sigurnosne i integracione mogućnosti:

- **Implementacija**: Zahtijeva konfiguraciju Identity Provider (IdP) i razmjenu sertifikata
- **Sigurnost**: Koristi potpisane XML izjave i podržava enkripciju
- **Primjena**: Idealno za preduzeća sa postojećom SAML infrastrukturom (Active Directory, Okta, itd.)
- **Složenost postavljanja**: Više uključeno - zahtijeva konfiguraciju IdP-a i upravljanje sertifikatima
- **Enterprise funkcije**: Napredno mapiranje uloga, centralizovano upravljanje korisnicima, audit zapisi

### Kada odabrati SAML

Razmotrite SAML autentikaciju ako vaša organizacija:

- Već koristi SAML-kompatibilnog Identity Provider-a (Okta, Azure AD, ADFS, itd.)
- Zahtijeva sigurnost i usklađenost enterprise razine
- Treba centralizovano upravljanje korisnicima i kontrolu pristupa
- Ima više aplikacija koje koriste SAML za autentikaciju
- Zahtijeva detaljne audit zapise i sigurnosno izvještavanje

### Kada odabrati Simple ili Secure SSO

Naša SSO rješenja fokusirana na widget mogu biti dovoljna ako vi:

- Imate prilagođeni sistem autentikacije
- Trebate brzu implementaciju sa minimalnim postavljanjem
- Ne trebate integraciju sa enterprise provajderom identiteta
- Želite kontrolisati podatke korisnika direktno iz vaše aplikacije
- Imate jednostavnije sigurnosne zahtjeve

Simple and Secure SSO se obično koriste za online portale, blogove, itd., gdje korisnik već ima nalog *putem vaše stranice ili aplikacije* ali ne mora nužno koristiti SAML.