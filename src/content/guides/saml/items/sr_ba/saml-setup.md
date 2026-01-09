Postavljanje SAML autentifikacije u FastComments zahtijeva i konfiguraciju u vašem administratorskom dashboardu i postavke u vašem provajderu identiteta.

### Preduslovi

Prije nego što konfigurirate SAML, osigurajte da imate:

- FastComments Flex ili Pro plan (SAML nije dostupan na Creators planu)
- Administrativni pristup vašem FastComments nalogu
- Administrativni pristup vašem provajderu identiteta
- SAML metadata ili informacije o sertifikatu vašeg IdP-a

### Pristup SAML konfiguraciji

1. Prijavite se u vaš [FastComments admin dashboard](https://fastcomments.com/auth/my-account)
2. Idite na **Postavke API/SSO** u lijevom sidebaru
3. Kliknite na dugme **SAML konfiguracija**

Ako ne vidite dugme SAML konfiguracija, provjerite da li:
- Vaš nalog ima potreban paket (Flex ili Pro)
- Imate administratorske dozvole
- Vaš korisnik ima uloge API Admin ili Admin Admin

### Osnovna SAML konfiguracija

#### Omogućavanje SAML autentifikacije

1. Označite checkbox **Omogući SAML autentifikaciju**
2. Ovo aktivira SAML za vaš tenant i omogućava polja za konfiguraciju

#### Obavezna polja

**IdP Single Sign-On URL** *(Obavezno)*
- URL na koji će korisnici biti preusmjereni za autentifikaciju
- Obično ga obezbijedi vaš provajder identiteta
- Primjer: `https://your-company.okta.com/app/fastcomments/sso/saml`

**IdP X.509 Certificate** *(Obavezno)*
- Javni sertifikat od vašeg provajdera identiteta
- Koristi se za provjeru autentičnosti SAML odgovora
- Mora uključivati kompletan sertifikat sa BEGIN/END markerima
- Primjer formata:
```
-----BEGIN CERTIFICATE-----
MIICXjCCAcegAwIBAgIBADANBgkqhkiG9w0BAQsFADA...
-----END CERTIFICATE-----
```

#### Opcionalna polja

**IdP Entity ID / Issuer**
- Identifikuje vašeg provajdera identiteta
- Ako ostavite prazno, podrazumijeva se vaš FastComments URL
- Trebalo bi odgovarati issuer-u konfigurisanom u vašem IdP-u

### Napredna konfiguracija

#### Sigurnosne postavke

**Signature Algorithm**
- Podrazumijevano SHA-256 (preporučeno)
- Opcije: SHA-1, SHA-256, SHA-512
- Trebalo bi odgovarati konfiguraciji vašeg IdP-a

**Digest Algorithm**
- Podrazumijevano SHA-256 (preporučeno)
- Koristi se za izračunavanje digest-a u SAML odgovorima
- Trebalo bi odgovarati konfiguraciji vašeg IdP-a

**Name ID Format**
- Podrazumijevano format Email Address
- Određuje kako su korisnički identifikatori formatirani
- Uobičajene opcije: Email Address, Persistent, Transient

#### Enkripcija (opcionalno)

**Private Key for Decryption**
- Potrebno samo ako vaš IdP enkriptuje SAML assertion-e
- Nalijepite vaš privatni ključ koji se koristi za dešifrovanje
- Većina deploymenta ne zahtijeva enkripciju assertion-a

### Čuvanje konfiguracije

1. Pregledajte sve postavke radi tačnosti
2. Kliknite **Sačuvaj SAML konfiguraciju**
3. Sistem će validirati vašu konfiguraciju
4. Ako je uspješno, vidjet ćete poruku o potvrdi

### Sljedeći koraci

Nakon što sačuvate vašu FastComments SAML konfiguraciju:

1. Konfigurišite vašeg provajdera identiteta koristeći informacije o Service Provider-u
2. Testirajte tok autentifikacije
3. Podesite korisničke uloge i dozvole po potrebi

Informacije o Service Provider-u potrebne za konfiguraciju vašeg IdP-a biće prikazane čim SAML bude omogućen.