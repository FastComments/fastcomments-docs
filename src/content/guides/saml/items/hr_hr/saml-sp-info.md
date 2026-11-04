Kada je SAML omogućen u FastComments, sustav automatski generira informacije o Service Provideru (SP) koje trebate konfigurirati u svom identitetskom pružatelju (IdP).

### Pristup informacijama Service Providera

SP informacije prikazuju se na stranici SAML konfiguracije nakon omogućavanja SAML autentifikacije. Ove informacije uključuju sve detalje koje vaš identitetski pružatelj treba za uspostavljanje SAML odnosa povjerenja.

### Endpointi Service Providera

#### SP Entity ID / Audience
**Svrha**: Jedinstveno identificira vašu FastComments instancu kao service providera
**Format**: `https://fastcomments.com/saml/{your-tenant-id}`
**Upotreba**: Konfigurirajte ovo kao Entity ID ili Audience u svom IdP-u

Ovaj identifikator osigurava da su SAML odgovori namijenjeni vašoj specifičnoj FastComments tenant instanci i sprječava da SAML odgovori budu prihvaćeni od drugih instanci.

#### Assertion Consumer Service (ACS) URL
**Svrha**: Endpoint na koji vaš IdP šalje SAML odgovore nakon autentifikacije korisnika
**Format**: `https://fastcomments.com/saml/callback/{your-tenant-id}`
**Upotreba**: Konfigurirajte ovo kao ACS URL ili Reply URL u svom IdP-u

Ovdje se korisnici preusmjeravaju nakon uspješne autentifikacije kod vašeg identitetskog pružatelja, zajedno sa SAML assertionom koji sadrži informacije o korisniku.

#### SP Metadata URL
**Svrha**: Pruža potpunu SAML konfiguraciju u standardnom XML formatu
**Format**: `https://fastcomments.com/saml/metadata/{your-tenant-id}`
**Upotreba**: Neki IdP-ovi mogu automatski uvesti konfiguraciju koristeći ovaj URL

Metadata URL sadrži sve potrebne SP informacije u XML formatu, što olakšava automatsku konfiguraciju kompatibilnih identitetskih pružatelja.

#### SAML Login URL
**Svrha**: Izravna poveznica za pokretanje SAML autentifikacije za vašu tenant instancu
**Format**: `https://fastcomments.com/saml/login/{your-tenant-id}`
**Upotreba**: Povežite korisnike izravno na SAML autentifikaciju ili testirajte tijek

Ovaj URL možete koristiti za testiranje SAML autentifikacije ili za pružanje izravne poveznice korisnicima za prijavu putem SAML-a.

### Podrška za SAML bindinge

FastComments podržava sljedeće SAML bindinge:

#### HTTP-POST Binding
- **Primarna metoda**: Najčešći binding za SAML odgovore
- **Sigurnost**: SAML odgovor šalje se putem HTTP POST-a na ACS URL
- **Upotreba**: Preporučeno za produkcijske implementacije

#### HTTP-Redirect Binding
- **Alternativna metoda**: SAML odgovor šalje se putem HTTP preusmjeravanja
- **Ograničenja**: Ograničena veličina payloada zbog ograničenja duljine URL-a
- **Upotreba**: Podržano, ali je HTTP-POST preferiran

### Name ID politika

FastComments konfigurira sljedeću Name ID politiku u SAML zahtjevima:

- **Zadani format**: `urn:oasis:names:tc:SAML:1.1:nameid-format:emailAddress`
- **Alternativni formati**: Persistent, Transient, Unspecified (konfigurabilno)
- **Zahtjev**: E-mail adresa koristi se kao primarni identifikator korisnika

### Atributi SAML zahtjeva

Prilikom pokretanja SAML autentifikacije, FastComments šalje zahtjeve s ovim karakteristikama:

#### Potpisivanje zahtjeva
- **Status**: Neobavezno (konfigurabilno)
- **Algoritam**: Odgovara konfiguriranom algoritmu potpisa
- **Certifikat**: Koristi certifikat specifičan za tenant ako je potpisivanje zahtjeva omogućeno

#### Traženi atributi
FastComments traži sljedeće atribute u SAML AuthnRequestima:

- **Email**: Obavezan za identifikaciju korisnika
- **First Name**: Neobavezno, za potrebe prikaza
- **Last Name**: Neobavezno, za potrebe prikaza
- **Roles/Groups**: Neobavezno, za kontrolu pristupa i dozvole

### Kopiranje SP informacija

Stranica SAML konfiguracije pruža klikabilna polja koja automatski kopiraju SP informacije u međuspremnik:

1. Kliknite bilo koje SP informacijsko polje (Entity ID, ACS URL itd.)
2. Vrijednost se automatski kopira u vaš međuspremnik
3. Zalijepite vrijednost u konfiguraciju svog identitetskog pružatelja
4. Kratko isticanje označava uspješno kopiranje

To olakšava točan prijenos SP informacija u vaš IdP bez grešaka u kucanju.

### Informacije o SP certifikatu

#### Upotreba certifikata
- **Svrha**: Šifrira komunikacije i potvrđuje identitet SP-a
- **Rotacija**: Certifikatima se automatski upravlja u FastCommentsu
- **Pristup**: Javni certifikati dostupni su putem metadata URL-a

#### Detalji certifikata
- **Algoritam**: RSA-2048 ili viši
- **Valjanost**: Certifikati se automatski obnavljaju prije isteka
- **Distribucija**: Dostupni kroz standardne SAML metadata

### Rješavanje problema s SP konfiguracijom

Ako vaš identitetski pružatelj prijavljuje probleme sa SP informacijama:

1. **Provjerite URL-ove**: Osigurajte da svi URL-ovi koriste HTTPS i uključuju ispravan tenant ID
2. **Provjerite metadata**: Koristite metadata URL za provjeru konfiguracije
3. **Testirajte povezivost**: Osigurajte da vaš IdP može doći do FastComments endpointa
4. **Provjerite format**: Potvrdite da vaš IdP podržava format SP informacija

Uobičajeni problemi uključuju:
- Netočan tenant ID u URL-ovima
- Probleme s mrežnom povezivošću između IdP-a i FastCommentsa
- IdP koji očekuje drugačije formate URL-ova ili dodatne opcije konfiguracije