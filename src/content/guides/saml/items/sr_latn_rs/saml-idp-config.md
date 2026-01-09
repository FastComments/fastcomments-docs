Nakon konfiguracije SAML-a u FastComments, potrebno je da podesite FastComments kao Service Provider u vašem pružaocu identiteta.

### Opšta konfiguracija IdP-a

Većina pružalaca identiteta zahteva sledeće informacije da bi dodali FastComments kao SAML aplikaciju:

#### Obavezne informacije o Service Provider-u

Ove vrednosti se automatski generišu i prikazuju na stranici za SAML konfiguraciju u FastComments:

**SP Entity ID / Audience**
- Format: `https://fastcomments.com/saml/{your-tenant-id}`
- Ovo jedinstveno identifikuje vašu FastComments instancu

**Assertion Consumer Service (ACS) URL**
- Format: `https://fastcomments.com/saml/callback/{your-tenant-id}`
- Gde vaš IdP šalje SAML odgovore nakon autentifikacije

**SP Metadata URL** *(ako vaš IdP to podržava)*
- Format: `https://fastcomments.com/saml/metadata/{your-tenant-id}`
- Pruža kompletnu SAML konfiguraciju u XML formatu

**SAML Login URL**
- Format: `https://fastcomments.com/saml/login/{your-tenant-id}`
- Direktan link za pokretanje SAML autentifikacije

### Obavezni SAML atributi

Konfigurišite vašeg pružaoca identiteta da šalje ove atribute sa SAML odgovorima:

#### Osnovni atributi

**Email Address** *(Obavezno)*
- **Attribute Name**: `email`, `emailAddress`, or `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/emailaddress`
- **Svrha**: Jedinstvena identifikacija korisnika i notifikacije
- **Format**: Validna email adresa

#### Opcioni atributi

**First Name**
- **Attribute Names**: `firstName`, `givenName`, or `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/givenname`
- **Svrha**: Prikazno ime korisnika

**Last Name**
- **Attribute Names**: `lastName`, `surname`, or `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/surname`
- **Svrha**: Prikazno ime korisnika

**Roles** *(Važno za kontrolu pristupa)*
- **Attribute Names**: `roles`, `groups`, `memberOf`, or custom attribute names
- **Svrha**: Dodeljivanje FastComments uloga i dozvola
- **Format**: Niz stringova uloga ili vrednosti odvojenih zarezom

### Uobičajene konfiguracije pružalaca identiteta

#### Microsoft Azure AD

1. **Dodajte Enterprise aplikaciju**
   - Pretražite "FastComments" ili kreirajte prilagođenu SAML aplikaciju
   - Koristite SP informacije koje obezbeđuje FastComments

2. **Konfigurišite atribute**
   - Email: `user.mail` or `user.userprincipalname`
   - First Name: `user.givenname`
   - Last Name: `user.surname`
   - Roles: `user.assignedroles` or directory groups

#### Okta

1. **Kreirajte SAML aplikaciju**
   - Koristite "Create New App" i izaberite SAML 2.0
   - Konfigurišite sa FastComments SP informacijama

2. **Attribute Statements**
   - Email: `user.email`
   - FirstName: `user.firstName`
   - LastName: `user.lastName`
   - Roles: `user.groups` or custom attributes

#### Google Workspace

1. **Dodajte SAML aplikaciju**
   - Idite na Apps > Web and mobile apps > Add App > Add custom SAML app
   - Konfigurišite sa FastComments SP informacijama

2. **Attribute Mapping**
   - Email: Primary email
   - First Name: First name
   - Last Name: Last name
   - Roles: Groups or custom attributes

#### Active Directory Federation Services (ADFS)

1. **Dodajte Relying Party Trust**
   - Koristite FastComments metadata URL ili ručnu konfiguraciju
   - Konfigurišite SP informacije kako su navedene

2. **Pravila zahteva**
   - Email: Email Address claim
   - Name: Name ID claim
   - Roles: Group membership or custom claims

### Fleksibilnost imena atributa

FastComments prihvata informacije o ulogama iz više naziva atributa kako bi podržao različite konfiguracije IdP-a:

- `roles`
- `groups`
- `memberOf`
- `role`
- `group`
- `http://schemas.microsoft.com/ws/2008/06/identity/claims/role`
- `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/role`

Ova fleksibilnost obezbeđuje kompatibilnost sa raznim pružaocima identiteta bez potrebe za specifičnim konvencijama imenovanja atributa.

### Testiranje vaše konfiguracije

Nakon konfiguracije vašeg pružaoca identiteta:

1. Sačuvajte IdP konfiguraciju
2. Testirajte sa namenskim test korisničkim nalogom
3. Proverite da li se atributi pravilno šalju
4. Proverite da li su uloge pravilno mapirane
5. Osigurajte da tok autentifikacije uspešno završava

Većina pružalaca identiteta nudi SAML alate za testiranje kako bi se validirala konfiguracija pre primene na produkcione korisnike.