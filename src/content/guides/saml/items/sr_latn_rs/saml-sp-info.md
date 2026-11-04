Kada je SAML omogućen u FastComments, sistem automatski generiše informacije o Service Provider-u (SP) koje treba da konfigurišete u svom identity provider-u.

### Pristup informacijama Service Provider-a

SP informacije se prikazuju na vašoj SAML stranici za konfiguraciju nakon omogućavanja SAML autentifikacije. Ove informacije sadrže sve detalje koje je vašem identity provider-u potrebno da uspostavi SAML odnos poverenja.

### Service Provider endpoint-i

#### SP Entity ID / Audience
**Svrha**: Jedinstveno identifikuje vašu FastComments instancu kao service provider
**Format**: `https://fastcomments.com/saml/{your-tenant-id}`
**Upotreba**: Konfigurišite ovo kao Entity ID ili Audience u vašem IdP-u

Ovaj identifikator obezbeđuje da su SAML odgovori namenjeni vašoj specifičnoj FastComments tenant instanci i sprečava da SAML odgovori budu prihvaćeni od strane drugih instanci.

#### Assertion Consumer Service (ACS) URL
**Svrha**: Endpoint na koji vaš IdP šalje SAML odgovore nakon autentifikacije korisnika
**Format**: `https://fastcomments.com/saml/callback/{your-tenant-id}`
**Upotreba**: Konfigurišite ovo kao ACS URL ili Reply URL u vašem IdP-u

Ovde se korisnici preusmeravaju nakon uspešne autentifikacije kod vašeg identity provider-a, zajedno sa SAML assertion-om koji sadrži informacije o korisniku.

#### SP Metadata URL
**Svrha**: Pruža kompletnu SAML konfiguraciju u standardnom XML formatu
**Format**: `https://fastcomments.com/saml/metadata/{your-tenant-id}`
**Upotreba**: Neki IdP-ovi mogu automatski da uvezu konfiguraciju koristeći ovaj URL

Metadata URL sadrži sve potrebne SP informacije u XML formatu, čineći lakim automatsko konfigurisanje kompatibilnih identity provider-a.

#### SAML Login URL
**Svrha**: Direktan link za pokretanje SAML autentifikacije za vaš tenant
**Format**: `https://fastcomments.com/saml/login/{your-tenant-id}`
**Upotreba**: Povežite korisnike direktno na SAML autentifikaciju ili testirajte tok

Ovaj URL možete koristiti za testiranje SAML autentifikacije ili da korisnicima pružite direktan link za prijavljivanje putem SAML-a.

### Podrška za SAML binding-e

FastComments podržava sledeće SAML binding-e:

#### HTTP-POST Binding
- **Primarna metoda**: Najčešći binding za SAML odgovore
- **Bezbednost**: SAML odgovor se šalje putem HTTP POST na ACS URL
- **Upotreba**: Preporučuje se za produkcijska okruženja

#### HTTP-Redirect Binding
- **Alternativna metoda**: SAML odgovor se šalje putem HTTP preusmerenja
- **Ograničenja**: Ograničena veličina payload-a zbog ograničenja dužine URL-a
- **Upotreba**: Podržano, ali je HTTP-POST preferiran

### Name ID politika

FastComments konfiguriše sledeću Name ID politiku u SAML zahtevima:

- **Podrazumevani format**: `urn:oasis:names:tc:SAML:1.1:nameid-format:emailAddress`
- **Alternativni formati**: Persistent, Transient, Unspecified (konfigurabilno)
- **Zahtev**: Email adresa se koristi kao primarni identifikator korisnika

### Atributi SAML zahteva

Prilikom pokretanja SAML autentifikacije, FastComments šalje zahteve sa sledećim karakteristikama:

#### Potpisivanje zahteva
- **Status**: Opciono (konfigurabilno)
- **Algoritam**: Odgovara konfigurisanom algoritmu za potpis
- **Sertifikat**: Koristi sertifikat specifičan za tenant ako je potpisivanje zahteva omogućeno

#### Traženi atributi
FastComments zahteva sledeće atribute u SAML AuthnRequests:

- **Email**: Obavezan za identifikaciju korisnika
- **Ime**: Opciono, u svrhu prikaza
- **Prezime**: Opciono, u svrhu prikaza
- **Uloge/Grupe**: Opciono, za kontrolu pristupa i dozvole

### Kopiranje SP informacija

SAML stranica za konfiguraciju pruža klikabilna polja koja automatski kopiraju SP informacije u vaš clipboard:

1. Kliknite na bilo koje SP polje (Entity ID, ACS URL, itd.)
2. Vrednost se automatski kopira u vaš clipboard
3. Nalepite vrednost u konfiguraciju vašeg identity provider-a
4. Kratko isticanje označava uspešno kopiranje

Ovo olakšava tačan prenos SP informacija u vaš IdP bez grešaka u kucanju.

### Informacije o SP sertifikatu

#### Upotreba sertifikata
- **Svrha**: Šifruje komunikaciju i verifikuje SP identitet
- **Rotacija**: Sertifikatima se automatski upravlja u FastComments
- **Pristup**: Javni sertifikati su dostupni putem metadata URL-a

#### Detalji sertifikata
- **Algoritam**: RSA-2048 ili viši
- **Validnost**: Sertifikati se automatski obnavljaju pre isteka
- **Distribucija**: Dostupni kroz standardne SAML metadata

### Rešavanje problema sa SP konfiguracijom

Ako vaš identity provider prijavljuje probleme sa SP informacijama:

1. **Proverite URL-ove**: Osigurajte da svi URL-ovi koriste HTTPS i da sadrže ispravan tenant ID
2. **Proverite Metadata**: Koristite metadata URL da proverite konfiguraciju
3. **Testirajte konektivnost**: Osigurajte da vaš IdP može da dođe do FastComments endpoint-a
4. **Validirajte format**: Potvrdite da vaš IdP podržava format SP informacija

Uobičajeni problemi uključuju:
- Neispravan tenant ID u URL-ovima
- Problemi sa mrežnom konektivnošću između IdP-a i FastComments
- IdP očekuje drugačije formate URL-ova ili dodatne opcije konfiguracije