Ovaj vodič pokriva uobičajene probleme s SAML autentifikacijom i njihova rješenja.

### Problemi s certifikatom i sigurnošću

#### Pogreška nevažećeg certifikata

**Simptomi**:
- Pogreška "Certificate validation failed"
- Korisnici ne mogu završiti SAML autentifikaciju
- SAML odgovori su odbijeni

**Uobičajeni uzroci**:
- Format certifikata je neispravan
- Certifikat je istekao
- Pružen je pogrešan certifikat
- Dodatni znakovi ili razmaci u certifikatu

**Rješenja**:
1. **Provjerite format certifikata**:
   - Osigurajte da certifikat uključuje oznake `-----BEGIN CERTIFICATE-----` i `-----END CERTIFICATE-----`
   - Uklonite sve dodatne razmake ili prijelome redaka
   - Kopirajte certifikat izravno iz IdP metapodataka ili konfiguracije

2. **Provjerite valjanost certifikata**:
   - Provjerite da certifikat nije istekao
   - Potvrdite da je certifikat za ispravan IdP
   - Koristite online validatore certifikata za provjeru formata

3. **Ponovno preuzmite certifikat**:
   - Preuzmite svježi certifikat s IdP-a
   - Koristite URL IdP metapodataka ako je dostupan
   - Potvrdite da certifikat odgovara trenutnoj IdP konfiguraciji

#### Provjera potpisa nije uspjela

**Simptomi**:
- Pogreške provjere potpisa SAML tvrdnje
- Autentifikacija ne uspijeva nakon IdP prijave
- Poruke o pogrešci "Invalid signature"

**Rješenja**:
1. **Neslaganje algoritma**:
   - Provjerite da algoritam potpisa u FastComments odgovara IdP-u
   - Isprobajte različite algoritme potpisa (SHA-256, SHA-1, SHA-512)
   - Provjerite da algoritam sažetka odgovara IdP konfiguraciji

2. **Problemi s certifikatom**:
   - Osigurajte da je konfiguriran ispravan certifikat za potpisivanje
   - Provjerite da certifikat odgovara privatnom ključu koji koristi IdP
   - Provjerite postoji li rotacija certifikata u IdP-u

### Problemi s konfiguracijom

#### Pogrešan Entity ID ili ACS URL

**Simptomi**:
- IdP prijavljuje "Unknown Service Provider"
- SAML odgovori idu na pogrešnu krajnju točku
- Autentifikacija se ne završava

**Rješenja**:
1. **Provjerite SP informacije**:
   - Kopirajte točan Entity ID iz FastComments konfiguracije
   - Osigurajte da ACS URL odgovara formatu: `https://fastcomments.com/saml/callback/{tenant-id}`
   - Provjerite ima li pogrešaka u tenant ID-u

2. **IdP konfiguracija**:
   - Ažurirajte IdP s ispravnim SP Entity ID-om
   - Konfigurirajte odgovarajući ACS/Reply URL
   - Provjerite postavke IdP bindinga (poželjan HTTP-POST)

#### Nedostajući ili neispravni atributi

**Simptomi**:
- Korisnici stvoreni bez odgovarajućih uloga
- Nedostaju podaci korisničkog profila
- Pogreške "Email required"

**Rješenja**:
1. **Atribut e-pošte**:
   - Osigurajte da IdP šalje atribut e-pošte
   - Provjerite mapiranje naziva atributa (email, emailAddress itd.)
   - Provjerite da je vrijednost e-pošte valjana adresa e-pošte

2. **Atributi uloga**:
   - Potvrdite da IdP šalje informacije o ulogama/grupama
   - Provjerite da nazivi atributa uloga odgovaraju očekivanjima FastComments
   - Provjerite da vrijednosti uloga točno odgovaraju nazivima uloga FastComments

3. **Format atributa**:
   - Testirajte i format uloga kao niza i kao vrijednosti odvojene zarezima
   - Osigurajte da vrijednosti atributa nemaju dodatne razmake
   - Provjerite osjetljivost na velika i mala slova u nazivima uloga

### Problemi s tijekom autentifikacije

#### Petlja preusmjeravanja

**Simptomi**:
- Preglednik beskonačno preusmjerava između FastComments i IdP-a
- Autentifikacija se nikada ne završava
- Višestruka preusmjeravanja prikazana u razvojnim alatima preglednika

**Rješenja**:
1. **Provjerite SP konfiguraciju**:
   - Provjerite da Entity ID točno odgovara IdP konfiguraciji
   - Osigurajte da je ACS URL ispravno konfiguriran u IdP-u
   - Provjerite ima li kosih crta na kraju URL-ova

2. **Problemi sa sesijom**:
   - Očistite kolačiće preglednika i pokušajte ponovno
   - Testirajte u anonimnom/privatnom prozoru preglednika
   - Provjerite postavke isteka sesije

#### Odbijen pristup nakon autentifikacije

**Simptomi**:
- SAML autentifikacija uspijeva
- Korisnik je preusmjeren na FastComments
- Prikazuje se "Access denied" ili pogreška dozvola

**Rješenja**:
1. **Dodjela uloga**:
   - Provjerite ima li korisnik odgovarajuće uloge u IdP-u
   - Provjerite se atribut uloge šalje u SAML odgovoru
   - Potvrdite da nazivi uloga točno odgovaraju zahtjevima FastComments

2. **Ograničenja paketa**:
   - Provjerite ima li račun Flex ili Pro plan
   - Provjerite je li SAML značajka omogućena za paket
   - Kontaktirajte podršku ako paket uključuje SAML, ali značajka nije dostupna

### Problemi specifični za davatelja identiteta

#### Microsoft Azure AD

**Uobičajeni problemi**:
- Dodjele uloga aplikacije se ne odražavaju u tokenima
- Tvrdnje (claims) se ne šalju ispravno
- Zahtjevi za dodjelom korisnika

**Rješenja**:
- Provjerite dodjelu korisnika aplikaciji FastComments
- Provjerite jesu li uloge aplikacije ispravno konfigurirane
- Osigurajte da mapiranje tvrdnji uključuje potrebne atribute

#### Okta

**Uobičajeni problemi**:
- Filteri grupa ne rade ispravno
- Pogrešno konfigurirane izjave atributa
- Problemi s dodjelom aplikacije

**Rješenja**:
- Pregledajte konfiguraciju izjava atributa
- Provjerite pravila dodjele grupa i filtriranja
- Provjerite je li aplikacija dodijeljena odgovarajućim korisnicima/grupama

#### Google Workspace

**Uobičajeni problemi**:
- Prilagođeni atributi se ne mapiraju ispravno
- Članstvo u grupama se ne šalje
- Pogreške u konfiguraciji SAML aplikacije

**Rješenja**:
- Konfigurirajte prilagođenu shemu za atribute uloga
- Provjerite propagaciju članstva u grupama
- Provjerite mapiranje atributa SAML aplikacije

### Problemi s mrežom i povezivošću

#### Pogreške isteka vremena

**Simptomi**:
- Proces autentifikacije istječe
- Pogreške "Request timeout" ili slične
- Spor tijek autentifikacije

**Rješenja**:
1. **Mrežna povezanost**:
   - Provjerite pravila vatrozida koja dopuštaju komunikaciju s FastComments
   - Provjerite DNS razrješavanje za fastcomments.com
   - Testirajte mrežnu povezanost od IdP-a do FastComments

2. **Problemi s performansama**:
   - Provjerite vrijeme odgovora IdP-a
   - Provjerite da provjera lanca certifikata nije spora
   - Razmotrite mrežnu latenciju između IdP-a i korisnika

#### SSL/TLS problemi

**Simptomi**:
- Upozorenja o certifikatima tijekom autentifikacije
- Neuspjeli SSL handshake
- Pogreške "Secure connection failed"

**Rješenja**:
- Osigurajte da sve SAML krajnje točke koriste HTTPS
- Provjerite valjanost certifikata za sve uključene domene
- Provjerite kompatibilnost TLS verzije

### Otklanjanje pogrešaka i zapisivanje

#### Omogućavanje informacija za otklanjanje pogrešaka

1. **Razvojni alati preglednika**:
   - Pratite karticu Network tijekom SAML toka
   - Provjerite konzolu za JavaScript pogreške
   - Istražite SAML POST zahtjeve (ako su vidljivi)

2. **IdP zapisivanje**:
   - Omogućite SAML otklanjanje pogrešaka u vašem IdP-u
   - Pregledajte IdP zapise za detalje SAML zahtjeva/odgovora
   - Provjerite postoje li problemi s mapiranjem atributa

#### Uobičajene poruke zapisa

**FastComments zapisi**:
- "SAML config not found" - SAML nije omogućen ili je pogrešno konfiguriran
- "Invalid certificate" - Provjera certifikata nije uspjela
- "Missing email attribute" - Potrebna e-pošta nije pružena u SAML odgovoru

**IdP zapisi**:
- "Unknown service provider" - Nepodudaranje Entity ID-a
- "Invalid ACS URL" - URL Assertion Consumer Service-a je neispravan
- "User not assigned" - Korisnik nema pristup SAML aplikaciji

### Traženje pomoći

#### Informacije koje treba prikupiti

Kada kontaktirate podršku, pružite:
- Točne poruke o pogreškama i vremenske oznake
- Detalje SAML konfiguracije (bez osjetljivih podataka)
- Vrstu i verziju IdP-a
- Korake za reprodukciju problema
- Informacije o pregledniku i mreži

#### FastComments podrška

Za probleme vezane uz SAML:
1. Koristite [portal za podršku](https://fastcomments.com/auth/my-account/help)
2. Uključite tenant ID i e-poštu pogođenih korisnika
3. Pružite poruke o pogreškama i detalje konfiguracije
4. Navedite vrstu IdP-a i pristup konfiguraciji

#### IdP podrška

Za probleme specifične za IdP:
- Posavjetujte se s IdP dokumentacijom za rješavanje SAML problema
- Koristite IdP kanale podrške za probleme s konfiguracijom
- Iskoristite IdP forumne zajednice za uobičajene probleme

### Savjeti za prevenciju

#### Najbolje prakse

1. **Temeljito testirajte**:
   - Testirajte promjene konfiguracije u neprodukcijskom okruženju
   - Provjerite s više testnih korisnika
   - Dokumentirajte ispravne konfiguracije

2. **Redovito pratite**:
   - Postavite praćenje neuspjeha SAML autentifikacije
   - Pregledajte datume isteka certifikata
   - Pratite promjene IdP konfiguracije

3. **Dokumentacija**:
   - Održavajte dokumentaciju SAML konfiguracije
   - Dokumentirajte sve prilagođene konfiguracije ili zaobilazna rješenja
   - Čuvajte kontakt informacije IdP administratora

#### Proaktivno održavanje

1. **Upravljanje certifikatima**:
   - Pratite datume isteka certifikata
   - Planirajte postupke rotacije certifikata
   - Testirajte ažuriranja certifikata prije isteka

2. **Pregledi konfiguracije**:
   - Redovito pregledavajte SAML konfiguraciju
   - Provjerite je li IdP konfiguracija i dalje aktualna
   - Ažurirajte dokumentaciju kako se mijene prave