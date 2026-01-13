Po konfiguraciji SAML v FastComments morate v svojem ponudniku identitete (IdP) nastaviti FastComments kot ponudnika storitev.

### Splošna konfiguracija IdP

Večina ponudnikov identitete (IdP) zahteva naslednje informacije za dodajanje FastComments kot SAML aplikacije:

#### Zahtevane informacije o ponudniku storitev

Te vrednosti so samodejno ustvarjene in prikazane na strani za SAML konfiguracijo v FastComments:

**SP Entity ID / Ciljna publika**
- Format: `https://fastcomments.com/saml/{your-tenant-id}`
- To enolično identificira vašo FastComments instanco

**Assertion Consumer Service (ACS) URL**
- Format: `https://fastcomments.com/saml/callback/{your-tenant-id}`
- Kam vaš IdP pošlje SAML odgovore po overitvi

**SP Metadata URL** *(če ga vaš IdP podpira)*
- Format: `https://fastcomments.com/saml/metadata/{your-tenant-id}`
- Ponuja celotno SAML konfiguracijo v XML formatu

**SAML Login URL**
- Format: `https://fastcomments.com/saml/login/{your-tenant-id}`
- Neposredna povezava za začetek SAML overjanja

### Zahtevani SAML atributi

Konfigurirajte svoj ponudnik identitete, da pošilja te atribute z SAML odgovori:

#### Bistveni atributi

**E-poštni naslov** *(zahtevano)*
- **Ime atributa**: `email`, `emailAddress`, or `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/emailaddress`
- **Namen**: Enolična identifikacija uporabnika in obvestila
- **Format**: Veljaven e-poštni naslov

#### Neobvezni atributi

**Ime**
- **Imena atributov**: `firstName`, `givenName`, or `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/givenname`
- **Namen**: Prikazno ime uporabnika

**Priimek**
- **Imena atributov**: `lastName`, `surname`, or `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/surname`
- **Namen**: Prikazno ime uporabnika

**Vloge** *(pomembno za nadzor dostopa)*
- **Imena atributov**: `roles`, `groups`, `memberOf`, or custom attribute names
- **Namen**: Dodeljevanje vlog in dovoljenj v FastComments
- **Format**: Polje nizov vlog ali vrednosti, ločenih z vejico

### Pogoste konfiguracije ponudnikov identitete

#### Microsoft Azure AD

1. **Dodajte Enterprise Application**
   - Poiščite "FastComments" ali ustvarite prilagojeno SAML aplikacijo
   - Uporabite SP informacije, ki jih zagotavlja FastComments

2. **Konfigurirajte atribute**
   - Email: `user.mail` or `user.userprincipalname`
   - First Name: `user.givenname`
   - Last Name: `user.surname`
   - Roles: `user.assignedroles` or directory groups

#### Okta

1. **Ustvarite SAML aplikacijo**
   - Uporabite "Create New App" in izberite SAML 2.0
   - Konfigurirajte s SP informacijami FastComments

2. **Izjave o atributih**
   - E-pošta: `user.email`
   - Ime: `user.firstName`
   - Priimek: `user.lastName`
   - Vloge: `user.groups` or custom attributes

#### Google Workspace

1. **Dodajte SAML aplikacijo**
   - Pojdite v Apps > Web and mobile apps > Add App > Add custom SAML app
   - Konfigurirajte s SP informacijami FastComments

2. **Preslikava atributov**
   - E-pošta: Primary email
   - Ime: First name
   - Priimek: Last name
   - Vloge: Groups or custom attributes

#### Active Directory Federation Services (ADFS)

1. **Dodajte Relying Party Trust**
   - Uporabite FastComments metadata URL ali ročno konfiguracijo
   - Konfigurirajte SP informacije, kot so podane

2. **Pravila zahtevkov**
   - Email: Email Address claim
   - Ime: Name ID claim
   - Vloge: Članstvo v skupinah ali prilagojeni zahtevki

### Fleksibilnost imen atributov

FastComments sprejema informacije o vlogah iz več imen atributov, da se prilagodi različnim konfiguracijam IdP:

- `roles`
- `groups`
- `memberOf`
- `role`
- `group`
- `http://schemas.microsoft.com/ws/2008/06/identity/claims/role`
- `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/role`

Ta prilagodljivost zagotavlja združljivost z različnimi ponudniki identitete, brez potrebe po specifičnih konvencijah poimenovanja atributov.

### Preizkušanje vaše konfiguracije

Po konfiguraciji ponudnika identitete:

1. Shranite IdP konfiguracijo
2. Preizkusite z namenskim testnim uporabniškim računom
3. Preverite, ali se atributi pravilno pošiljajo
4. Preverite, ali so vloge pravilno preslikane
5. Zagotovite, da se postopek overjanja uspešno zaključi

Večina ponudnikov identitete ponuja orodja za testiranje SAML, s katerimi lahko preverite konfiguracijo, preden jo uvedete za produkcijske uporabnike.