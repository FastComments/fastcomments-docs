Ovaj vodič pokriva česte SAML probleme sa autentifikacijom i njihova rešenja.

### Certificate and Security Issues

#### Invalid Certificate Error

**Symptoms**:
- Greška "Certificate validation failed"
- Korisnici ne mogu da završe SAML autentifikaciju
- SAML odgovori se odbacuju

**Common Causes**:
- Format sertifikata je neispravan
- Sertifikat je istekao
- Dat je pogrešan sertifikat
- Dodatni karakteri ili razmaci u sertifikatu

**Solutions**:
1. **Verify Certificate Format**:
   - Uverite se da sertifikat sadrži markere `-----BEGIN CERTIFICATE-----` i `-----END CERTIFICATE-----`
   - Uklonite sve dodatne razmake ili prekide linija
   - Kopirajte sertifikat direktno iz IdP metadata ili konfiguracije

2. **Check Certificate Validity**:
   - Proverite da sertifikat nije istekao
   - Potvrdite da je sertifikat za ispravnog IdP-a
   - Koristite online validatore sertifikata da proverite format

3. **Re-download Certificate**:
   - Preuzmite novi sertifikat sa IdP-a
   - Koristite IdP metadata URL ako je dostupan
   - Potvrdite da sertifikat odgovara trenutnoj IdP konfiguraciji

#### Signature Verification Failed

**Symptoms**:
- Greške pri verifikaciji potpisa SAML asertacije
- Autentifikacija ne uspeva nakon IdP prijave
- Poruke o grešci "Invalid signature"

**Solutions**:
1. **Algorithm Mismatch**:
   - Proverite da algoritam potpisa u FastComments odgovara onom kod IdP-a
   - Isprobajte različite algoritme potpisa (SHA-256, SHA-1, SHA-512)
   - Verifikujte da digest algoritam odgovara IdP konfiguraciji

2. **Certificate Issues**:
   - Uverite se da je ispravan sertifikat za potpis konfigurisan
   - Proverite da sertifikat odgovara privatnom ključu koji koristi IdP
   - Proverite da li je došlo do rotacije sertifikata u IdP-u

### Configuration Issues

#### Wrong Entity ID or ACS URL

**Symptoms**:
- IdP prijavljuje "Unknown Service Provider"
- SAML odgovori idu na pogrešan endpoint
- Autentifikacija se ne završava

**Solutions**:
1. **Verify SP Information**:
   - Kopirajte tačan Entity ID iz FastComments konfiguracije
   - Uverite se da ACS URL odgovara formatu: `https://fastcomments.com/saml/callback/{tenant-id}`
   - Proverite pravopis tenant ID-a

2. **IdP Configuration**:
   - Ažurirajte IdP sa tačnim SP Entity ID-jem
   - Konfigurišite ispravan ACS/Reply URL
   - Verifikujte IdP postavke binding-a (HTTP-POST je preporučen)

#### Missing or Incorrect Attributes

**Symptoms**:
- Korisnici se kreiraju bez odgovarajućih uloga
- Nedostaju informacije profila korisnika
- Greške "Email required"

**Solutions**:
1. **Email Attribute**:
   - Uverite se da IdP šalje atribut email
   - Proverite mapiranje imena atributa (email, emailAddress, itd.)
   - Verifikujte da je vrednost email validna email adresa

2. **Role Attributes**:
   - Potvrdite da IdP šalje informacije o ulogama/grupama
   - Proverite da imena atributa uloga odgovaraju FastComments očekivanjima
   - Verifikujte da vrednosti uloga tačno odgovaraju imenima uloga u FastComments

3. **Attribute Format**:
   - Testirajte i nizove i formate uloga razdvojene zarezima
   - Uverite se da vrednosti atributa nemaju dodatne razmake
   - Proverite osetljivost na velika/mala slova u imenima uloga

### Authentication Flow Issues

#### Redirect Loop

**Symptoms**:
- Pregledač beskonačno preusmerava između FastComments i IdP-a
- Autentifikacija se nikada ne završava
- Više preusmeravanja prikazano u alatkama za razvoj pregledača

**Solutions**:
1. **Check SP Configuration**:
   - Proverite da Entity ID tačno odgovara IdP konfiguraciji
   - Uverite se da je ACS URL ispravno konfigurisan u IdP-u
   - Proverite da li postoje završne kosе crte u URL-ovima

2. **Session Issues**:
   - Očistite kolačiće pregledača i pokušajte ponovo
   - Testirajte u incognito/privatnom prozoru pregledača
   - Proverite podešavanja isteka sesije

#### Access Denied After Authentication

**Symptoms**:
- SAML autentifikacija uspeva
- Korisnik je preusmeren na FastComments
- Prikazuje se "Access denied" ili greška sa dozvolama

**Solutions**:
1. **Role Assignment**:
   - Proverite da korisnik ima odgovarajuće uloge u IdP-u
   - Proverite da li se atribut uloge šalje u SAML odgovoru
   - Potvrdite da imena uloga tačno odgovaraju zahtevima FastComments

2. **Package Limitations**:
   - Proverite da nalog ima Flex ili Pro plan
   - Proverite da li je SAML funkcija omogućena za taj plan
   - Kontaktirajte podršku ako paket uključuje SAML, ali funkcija nije dostupna

### Identity Provider Specific Issues

#### Microsoft Azure AD

**Common Issues**:
- Dodela uloga aplikaciji se ne odražava u tokenima
- Klaimovi se ne šalju ispravno
- Zahtevi za dodelu korisnika

**Solutions**:
- Proverite dodelu korisnika FastComments aplikaciji
- Verifikujte da su app uloge ispravno konfigurisane
- Uverite se da mapiranje klaimova uključuje potrebne atribute

#### Okta

**Common Issues**:
- Filteri grupa ne rade ispravno
- Izjave o atributima su pogrešno konfigurisane
- Problemi sa dodelom aplikacije

**Solutions**:
- Pregledajte konfiguraciju izjave o atributima
- Proverite dodelu grupa i pravila filtriranja
- Verifikujte da je aplikacija dodeljena odgovarajućim korisnicima/grupama

#### Google Workspace

**Common Issues**:
- Prilagođeni atributi se ne mapiraju ispravno
- Članstvo u grupi se ne šalje
- Greške u konfiguraciji SAML aplikacije

**Solutions**:
- Konfigurišite prilagođenu šemu za atribute uloga
- Proverite propagaciju članstva u grupama
- Verifikujte mapiranje atributa SAML aplikacije

### Network and Connectivity Issues

#### Timeout Errors

**Symptoms**:
- Proces autentifikacije ističe vreme
- Greške "Request timeout" ili slične
- Spori tok autentifikacije

**Solutions**:
1. **Network Connectivity**:
   - Proverite firewall pravila da omogućavaju komunikaciju sa FastComments
   - Verifikujte DNS rezoluciju za fastcomments.com
   - Testirajte mrežnu povezanost od IdP-a do FastComments

2. **Performance Issues**:
   - Proverite vremena odgovora IdP-a
   - Verifikujte da validacija lanca sertifikata nije spora
   - Razmotrite mrežnu latenciju između IdP-a i korisnika

#### SSL/TLS Issues

**Symptoms**:
- Upozorenja o sertifikatima tokom autentifikacije
- Neuspešne SSL rukovanja
- Greške "Secure connection failed"

**Solutions**:
- Uverite se da svi SAML endpoint-i koriste HTTPS
- Proverite validnost sertifikata za sve uključene domene
- Verifikujte kompatibilnost TLS verzije

### Debugging and Logging

#### Enabling Debug Information

1. **Browser Developer Tools**:
   - Pratite karticu Network tokom SAML toka
   - Proverite Console za JavaScript greške
   - Ispitajte SAML POST zahteve (ako su vidljivi)

2. **IdP Logging**:
   - Omogućite SAML debug u vašem IdP-u
   - Pregledajte IdP logove za detalje SAML zahteva/odgovora
   - Proverite probleme sa mapiranjem atributa

#### Common Log Messages

**FastComments Logs**:
- "SAML config not found" - SAML nije omogućen ili je pogrešno konfigurisan
- "Invalid certificate" - Validacija sertifikata nije uspela
- "Missing email attribute" - Obavezni email nije dostavljen u SAML odgovoru

**IdP Logs**:
- "Unknown service provider" - Entity ID se ne poklapa
- "Invalid ACS URL" - Assertion Consumer Service URL je netačan
- "User not assigned" - Korisnik nema pristup SAML aplikaciji

### Getting Help

#### Information to Gather

Kada kontaktirate podršku, priložite:
- Tačne poruke o grešci i vremenske oznake
- Detalje SAML konfiguracije (bez osetljivih podataka)
- Tip i verziju IdP-a
- Korake za reprodukciju problema
- Informacije o pregledaču i mreži

#### FastComments Support

Za SAML povezane probleme:
1. Koristite [support portal](https://fastcomments.com/auth/my-account/help)
2. Uključite tenant ID i email adrese pogođenih korisnika
3. Dostavite poruke o greškama i detalje konfiguracije
4. Navedite tip IdP-a i pristup konfiguraciji

#### IdP Support

Za probleme specifične za IdP:
- Konsultujte IdP dokumentaciju za rešavanje SAML problema
- Koristite IdP kanale podrške za probleme sa konfiguracijom
- Iskoristite IdP zajedničke forume za česte probleme

### Prevention Tips

#### Best Practices

1. **Test Thoroughly**:
   - Testirajte promene konfiguracije u neprodukcijskom okruženju
   - Verifikujte sa više test korisnika
   - Dokumentujte radne konfiguracije

2. **Monitor Regularly**:
   - Postavite nadzor za SAML neuspehe autentifikacije
   - Pregledajte datume isteka sertifikata
   - Pratite promene u IdP konfiguraciji

3. **Documentation**:
   - Održavajte dokumentaciju SAML konfiguracije
   - Dokumentujte sve prilagođene konfiguracije ili privremena rešenja
   - Čuvajte kontakt informacije administratora IdP-a

#### Proactive Maintenance

1. **Certificate Management**:
   - Pratite datume isteka sertifikata
   - Planirajte procedure rotacije sertifikata
   - Testirajte ažuriranja sertifikata pre isteka

2. **Configuration Reviews**:
   - Redovno pregledajte SAML konfiguraciju
   - Verifikujte da IdP konfiguracija ostane aktuelna
   - Ažurirajte dokumentaciju kad se izvrše promene