Kada je SAML omogućen u FastCommentsu, sustav automatski generira informacije o Service Provideru (SP) koje trebate konfigurirati u svom davatelju identiteta.

### Pristup informacijama o Service Provideru

Informacije o SP-u prikazuju se na stranici konfiguracije SAML-a nakon što omogućite SAML autentikaciju. Ove informacije uključuju sve detalje koje vaš davatelj identiteta treba za uspostavu SAML povjerenja.

### Krajnje točke Service Providera

#### SP Entity ID / Audience
**Svrha**: Jedinstveno identificira vašu FastComments instancu kao pružatelja usluga  
**Format**: `https://fastcomments.com/saml/{your-tenant-id}`  
**Upotreba**: Konfigurirajte ovo kao Entity ID ili Audience u vašem IdP-u

Ovaj identifikator osigurava da su SAML odgovori namijenjeni za vaš specifični FastComments tenant i sprječava prihvaćanje SAML odgovora od drugih instanci.

#### Assertion Consumer Service (ACS) URL
**Svrha**: Krajnja točka na koju vaš IdP šalje SAML odgovore nakon autentikacije korisnika  
**Format**: `https://fastcomments.com/saml/callback/{your-tenant-id}`  
**Upotreba**: Konfigurirajte ovo kao ACS URL ili Reply URL u vašem IdP-u

Ovdje se korisnici preusmjeravaju nakon uspješne autentikacije kod vašeg davatelja identiteta, zajedno sa SAML asercijom koja sadrži informacije o korisniku.

#### SP Metadata URL
**Svrha**: Pruža kompletnu SAML konfiguraciju u standardnom XML formatu  
**Format**: `https://fastcomments.com/saml/metadata/{your-tenant-id}`  
**Upotreba**: Neki IdP-ovi mogu automatski uvesti konfiguraciju koristeći ovaj URL

Metadata URL sadrži sve potrebne SP informacije u XML formatu, što olakšava automatsku konfiguraciju kompatibilnih davatelja identiteta.

#### SAML Login URL
**Svrha**: Direktna poveznica za pokretanje SAML autentikacije za vaš tenant  
**Format**: `https://fastcomments.com/saml/login/{your-tenant-id}`  
**Upotreba**: Povežite korisnike izravno na SAML autentikaciju ili testirajte tok

Ovaj URL možete koristiti za testiranje SAML autentikacije ili korisnicima dati izravnu poveznicu za prijavu putem SAML-a.

### Podrška za SAML bindinge

FastComments podržava sljedeće SAML bindinge:

#### HTTP-POST Binding
- **Primarna metoda**: Najčešći binding za SAML odgovore  
- **Sigurnost**: SAML odgovor se šalje putem HTTP POST na ACS URL  
- **Upotreba**: Preporučeno za produkcijska okruženja

#### HTTP-Redirect Binding
- **Alternativna metoda**: SAML odgovor se šalje putem HTTP preusmjeravanja  
- **Ograničenja**: Ograničena veličina payloada zbog ograničenja duljine URL-a  
- **Upotreba**: Podržano, ali HTTP-POST je preporučen

### Name ID policy

FastComments konfigurira sljedeće pravilo Name ID u SAML zahtjevima:

- **Zadani format**: `urn:oasis:names:tc:SAML:1.1:nameid-format:emailAddress`  
- **Alternativni formati**: Persistent, Transient, Unspecified (konfigurabilno)  
- **Zahtjev**: Email adresa se koristi kao primarni identifikator korisnika

### Atributi SAML zahtjeva

Prilikom pokretanja SAML autentikacije, FastComments šalje zahtjeve s ovim karakteristikama:

#### Potpisivanje zahtjeva
- **Status**: Opcionalno (konfigurabilno)  
- **Algoritam**: Usklađuje se s konfiguriranim algoritmom potpisa  
- **Certifikat**: Koristi tenant-specifični certifikat ako je potpisivanje zahtjeva omogućeno

#### Zahtijevani atributi
FastComments zahtijeva sljedeće atribute u SAML AuthnRequests:

- **Email**: Potreban za identifikaciju korisnika  
- **Ime**: Opcionalno za prikaz  
- **Prezime**: Opcionalno za prikaz  
- **Uloge/Grupе**: Opcionalno za kontrolu pristupa i dozvole

### Kopiranje SP informacija

Stranica konfiguracije SAML-a pruža polja na koja se može kliknuti i koja automatski kopiraju SP informacije u vaš međuspremnik:

1. Kliknite bilo koje polje s informacijama o SP-u (Entity ID, ACS URL itd.)  
2. Vrijednost se automatski kopira u vaš međuspremnik  
3. Zalijepite vrijednost u konfiguraciju vašeg davatelja identiteta  
4. Kratko isticanje označava uspješno kopiranje

To olakšava točan prijenos SP informacija u vaš IdP bez pogrešaka pri tipkanju.

### Informacije o SP certifikatu

#### Upotreba certifikata
- **Svrha**: Šifrira komunikaciju i potvrđuje identitet SP-a  
- **Rotacija**: Certifikati se automatski upravljaju od strane FastCommentsa  
- **Pristup**: Javne certifikate možete dobiti putem metadata URL-a

#### Detalji certifikata
- **Algoritam**: RSA-2048 ili jači  
- **Valjanost**: Certifikati se automatski obnavljaju prije isteka  
- **Distribucija**: Dostupno putem standardne SAML metadata

### Rješavanje problema konfiguracije SP-a

Ako vaš davatelj identiteta prijavljuje probleme s informacijama o SP-u:

1. **Provjerite URL-ove**: Osigurajte da svi URL-ovi koriste HTTPS i sadrže ispravan tenant ID  
2. **Provjerite metadata**: Koristite metadata URL za provjeru konfiguracije  
3. **Testirajte povezanost**: Provjerite može li vaš IdP dosegnuti FastComments krajnje točke  
4. **Potvrdite format**: Potvrdite da vaš IdP podržava format informacija o SP-u

Česti problemi uključuju:
- Pogrešan tenant ID u URL-ovima  
- Problemi mrežne povezanosti između IdP-a i FastCommentsa  
- IdP očekuje drugačiji format URL-ova ili dodatne opcije konfiguracije