Postavljanje SAML autentifikacije u FastComments zahteva i konfiguraciju u vašem administratorskom panelu i podešavanje kod vašeg provajdera identiteta.

### Preduslovi

Pre nego što konfigurišete SAML, uverite se da imate:

- FastComments Flex ili Pro plan (SAML nije dostupan na Creators planu)
- Administrativni pristup vašem FastComments nalogu
- Administrativni pristup vašem provajderu identiteta
- SAML metadata vašeg IdP-a ili informacije o sertifikatu

### Pristup SAML konfiguraciji

1. Prijavite se na vaš [FastComments admin dashboard](https://fastcomments.com/auth/my-account)
2. Idite na **API/SSO Settings** u levom bočnom meniju
3. Kliknite na dugme **SAML Config**

Ako ne vidite dugme **SAML Config**, proverite da:
- Vaš nalog ima potreban paket (Flex ili Pro)
- Imate administrativne dozvole
- Vaš korisnik ima uloge API Admin ili Admin Admin

### Osnovna SAML konfiguracija

#### Omogućavanje SAML autentifikacije

1. Označite polje za potvrdu **Enable SAML Authentication**
2. Time se aktivira SAML za vaš tenant i polja za konfiguraciju postaju dostupna

#### Obavezna polja

**IdP Single Sign-On URL** *(Required)*
- URL na koji će korisnici biti preusmereni radi autentifikacije
- Obično ga daje vaš provajder identiteta
- Primer: `https://your-company.okta.com/app/fastcomments/sso/saml`

**IdP X.509 Certificate** *(Required)*
- Javni sertifikat od vašeg provajdera identiteta
- Koristi se za verifikaciju autentičnosti SAML odgovora
- Mora sadržati potpuni sertifikat sa BEGIN/END markerima
- Primer formata:
```
-----BEGIN CERTIFICATE-----
MIICXjCCAcegAwIBAgIBADANBgkqhkiG9w0BAQsFADA...
-----END CERTIFICATE-----
```

#### Neobavezna polja

**IdP Entity ID / Issuer**
- Identifikuje vašeg provajdera identiteta
- Ako ostane prazno, podrazumeva se vaš FastComments URL
- Trebalo bi da se poklapa sa issuer-om podesnim u vašem IdP-u

### Napredna konfiguracija

#### Bezbednosna podešavanja

**Signature Algorithm**
- Podrazumevano: SHA-256 (preporučeno)
- Opcije: SHA-1, SHA-256, SHA-512
- Trebalo bi da se poklapa sa podešavanjem vašeg IdP-a

**Digest Algorithm**
- Podrazumevano: SHA-256 (preporučeno)
- Koristi se za izračunavanje digest-a u SAML odgovorima
- Trebalo bi da se poklapa sa podešavanjem vašeg IdP-a

**Name ID Format**
- Podrazumevano: format Email Address
- Određuje kako su identifikatori korisnika formatirani
- Uobičajene opcije: Email Address, Persistent, Transient

#### Šifrovanje (opciono)

**Private Key for Decryption**
- Potreban samo ako vaš IdP enkriptuje SAML assertion-e
- Nalepite vaš privatni ključ koji se koristi za dekripciju
- Većina implementacija ne zahteva enkripciju assertion-a

### Čuvanje konfiguracije

1. Pregledajte sva podešavanja radi tačnosti
2. Kliknite **Save SAML Configuration**
3. Sistem će validirati vašu konfiguraciju
4. Ako je uspešno, videćete poruku o potvrdi

### Sledeći koraci

Nakon čuvanja SAML konfiguracije u FastComments:

1. Konfigurišite vaš provajder identiteta koristeći informacije Service Provider-a
2. Testirajte tok autentifikacije
3. Podesite korisničke uloge i dozvole po potrebi

Informacije o Service Provider-u potrebne za podešavanje vašeg IdP-a biće prikazane kada se SAML omogući.