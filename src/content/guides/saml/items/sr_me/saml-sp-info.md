Kada je SAML omogućen u FastComments, sistem automatski generiše informacije o Pružaocu usluge (SP) koje trebate konfigurirati u svom provajderu identiteta.

### Pristup informacijama o Pružaocu usluge

Informacije o SP-u prikazane su na vašoj SAML stranici za konfiguraciju nakon što omogućite SAML autentifikaciju. Ove informacije sadrže sve detalje koje vaš provajder identiteta treba da uspostavi SAML odnos povjerenja.

### Krajnje tačke Pružaoca usluge

#### SP Entity ID / Publika
**Svrha**: Jedinstveno identifikuje vašu FastComments instancu kao pružaoca usluge  
**Format**: `https://fastcomments.com/saml/{your-tenant-id}`  
**Upotreba**: Konfigurišite ovo kao Entity ID ili Audience u vašem IdP-u

Ovaj identifikator osigurava da su SAML odgovori namijenjeni vašem konkretnom FastComments tenant-u i sprečava prihvatanje SAML odgovora od strane drugih instanci.

#### Assertion Consumer Service (ACS) URL
**Svrha**: Krajnja tačka na koju vaš IdP šalje SAML odgovore nakon autentifikacije korisnika  
**Format**: `https://fastcomments.com/saml/callback/{your-tenant-id}`  
**Upotreba**: Konfigurišite ovo kao ACS URL ili Reply URL u vašem IdP-u

Ovo je mjesto na koje se korisnici preusmjeravaju nakon uspješne autentifikacije kod vašeg provajdera identiteta, zajedno sa SAML asercijom koja sadrži informacije o korisniku.

#### SP Metadata URL
**Svrha**: Pruža kompletnu SAML konfiguraciju u standardnom XML formatu  
**Format**: `https://fastcomments.com/saml/metadata/{your-tenant-id}`  
**Upotreba**: Neki IdP-ovi mogu automatski uvesti konfiguraciju koristeći ovaj URL

Metadata URL sadrži sve potrebne SP informacije u XML formatu, što olakšava automatsku konfiguraciju kompatibilnih provajdera identiteta.

#### SAML Login URL
**Svrha**: Direktan link za pokretanje SAML autentifikacije za vaš tenant  
**Format**: `https://fastcomments.com/saml/login/{your-tenant-id}`  
**Upotreba**: Povežite korisnike direktno na SAML autentifikaciju ili testirajte tok

Možete koristiti ovaj URL za testiranje SAML autentifikacije ili pružanje korisnicima direktnog linka za prijavu putem SAML-a.

### Podrška za SAML binding-e

FastComments podržava sljedeće SAML binding-e:

#### HTTP-POST Binding
- **Primarna metoda**: Najčešći binding za SAML odgovore  
- **Sigurnost**: SAML odgovor se šalje putem HTTP POST na ACS URL  
- **Upotreba**: Preporučeno za produkcione implementacije

#### HTTP-Redirect Binding
- **Alternativna metoda**: SAML odgovor se šalje putem HTTP preusmjeravanja  
- **Ograničenja**: Ograničena veličina payload-a zbog dužine URL-a  
- **Upotreba**: Podržano, ali HTTP-POST je preferiran

### Politika Name ID

FastComments konfiguriše sljedeću Name ID politiku u SAML zahtjevima:

- **Podrazumijevani format**: `urn:oasis:names:tc:SAML:1.1:nameid-format:emailAddress`  
- **Alternativni formati**: Persistent, Transient, Unspecified (konfigurisano)  
- **Zahtjev**: Email adresa se koristi kao primarni identifikator korisnika

### Atributi u SAML zahtjevu

Prilikom pokretanja SAML autentifikacije, FastComments šalje zahtjeve sa ovim karakteristikama:

#### Potpisivanje zahtjeva
- **Status**: Opcionalno (konfigurisano)  
- **Algoritam**: Odgovara konfigurisanom algoritmu potpisa  
- **Sertifikat**: Koristi tenant-specifičan sertifikat ako je potpisivanje zahtjeva omogućeno

#### Zatraženi atributi
FastComments traži sljedeće atribute u SAML AuthnRequest-ima:

- **Email**: Obavezno za identifikaciju korisnika  
- **Ime**: Opcionalno za prikaz  
- **Prezime**: Opcionalno za prikaz  
- **Uloge/Grupе**: Opcionalno za kontrolu pristupa i dozvole

### Kopiranje informacija o SP-u

SAML stranica za konfiguraciju pruža klikabilna polja koja automatski kopiraju informacije o SP-u u vaš clipboard:

1. Kliknite bilo koje polje sa informacijama o SP-u (Entity ID, ACS URL, itd.)  
2. Vrijednost se automatski kopira u vaš clipboard  
3. Zalijepite vrijednost u konfiguraciju vašeg provajdera identiteta  
4. Kratko isticanje označava uspješno kopiranje

Ovo olakšava tačan prenos informacija o SP-u u vaš IdP bez grešaka pri kucanju.

### Informacije o SP sertifikatu

#### Upotreba sertifikata
- **Svrha**: Šifruje komunikaciju i potvrđuje identitet SP-a  
- **Rotacija**: Sertifikati se automatski upravljaju od strane FastComments  
- **Pristup**: Javni sertifikati su dostupni putem metadata URL-a

#### Detalji sertifikata
- **Algoritam**: RSA-2048 ili jači  
- **Važenje**: Sertifikati se automatski obnavljaju prije isteka  
- **Distribucija**: Dostupno kroz standardnu SAML metadata

### Otklanjanje problema sa SP konfiguracijom

Ako vaš provajder identiteta prijavi probleme sa informacijama o SP-u:

1. **Provjerite URL-ove**: Osigurajte da svi URL-ovi koriste HTTPS i da sadrže tačan tenant ID  
2. **Provjerite metadata**: Koristite metadata URL da verifikujete konfiguraciju  
3. **Testirajte konektivnost**: Osigurajte da vaš IdP može dohvatiti FastComments krajnje tačke  
4. **Validirajte format**: Potvrdite da vaš IdP podržava format informacija o SP-u

Uobičajeni problemi uključuju:
- Netočan tenant ID u URL-ovima  
- Problemi sa mrežnom konektivnošću između IdP-a i FastComments  
- IdP očekuje drugačije formate URL-ova ili dodatne opcije konfiguracije