Nakon konfiguriranja SAML-a u FastCommentsu, morate postaviti FastComments kao pružatelja usluge (Service Provider) u svom pružatelju identiteta.

### Opća konfiguracija IdP-a

Većina pružatelja identiteta zahtijeva sljedeće informacije za dodavanje FastComments-a kao SAML aplikacije:

#### Obvezne informacije o pružatelju usluge

Ove vrijednosti se automatski generiraju i prikazuju na stranici za SAML konfiguraciju u FastCommentsu:

**SP Entity ID / Audience**
- Format: `https://fastcomments.com/saml/{your-tenant-id}`
- Ovo jedinstveno identificira vašu FastComments instancu

**Assertion Consumer Service (ACS) URL**
- Format: `https://fastcomments.com/saml/callback/{your-tenant-id}`
- Gdje vaš IdP šalje SAML odgovore nakon autentikacije

**SP Metadata URL** *(if supported by your IdP)*
- Format: `https://fastcomments.com/saml/metadata/{your-tenant-id}`
- Pruža kompletnu SAML konfiguraciju u XML formatu

**SAML Login URL**
- Format: `https://fastcomments.com/saml/login/{your-tenant-id}`
- Izravan link za iniciranje SAML autentikacije

### Obavezni SAML atributi

Konfigurirajte svog pružatelja identiteta da šalje ove atribute sa SAML odgovorima:

#### Osnovni atributi

**Adresa e-pošte** *(obavezno)*
- **Naziv atributa**: `email`, `emailAddress`, or `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/emailaddress`
- **Svrha**: Jedinstvena identifikacija korisnika i obavijesti
- **Format**: Valjana adresa e-pošte

#### Neobavezni atributi

**Ime**
- **Nazivi atributa**: `firstName`, `givenName`, or `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/givenname`
- **Svrha**: Prikazno ime korisnika

**Prezime**
- **Nazivi atributa**: `lastName`, `surname`, or `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/surname`
- **Svrha**: Prikazno ime korisnika

**Uloge** *(važno za kontrolu pristupa)*
- **Nazivi atributa**: `roles`, `groups`, `memberOf`, or custom attribute names
- **Svrha**: Dodjela uloga i dozvola u FastCommentsu
- **Format**: Niz stringova uloga ili vrijednosti odvojenih zarezom

### Uobičajene konfiguracije pružatelja identiteta

#### Microsoft Azure AD

1. **Dodajte Enterprise aplikaciju**
   - Pretražite "FastComments" ili stvorite prilagođenu SAML aplikaciju
   - Upotrijebite SP informacije koje pruža FastComments

2. **Konfigurirajte atribute**
   - E-pošta: `user.mail` or `user.userprincipalname`
   - Ime: `user.givenname`
   - Prezime: `user.surname`
   - Uloge: `user.assignedroles` or directory groups

#### Okta

1. **Stvorite SAML aplikaciju**
   - Upotrijebite "Create New App" i odaberite SAML 2.0
   - Konfigurirajte koristeći SP informacije iz FastCommentsa

2. **Izjave atributa**
   - Email: `user.email`
   - FirstName: `user.firstName`
   - LastName: `user.lastName`
   - Roles: `user.groups` or custom attributes

#### Google Workspace

1. **Dodajte SAML aplikaciju**
   - Idite na Apps > Web and mobile apps > Add App > Add custom SAML app
   - Konfigurirajte koristeći SP informacije iz FastCommentsa

2. **Mapiranje atributa**
   - Email: Primary email
   - First Name: First name
   - Last Name: Last name
   - Roles: Groups or custom attributes

#### Active Directory Federation Services (ADFS)

1. **Dodajte Relying Party Trust**
   - Koristite FastComments metadata URL ili ručnu konfiguraciju
   - Konfigurirajte SP informacije kako je navedeno

2. **Pravila tvrdnji**
   - Email: Email Address claim
   - Name: Name ID claim
   - Roles: Group membership or custom claims

### Fleksibilnost naziva atributa

FastComments prihvaća informacije o ulogama iz više naziva atributa kako bi prilagodio različite IdP konfiguracije:

- `roles`
- `groups`
- `memberOf`
- `role`
- `group`
- `http://schemas.microsoft.com/ws/2008/06/identity/claims/role`
- `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/role`

Ova fleksibilnost osigurava kompatibilnost s različitim pružateljima identiteta bez potrebe za određenim konvencijama imenovanja atributa.

### Testiranje vaše konfiguracije

Nakon konfiguriranja vašeg pružatelja identiteta:

1. Spremite IdP konfiguraciju
2. Testirajte s namjenskim test korisničkim računom
3. Potvrdite da se atributi pravilno šalju
4. Provjerite da su uloge pravilno mapirane
5. Osigurajte da se autentikacijski tok uspješno dovrši

Većina pružatelja identiteta nudi SAML alate za testiranje kako bi se konfiguracija verificirala prije uvođenja u produkciju.