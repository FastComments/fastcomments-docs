FastComments nudi i SSO i SAML autentikaciju. Razumijevanje razlika pomaže vam odabrati pravi pristup za vašu organizaciju.

### Simple/Secure SSO rješenja

FastComments nudi dva različita SSO toka za autentikaciju u widgetu za komentare putem vaše stranice.
Ovo se razlikuje od SAML-a i ne zahtijeva SAML. Umjesto toga, Simple SSO jednostavno zahtijeva prosljeđivanje objekta widgetu za komentare, dok Secure SSO radi to isto plus hashira payload pomoću API key.

SAML, s druge strane, autentificira korisnika za cijeli proizvod (na temelju njihovih dozvola) *kao i* widget za komentare
(ako imaju omogućene kolačiće trećih strana za našu domenu).

### SAML autentikacija

SAML je autentikacijski protokol razine poduzeća koji pruža robusniju sigurnost i mogućnosti integracije:

- **Implementacija**: Zahtijeva konfiguraciju Identity Provider (IdP) i razmjenu certifikata
- **Sigurnost**: Koristi potpisane XML asercije i podržava enkripciju
- **Primjena**: Idealno za poduzeća s postojećom SAML infrastrukturom (Active Directory, Okta, itd.)
- **Složenost postavljanja**: Više uključno - zahtijeva konfiguraciju IdP-a i upravljanje certifikatima
- **Značajke za poduzeća**: Napredno mapiranje uloga, centralizirano upravljanje korisnicima, audit zapisi

### Kada odabrati SAML

Razmislite o SAML autentikaciji ako vaša organizacija:

- Već koristi SAML-kompatibilnog provajdera identiteta (Okta, Azure AD, ADFS, itd.)
- Zahtijeva sigurnost i usklađenost razine poduzeća
- Treba centralizirano upravljanje korisnicima i kontrolu pristupa
- Ima više aplikacija koje koriste SAML za autentikaciju
- Zahtijeva detaljne audit zapise i sigurnosno izvještavanje

### Kada odabrati Simple ili Secure SSO

Naša SSO rješenja fokusirana na widget mogu biti dovoljna ako vi:

- Imate prilagođeni sustav autentikacije
- Trebate brzo rješenje s minimalnim postavljanjem
- Ne zahtijevate integraciju s provajderom identiteta razine poduzeća
- Želite direktno kontrolirati korisničke podatke iz vaše aplikacije
- Imate jednostavnije sigurnosne zahtjeve

Simple i Secure SSO se često koriste za online portale, blogove itd., gdje korisnik već ima račun *putem vaše stranice ili aplikacije*
ali ne koristi nužno SAML.