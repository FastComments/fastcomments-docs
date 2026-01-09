Ovaj vodič pokriva uobičajene probleme s SAML autentikacijom i njihova rješenja.

### Certificate and Security Issues

#### Invalid Certificate Error

**Symptoms**:
- "Certificate validation failed" greška
- Korisnici ne mogu dovršiti SAML autentikaciju
- SAML odgovori se odbijaju

**Common Causes**:
- Format certifikata je neispravan
- Certifikat je istekao
- Pružен je pogrešan certifikat
- Dodatni znakovi ili razmaci u certifikatu

**Solutions**:
1. **Verify Certificate Format**:
   - Osigurajte da certifikat sadrži markere `-----BEGIN CERTIFICATE-----` i `-----END CERTIFICATE-----`
   - Uklonite dodatne razmake ili prijelome linija
   - Kopirajte certifikat izravno iz IdP metapodataka ili konfiguracije

2. **Check Certificate Validity**:
   - Provjerite da certifikat nije istekao
   - Potvrdite da je certifikat za ispravan IdP
   - Koristite online validatore certifikata za provjeru formata

3. **Re-download Certificate**:
   - Preuzmite svježi certifikat s IdP-a
   - Koristite IdP URL metapodataka ako je dostupan
   - Potvrdite da certifikat odgovara trenutnoj IdP konfiguraciji

#### Signature Verification Failed

**Symptoms**:
- Pogreške pri provjeri potpisa SAML asercije
- Autentikacija ne uspijeva nakon prijave na IdP
- Poruke o pogrešci "Invalid signature"

**Solutions**:
1. **Algorithm Mismatch**:
   - Provjerite da algoritam potpisa u FastComments odgovara IdP-u
   - Isprobajte različite algoritme potpisa (SHA-256, SHA-1, SHA-512)
   - Provjerite da digest algoritam odgovara IdP konfiguraciji

2. **Certificate Issues**:
   - Osigurajte da je konfiguriran ispravan certifikat za potpisivanje
   - Potvrdite da certifikat odgovara privatnom ključu koji koristi IdP
   - Provjerite rotaciju certifikata u IdP-u

### Configuration Issues

#### Wrong Entity ID or ACS URL

**Symptoms**:
- IdP prijavljuje "Unknown Service Provider"
- SAML odgovori idu na pogrešni endpoint
- Autentikacija se ne dovrši

**Solutions**:
1. **Verify SP Information**:
   - Kopirajte točan Entity ID iz FastComments konfiguracije
   - Osigurajte da ACS URL odgovara formatu: `https://fastcomments.com/saml/callback/{tenant-id}`
   - Provjerite tipfeler u tenant ID-u

2. **IdP Configuration**:
   - Ažurirajte IdP s točnim SP Entity ID-jem
   - Konfigurirajte ispravan ACS/Reply URL
   - Provjerite IdP binding postavke (preferira se HTTP-POST)

#### Missing or Incorrect Attributes

**Symptoms**:
- Korisnici se kreiraju bez odgovarajućih uloga
- Nedostaju informacije u korisničkom profilu
- Greške "Email required"

**Solutions**:
1. **Email Attribute**:
   - Osigurajte da IdP šalje atribut email
   - Provjerite mapiranje naziva atributa (email, emailAddress, itd.)
   - Potvrdite da je vrijednost emaila valjana adresa e-pošte

2. **Role Attributes**:
   - Potvrdite da IdP šalje informacije o ulogama/grupama
   - Provjerite da nazivi atributa uloga odgovaraju očekivanjima FastComments-a
   - Provjerite da vrijednosti uloga točno odgovaraju imenima uloga u FastComments-u

3. **Attribute Format**:
   - Testirajte i niz (array) i formate s vrijednostima odvojenim zarezom za uloge
   - Osigurajte da vrijednosti atributa nemaju dodatne razmake
   - Provjerite osjetljivost na velika/mala slova u nazivima uloga

### Authentication Flow Issues

#### Redirect Loop

**Symptoms**:
- Preglednik se beskrajno preusmjerava između FastComments-a i IdP-a
- Autentikacija nikada ne završi
- Višestruka preusmjeravanja se prikazuju u alatima za razvoj preglednika

**Solutions**:
1. **Check SP Configuration**:
   - Provjerite da Entity ID točno odgovara IdP konfiguraciji
   - Osigurajte da je ACS URL ispravno konfiguriran u IdP-u
   - Provjerite završne kosine (trailing slashes) u URL-ovima

2. **Session Issues**:
   - Očistite kolačiće preglednika i pokušajte ponovno
   - Testirajte u incognito/privatnom prozoru preglednika
   - Provjerite postavke isteka sesije

#### Access Denied After Authentication

**Symptoms**:
- SAML autentikacija uspijeva
- Korisnik je preusmjeren na FastComments
- Prikazuje se poruka "Access denied" ili greška s dozvolama

**Solutions**:
1. **Role Assignment**:
   - Provjerite ima li korisnik odgovarajuće uloge u IdP-u
   - Provjerite šalje li se atribut uloge u SAML odgovoru
   - Potvrdite da nazivi uloga točno odgovaraju zahtjevima FastComments-a

2. **Package Limitations**:
   - Provjerite ima li račun Flex ili Pro plan
   - Provjerite je li SAML značajka omogućena za paket
   - Kontaktirajte podršku ako paket uključuje SAML, ali značajka nije dostupna

### Identity Provider Specific Issues

#### Microsoft Azure AD

**Common Issues**:
- Dodjele uloga aplikaciji se ne odražavaju u tokenima
- Claimovi se ne šalju ispravno
- Zahtjevi za dodjelu korisnika

**Solutions**:
- Provjerite dodjelu korisnika aplikaciji FastComments
- Provjerite jesu li app role pravilno konfigurirane
- Osigurajte da mapiranje claimova uključuje potrebne atribute

#### Okta

**Common Issues**:
- Filtri grupa ne rade ispravno
- Atributne izjave su krivo konfigurirane
- Problemi s dodjelom aplikacije

**Solutions**:
- Pregledajte konfiguraciju atributne izjave
- Provjerite pravila dodjele i filtriranja grupa
- Potvrdite da je aplikacija dodijeljena odgovarajućim korisnicima/grupama

#### Google Workspace

**Common Issues**:
- Prilagođeni atributi se ne mapiraju ispravno
- Članstvo u grupama se ne šalje
- Pogreške u konfiguraciji SAML aplikacije

**Solutions**:
- Konfigurirajte prilagođenu shemu za atribute uloga
- Provjerite propagaciju članstva u grupama
- Potvrdite mapiranje atributa SAML aplikacije

### Network and Connectivity Issues

#### Timeout Errors

**Symptoms**:
- Proces autentikacije ističe
- "Request timeout" ili slične pogreške
- Spori tijek autentikacije

**Solutions**:
1. **Network Connectivity**:
   - Provjerite firewall pravila da dopuštaju komunikaciju s FastComments-om
   - Potvrdite DNS rezoluciju za fastcomments.com
   - Testirajte mrežnu povezanost s IdP-a prema FastComments-u

2. **Performance Issues**:
   - Provjerite vrijeme odgovora IdP-a
   - Potvrdite da provjera lanca certifikata nije spora
   - Razmotrite mrežnu latenciju između IdP-a i korisnika

#### SSL/TLS Issues

**Symptoms**:
- Upozorenja o certifikatu tijekom autentikacije
- Neuspjesi SSL rukovanja (handshake)
- Poruke "Secure connection failed"

**Solutions**:
- Osigurajte da svi SAML endpointi koriste HTTPS
- Provjerite valjanost certifikata za sve uključene domene
- Potvrdite kompatibilnost verzije TLS-a

### Debugging and Logging

#### Enabling Debug Information

1. **Browser Developer Tools**:
   - Pratite Network karticu tijekom SAML tijeka
   - Provjerite Console za JavaScript pogreške
   - Ispitajte SAML POST zahtjeve (ako su vidljivi)

2. **IdP Logging**:
   - Omogućite SAML debug u vašem IdP-u
   - Pregledajte IdP logove za detalje SAML zahtjeva/odgovora
   - Provjerite probleme s mapiranjem atributa

#### Common Log Messages

**FastComments Logs**:
- "SAML config not found" - SAML nije omogućen ili je krivo konfiguriran
- "Invalid certificate" - Validacija certifikata nije uspjela
- "Missing email attribute" - Obavezni email nije poslan u SAML odgovoru

**IdP Logs**:
- "Unknown service provider" - Neusklađenost Entity ID-a
- "Invalid ACS URL" - Assertion Consumer Service URL je netočan
- "User not assigned" - Korisnik nema pristup SAML aplikaciji

### Getting Help

#### Information to Gather

Prilikom kontaktiranja podrške, dostavite:
- Točne poruke o pogrešci i vremenske oznake
- Detalje SAML konfiguracije (bez osjetljivih podataka)
- Tip i verziju IdP-a
- Korake za reprodukciju problema
- Informacije o pregledniku i mreži

#### FastComments Support

Za SAML povezane probleme:
1. Koristite [support portal](https://fastcomments.com/auth/my-account/help)
2. Uključite tenant ID i e-mail adrese pogođenih korisnika
3. Dostavite poruke o pogrešci i detalje konfiguracije
4. Navedite tip IdP-a i pristup konfiguraciji

#### IdP Support

Za probleme specifične za IdP:
- Konzultirajte IdP dokumentaciju za rješavanje SAML problema
- Koristite IdP kanale podrške za probleme s konfiguracijom
- Iskoristite IdP community forume za uobičajene probleme

### Prevention Tips

#### Best Practices

1. **Test Thoroughly**:
   - Testirajte promjene konfiguracije u neprodukcijskom okruženju
   - Provjerite s više testnih korisnika
   - Dokumentirajte radne konfiguracije

2. **Monitor Regularly**:
   - Postavite nadzor za SAML neuspjehe autentikacije
   - Pregledavajte datume isteka certifikata
   - Pratite promjene u IdP konfiguraciji

3. **Documentation**:
   - Održavajte dokumentaciju SAML konfiguracije
   - Dokumentirajte sve prilagođene konfiguracije ili zaobilazna rješenja
   - Čuvajte kontakt informacije administratora IdP-a

#### Proactive Maintenance

1. **Certificate Management**:
   - Pratite datume isteka certifikata
   - Planirajte postupke rotacije certifikata
   - Testirajte ažuriranja certifikata prije isteka

2. **Configuration Reviews**:
   - Redovito pregledavajte SAML konfiguraciju
   - Potvrdite da IdP konfiguracija ostaje ažurirana
   - Ažurirajte dokumentaciju kad se naprave promjene