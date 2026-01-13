Efter at have konfigureret SAML i FastComments, skal du konfigurere FastComments som en tjenesteudbyder i din identitetsudbyder.

### Generel IdP-konfiguration

De fleste identitetsudbydere kræver følgende oplysninger for at tilføje FastComments som en SAML-applikation:

#### Påkrævet tjenesteudbyderinformation

Disse værdier genereres automatisk og vises på din FastComments SAML-konfigurationsside:

**SP Entity ID / Audience**
- Format: `https://fastcomments.com/saml/{your-tenant-id}`
- Dette identificerer entydigt din FastComments-forekomst

**Assertion Consumer Service (ACS) URL**
- Format: `https://fastcomments.com/saml/callback/{your-tenant-id}`
- Hvor din IdP sender SAML-svar efter autentificering

**SP Metadata URL** *(hvis understøttet af din IdP)*
- Format: `https://fastcomments.com/saml/metadata/{your-tenant-id}`
- Leverer komplet SAML-konfiguration i XML-format

**SAML Login URL**
- Format: `https://fastcomments.com/saml/login/{your-tenant-id}`
- Direkte link til at igangsætte SAML-autentificering

### Påkrævede SAML-attributter

Konfigurer din identitetsudbyder til at sende disse attributter med SAML-svar:

#### Væsentlige attributter

**Email Address** *(Påkrævet)*
- **Attribute Name**: `email`, `emailAddress`, or `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/emailaddress`
- **Formål**: Entydig brugeridentifikation og meddelelser
- **Format**: Gyldig e-mailadresse

#### Valgfrie attributter

**First Name**
- **Attribute Names**: `firstName`, `givenName`, or `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/givenname`
- **Formål**: Brugernavn der vises

**Last Name**
- **Attribute Names**: `lastName`, `surname`, or `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/surname`
- **Formål**: Brugernavn der vises

**Roles** *(Vigtigt for adgangskontrol)*
- **Attribute Names**: `roles`, `groups`, `memberOf`, or custom attribute names
- **Formål**: Tildeling af FastComments-roller og tilladelser
- **Format**: Array af rolletekster eller kommaseparerede værdier

### Almindelige identitetsudbyderkonfigurationer

#### Microsoft Azure AD

1. **Tilføj Enterprise Application**
   - Søg efter "FastComments" eller opret en brugerdefineret SAML-applikation
   - Brug SP-oplysningerne leveret af FastComments

2. **Konfigurer attributter**
   - Email: `user.mail` or `user.userprincipalname`
   - First Name: `user.givenname`
   - Last Name: `user.surname`
   - Roles: `user.assignedroles` or directory groups

#### Okta

1. **Opret SAML-applikation**
   - Brug "Create New App" og vælg SAML 2.0
   - Konfigurer med FastComments SP-oplysninger

2. **Attribute Statements**
   - Email: `user.email`
   - FirstName: `user.firstName`
   - LastName: `user.lastName`
   - Roles: `user.groups` or custom attributes

#### Google Workspace

1. **Tilføj SAML-applikation**
   - Gå til Apps > Web and mobile apps > Add App > Add custom SAML app
   - Konfigurer med FastComments SP-oplysninger

2. **Attributkortlægning**
   - Email: Primary email
   - First Name: First name
   - Last Name: Last name
   - Roles: Groups or custom attributes

#### Active Directory Federation Services (ADFS)

1. **Tilføj Relying Party Trust**
   - Brug FastComments-metadata-URL'en eller manuel konfiguration
   - Konfigurer SP-oplysninger som angivet

2. **Claim Rules**
   - Email: Email Address claim
   - Name: Name ID claim
   - Roles: Gruppemedlemskab eller brugerdefinerede claims

### Fleksibilitet i attributnavne

FastComments accepterer rolleinformation fra flere attributnavne for at imødekomme forskellige IdP-konfigurationer:

- `roles`
- `groups`
- `memberOf`
- `role`
- `group`
- `http://schemas.microsoft.com/ws/2008/06/identity/claims/role`
- `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/role`

Denne fleksibilitet sikrer kompatibilitet med forskellige identitetsudbydere uden at kræve specifikke navngivningskonventioner for attributter.

### Test af din konfiguration

Efter at have konfigureret din identitetsudbyder:

1. Gem IdP-konfigurationen
2. Test med en dedikeret testbruger
3. Bekræft, at attributter sendes korrekt
4. Kontroller, at roller er korrekt kortlagt
5. Sørg for, at autentificeringsflowet fuldføres succesfuldt

De fleste identitetsudbydere tilbyder SAML-testværktøjer til at validere konfigurationen, før du ruller ud til produktionsbrugere.