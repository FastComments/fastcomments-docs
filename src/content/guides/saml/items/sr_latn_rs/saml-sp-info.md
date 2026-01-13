Kada je SAML omogućen u FastComments, sistem automatski generiše informacije o provajderu usluge (SP) koje treba da konfigurišete u svom provajderu identiteta.

### Pristup informacijama o provajderu usluge

Informacije o SP prikazuju se na stranici za SAML konfiguraciju nakon što omogućite SAML autentifikaciju. Ove informacije uključuju sve detalje koje vaš provajder identiteta treba da uspostavi SAML odnos poverenja.

### Krajnje tačke provajdera usluge

#### SP Entity ID / Audience
**Svrha**: Jedinstveno identifikuje vašu FastComments instancu kao provajdera usluge  
**Format**: `https://fastcomments.com/saml/{your-tenant-id}`  
**Upotreba**: Konfigurišite ovo kao Entity ID ili Audience u vašem IdP-u

Ovaj identifikator osigurava da su SAML odgovori namenjeni vašem konkretnom FastComments tenant-u i sprečava prihvatanje SAML odgovora od strane drugih instanci.

#### Assertion Consumer Service (ACS) URL
**Svrha**: Krajnja tačka na koju vaš IdP šalje SAML odgovore nakon autentifikacije korisnika  
**Format**: `https://fastcomments.com/saml/callback/{your-tenant-id}`  
**Upotreba**: Konfigurišite ovo kao ACS URL ili Reply URL u vašem IdP-u

Ovo je mesto na koje se korisnici preusmeravaju nakon uspešne autentifikacije kod vašeg provajdera identiteta, zajedno sa SAML asertacijom koja sadrži informacije o korisniku.

#### SP Metadata URL
**Svrha**: Pruža kompletnu SAML konfiguraciju u standardnom XML formatu  
**Format**: `https://fastcomments.com/saml/metadata/{your-tenant-id}`  
**Upotreba**: Neki IdP-ovi mogu automatski uvesti konfiguraciju koristeći ovaj URL

Metadata URL sadrži sve potrebne informacije o SP-u u XML formatu, što olakšava automatsku konfiguraciju kompatibilnih provajdera identiteta.

#### SAML Login URL
**Svrha**: Direktan link za pokretanje SAML autentifikacije za vaš tenant  
**Format**: `https://fastcomments.com/saml/login/{your-tenant-id}`  
**Upotreba**: Povežite korisnike direktno na SAML autentifikaciju ili testirajte tok

Možete koristiti ovaj URL za testiranje SAML autentifikacije ili da korisnicima obezbedite direktan link za prijavu putem SAML-a.

### Podrška za SAML binding-e

FastComments podržava sledeće SAML binding-e:

#### HTTP-POST Binding
- **Primarna metoda**: Najčešći binding za SAML odgovore  
- **Bezbednost**: SAML odgovor se šalje putem HTTP POST na ACS URL  
- **Upotreba**: Preporučeno za produkcione implementacije

#### HTTP-Redirect Binding
- **Alternativna metoda**: SAML odgovor se šalje putem HTTP preusmeravanja  
- **Ograničenja**: Ograničen kapacitet payload-a zbog ograničenja dužine URL-a  
- **Upotreba**: Podržano, ali HTTP-POST je preferiran

### Politika Name ID

FastComments u SAML zahtevima konfiguriše sledeću politiku Name ID:

- **Podrazumevani format**: `urn:oasis:names:tc:SAML:1.1:nameid-format:emailAddress`  
- **Alternativni formati**: Persistent, Transient, Unspecified (podesivo)  
- **Zahtev**: Email adresa se koristi kao primarni identifikator korisnika

### Atributi SAML zahteva

Prilikom iniciranja SAML autentifikacije, FastComments šalje zahteve sa sledećim karakteristikama:

#### Potpisivanje zahteva
- **Status**: Opcionalno (podesivo)  
- **Algoritam**: Poklapa se sa konfigurisanim algoritmom potpisa  
- **Sertifikat**: Koristi tenant-specifičan sertifikat ako je potpisivanje zahteva omogućeno

#### Traženi atributi
FastComments zahteva sledeće atribute u SAML AuthnRequests:

- **Email**: Obavezno za identifikaciju korisnika  
- **First Name**: Opcionalno za prikaz  
- **Last Name**: Opcionalno za prikaz  
- **Roles/Groups**: Opcionalno za kontrolu pristupa i dozvola

### Kopiranje SP informacija

Stranica za SAML konfiguraciju pruža polja koja se mogu kliknuti i koja automatski kopiraju SP informacije u vaš clipboard:

1. Kliknite bilo koje polje sa informacijom o SP-u (Entity ID, ACS URL, itd.)  
2. Vrednost se automatski kopira u vaš clipboard  
3. Nalepite vrednost u konfiguraciju vašeg provajdera identiteta  
4. Kratko isticanje označava uspešno kopiranje

Ovo olakšava tačan prenos SP informacija u vaš IdP bez grešaka pri kucanju.

### Informacije o SP sertifikatu

#### Upotreba sertifikata
- **Svrha**: Šifruje komunikaciju i verifikuje identitet SP-a  
- **Rotacija**: FastComments automatski upravlja sertifikatima  
- **Pristup**: Javni sertifikati su dostupni putem metadata URL-a

#### Detalji sertifikata
- **Algoritam**: RSA-2048 ili viši  
- **Validnost**: Sertifikati se automatski obnavljaju pre isteka  
- **Distribucija**: Dostupno kroz standardni SAML metadata

### Otklanjanje problema sa SP konfiguracijom

Ako vaš provajder identiteta prijavi probleme sa SP informacijama:

1. **Proverite URL-ove**: Uverite se da svi URL-ovi koriste HTTPS i uključuju ispravan tenant ID  
2. **Proverite metadata**: Koristite metadata URL da verifikujete konfiguraciju  
3. **Testirajte konektivnost**: Uverite se da vaš IdP može da pristupi FastComments endpoint-ima  
4. **Proverite format**: Potvrdite da vaš IdP podržava format SP informacija

Uobičajeni problemi uključuju:
- Pogrešan tenant ID u URL-ovima  
- Problemi sa mrežnom konektivnošću između IdP-a i FastComments-a  
- IdP očekuje različite formate URL-ova ili dodatne opcije konfiguracije