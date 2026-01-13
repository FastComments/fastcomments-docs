Ovaj vodič pokriva uobičajene probleme sa SAML autentifikacijom i njihova rešenja.

### Problemi sa sertifikatima i bezbednošću

#### Greška nevažećeg sertifikata

**Simptomi**:
- "Certificate validation failed" greška
- Korisnici ne mogu da završe SAML autentifikaciju
- SAML odgovori se odbijaju

**Uobičajeni uzroci**:
- Format sertifikata nije ispravan
- Sertifikat je istekao
- Pogrešan sertifikat je dostavljen
- Dodatni karakteri ili razmaci u sertifikatu

**Rešenja**:
1. **Proverite format sertifikata**:
   - Uverite se da sertifikat sadrži `-----BEGIN CERTIFICATE-----` i `-----END CERTIFICATE-----` markere
   - Uklonite bilo koji dodatni razmak ili prelome linija
   - Kopirajte sertifikat direktno iz IdP metapodataka ili konfiguracije

2. **Proverite važenje sertifikata**:
   - Proverite da sertifikat nije istekao
   - Potvrdite da je sertifikat za odgovarajući IdP
   - Koristite online validatora sertifikata za proveru formata

3. **Ponovo preuzmite sertifikat**:
   - Preuzmite svež sertifikat sa IdP-a
   - Koristite IdP metadata URL ako je dostupan
   - Potvrdite da sertifikat odgovara trenutnoj IdP konfiguraciji

#### Verifikacija potpisa nije uspela

**Simptomi**:
- Greške pri validaciji potpisa SAML assertion-a
- Autentifikacija ne uspeva nakon prijave na IdP
- Poruke o grešci "Invalid signature"

**Rešenja**:
1. **Neslaganje algoritama**:
   - Proverite da li potpisni algoritam u FastComments odgovara IdP-u
   - Probajte različite algoritme potpisa (SHA-256, SHA-1, SHA-512)
   - Verifikujte da digest algoritam odgovara IdP konfiguraciji

2. **Problemi sa sertifikatom**:
   - Uverite se da je ispravan sertifikat za potpisivanje konfigurisan
   - Proverite da li sertifikat odgovara privatnom ključu koji koristi IdP
   - Proverite da li je došlo do rotacije sertifikata u IdP

### Problemi sa konfiguracijom

#### Pogrešan Entity ID ili ACS URL

**Simptomi**:
- IdP prijavljuje "Unknown Service Provider"
- SAML odgovori idu na pogrešan endpoint
- Autentifikacija se ne kompletira

**Rešenja**:
1. **Proverite informacije o SP-u**:
   - Kopirajte tačan Entity ID iz FastComments konfiguracije
   - Uverite se da ACS URL odgovara formatu: `https://fastcomments.com/saml/callback/{tenant-id}`
   - Proverite da nema tipografskih grešaka u tenant ID

2. **Konfiguracija IdP-a**:
   - Ažurirajte IdP sa tačnim SP Entity ID-jem
   - Konfigurišite ispravan ACS/Reply URL
   - Proverite IdP binding podešavanja (HTTP-POST je poželjan)

#### Nedostaju ili netačni atributi

**Simptomi**:
- Korisnici se kreiraju bez odgovarajućih uloga
- Nedostaju informacije profila korisnika
- Greške "Email required"

**Rešenja**:
1. **Atribut email**:
   - Uverite se da IdP šalje atribut email
   - Proverite mapiranje imenа atributa (email, emailAddress, itd.)
   - Verifikujte da je vrednost email validna email adresa

2. **Atributi uloga**:
   - Potvrdite da IdP šalje informacije o ulozi/grupi
   - Proverite da imena atributa uloga odgovaraju FastComments očekivanjima
   - Verifikujte da vrednosti uloga tačno odgovaraju imenima uloga u FastComments

3. **Format atributa**:
   - Testirajte i niz i format razdvojen zarezima za uloge
   - Uverite se da vrednosti atributa nemaju dodatne razmake
   - Proverite osetljivost na velika/mala slova u imenima uloga

### Problemi u toku autentifikacionog toka

#### Petlja preusmeravanja

**Simptomi**:
- Pregledač se beskonačno preusmerava između FastComments i IdP
- Autentifikacija se nikada ne završava
- Više preusmeravanja prikazano u alatima za razvoj pregledača

**Rešenja**:
1. **Proverite SP konfiguraciju**:
   - Verifikujte da Entity ID tačno odgovara IdP konfiguraciji
   - Uverite se da je ACS URL pravilno konfigurisan u IdP
   - Proverite za završne kosе crte u URL-ovima

2. **Problemi sa sesijom**:
   - Očistite kolačiće pregledača i pokušajte ponovo
   - Testirajte u incognito/privatnom prozoru pregledača
   - Proverite podešavanja isteka sesije

#### Pristup odbijen nakon autentifikacije

**Simptomi**:
- SAML autentifikacija uspeva
- Korisnik je preusmeren na FastComments
- Prikazuje se "Access denied" ili greška vezana za dozvole

**Rešenja**:
1. **Dodela uloga**:
   - Proverite da korisnik ima odgovarajuće uloge u IdP
   - Proverite da li se atribut uloge šalje u SAML odgovoru
   - Potvrdite da imena uloga tačno odgovaraju zahtevima FastComments

2. **Ograničenja paketa**:
   - Verifikujte da nalog ima Flex ili Pro plan
   - Proverite da li je SAML funkcija omogućena za paket
   - Kontaktirajte podršku ako paket uključuje SAML, ali funkcija nije dostupna

### Problemi specifični za provajdere identiteta

#### Microsoft Azure AD

**Uobičajeni problemi**:
- Dodela app role se ne reflektuje u tokenima
- Claims se ne šalju pravilno
- Zahtevi za dodelu korisnika

**Rešenja**:
- Proverite dodelu korisnika FastComments aplikaciji
- Verifikujte da su app role ispravno konfigurisane
- Uverite se da mapiranje claims uključuje neophodne atribute

#### Okta

**Uobičajeni problemi**:
- Filteri grupa ne rade ispravno
- Attribute statements su pogrešno konfigurisani
- Problemi sa dodelom aplikacije

**Rešenja**:
- Pregledajte konfiguraciju attribute statement-a
- Proverite dodelu grupa i pravila filtriranja
- Verifikujte da je aplikacija dodeljena odgovarajućim korisnicima/grupama

#### Google Workspace

**Uobičajeni problemi**:
- Prilagođeni atributi se ne mapiraju ispravno
- Članstvo u grupama se ne šalje
- Greške u konfiguraciji SAML aplikacije

**Rešenja**:
- Konfigurišite prilagođenu šemu za atribute uloga
- Proverite propagaciju članstva u grupama
- Verifikujte mapiranje atributa SAML aplikacije

### Mrežni i konektivni problemi

#### Greške isteka vremena

**Simptomi**:
- Proces autentifikacije ističe
- "Request timeout" ili slične greške
- Spor tok autentifikacije

**Rešenja**:
1. **Mrežna konektivnost**:
   - Proverite firewall pravila koja dozvoljavaju komunikaciju sa FastComments
   - Verifikujte DNS rezoluciju za fastcomments.com
   - Testirajte mrežnu konektivnost od IdP do FastComments

2. **Problemi sa performansama**:
   - Proverite vreme odgovora IdP-a
   - Verifikujte da validacija lanca sertifikata nije spora
   - Razmotrite latenciju mreže između IdP-a i korisnika

#### Problemi sa SSL/TLS

**Simptomi**:
- Upozorenja o sertifikatu tokom autentifikacije
- Neuspeh SSL handshake-a
- Greške "Secure connection failed"

**Rešenja**:
- Uverite se da svi SAML endpoint-i koriste HTTPS
- Proverite validnost sertifikata za sve uključene domene
- Verifikujte kompatibilnost verzije TLS-a

### Otklanjanje grešaka i logovanje

#### Omogućavanje informacija za otklanjanje grešaka

1. **Alati za programere u pregledaču**:
   - Pratite Network tab tokom SAML toka
   - Proverite Console za JavaScript greške
   - Ispitajte SAML POST zahteve (ako su vidljivi)

2. **Logovanje IdP-a**:
   - Omogućite SAML debagovanje u svom IdP-u
   - Pregledajte IdP logove za detalje o SAML zahtevima/odgovorima
   - Proverite probleme u mapiranju atributa

#### Uobičajene poruke u logovima

**FastComments logovi**:
- "SAML config not found" - SAML nije omogućen ili je pogrešno konfigurisan
- "Invalid certificate" - Validacija sertifikata nije uspela
- "Missing email attribute" - Obavezan email nije prosleđen u SAML odgovoru

**IdP logovi**:
- "Unknown service provider" - Neusklađenost Entity ID-a
- "Invalid ACS URL" - Assertion Consumer Service URL je netačan
- "User not assigned" - Korisnik nema pristup SAML aplikaciji

### Dobijanje pomoći

#### Informacije koje treba prikupiti

Prilikom kontaktiranja podrške, obezbedite:
- Tačne poruke o grešci i vremenske oznake
- Detalje o SAML konfiguraciji (bez osetljivih podataka)
- Tip i verziju IdP-a
- Korake za reprodukciju problema
- Informacije o pregledaču i mreži

#### FastComments podrška

Za probleme vezane za SAML:
1. Koristite [support portal](https://fastcomments.com/auth/my-account/help)
2. Uključite tenant ID i email adrese pogođenih korisnika
3. Obezbedite poruke o grešci i detalje konfiguracije
4. Navedite tip IdP-a i pristup konfiguraciji

#### Podrška IdP-a

Za probleme specifične za IdP:
- Konsultujte dokumentaciju IdP-a za SAML rešavanje problema
- Koristite kanale podrške IdP-a za probleme konfiguracije
- Iskoristite forume zajednice IdP-a za uobičajene probleme

### Saveti za prevenciju

#### Najbolje prakse

1. **Temeljno testirajte**:
   - Testirajte promene konfiguracije u neprodukcijonom okruženju
   - Verifikujte sa više test korisnika
   - Dokumentujte radne konfiguracije

2. **Redovno nadgledajte**:
   - Postavite nadzor za neuspehe SAML autentifikacije
   - Pregledajte datume isteka sertifikata
   - Nadgledajte promene u IdP konfiguraciji

3. **Dokumentacija**:
   - Održavajte dokumentaciju SAML konfiguracije
   - Dokumentujte sve prilagođene konfiguracije ili zaobilazna rešenja
   - Čuvajte kontakt informacije za administratore IdP-a

#### Proaktivno održavanje

1. **Upravljanje sertifikatima**:
   - Nadgledajte datume isteka sertifikata
   - Planirajte procedure rotacije sertifikata
   - Testirajte ažuriranja sertifikata pre isteka

2. **Pregledi konfiguracije**:
   - Redovno pregledajte SAML konfiguraciju
   - Verifikujte da IdP konfiguracija ostaje ažurna
   - Ažurirajte dokumentaciju kako se promene vrše