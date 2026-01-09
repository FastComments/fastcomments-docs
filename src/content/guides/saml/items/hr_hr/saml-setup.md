Postavljanje SAML autentikacije u FastComments zahtijeva konfiguraciju i u vašem administratorskom panelu i postavke u vašem pružatelju identiteta.

### Preduvjeti

Prije konfiguriranja SAML-a, osigurajte da imate:

- Flex ili Pro plan FastComments-a (SAML nije dostupan na Creators planu)
- Administrativni pristup vašem FastComments računu
- Administrativni pristup vašem pružatelju identiteta
- SAML metadata vašeg IdP-a ili informacije o certifikatu

### Pristup SAML konfiguraciji

1. Prijavite se u svoj [FastComments administratorski panel](https://fastcomments.com/auth/my-account)
2. Idite na **API/SSO Settings** u lijevom izborniku
3. Kliknite gumb **SAML Config**

Ako ne vidite gumb SAML Config, provjerite da:
- vaš račun ima potrebni paket (Flex ili Pro)
- imate administratorske ovlasti
- vaš korisnik ima uloge API Admin ili Admin Admin

### Osnovna SAML konfiguracija

#### Omogućite SAML autentikaciju

1. Označite potvrdni okvir **Enable SAML Authentication**
2. Ovo aktivira SAML za vaš tenant i čini polja za konfiguraciju dostupnima

#### Obavezna polja

**IdP Single Sign-On URL** *(Obavezno)*
- URL na koji će korisnici biti preusmjereni radi autentikacije
- Obično ga pruža vaš pružatelj identiteta
- Primjer: `https://your-company.okta.com/app/fastcomments/sso/saml`

**IdP X.509 Certificate** *(Obavezno)*
- Javni certifikat vašeg pružatelja identiteta
- Koristi se za provjeru autentičnosti SAML odgovora
- Mora uključivati cijeli certifikat s BEGIN/END oznakama
- Primjer formata:
```
-----BEGIN CERTIFICATE-----
MIICXjCCAcegAwIBAgIBADANBgkqhkiG9w0BAQsFADA...
-----END CERTIFICATE-----
```

#### Opcionalna polja

**IdP Entity ID / Issuer**
- Identificira vašeg pružatelja identiteta
- Ako je ostavljeno prazno, zadan je vaš FastComments URL
- Trebao bi se podudarati s issuer-om konfiguriranim u vašem IdP-u

### Napredna konfiguracija

#### Sigurnosne postavke

**Signature Algorithm**
- Zadano: SHA-256 (preporučeno)
- Opcije: SHA-1, SHA-256, SHA-512
- Treba odgovarati konfiguraciji vašeg IdP-a

**Digest Algorithm**
- Zadano: SHA-256 (preporučeno)
- Koristi se za izračun sažetka u SAML odgovorima
- Treba odgovarati konfiguraciji vašeg IdP-a

**Name ID Format**
- Zadano: format Email Address
- Određuje kako su formatirani identifikatori korisnika
- Uobičajene opcije: Email Address, Persistent, Transient

#### Šifriranje (neobavezno)

**Private Key for Decryption**
- Potrebno samo ako vaš IdP šifrira SAML assertions
- Zalijepite svoj privatni ključ koji se koristi za dešifriranje
- Većina implementacija ne zahtijeva šifriranje assertions

### Spremanje konfiguracije

1. Pregledajte sve postavke radi točnosti
2. Kliknite **Save SAML Configuration**
3. Sustav će provjeriti vašu konfiguraciju
4. Ako je uspješno, vidjet ćete poruku o potvrdi

### Sljedeći koraci

Nakon spremanja FastComments SAML konfiguracije:

1. Konfigurirajte svog pružatelja identiteta koristeći informacije o Service Provideru
2. Testirajte tijek autentikacije
3. Postavite korisničke uloge i dozvole po potrebi

Informacije o Service Provideru potrebne za konfiguraciju vašeg IdP-a bit će prikazane nakon što SAML bude omogućen.