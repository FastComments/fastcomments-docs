Nadat u SAML in FastComments hebt geconfigureerd, moet u FastComments instellen als Service Provider in uw identiteitsprovider.

### Algemene IdP-configuratie

De meeste identiteitsproviders vereisen de volgende informatie om FastComments toe te voegen als een SAML-applicatie:

#### Vereiste Service Provider-informatie

Deze waarden worden automatisch gegenereerd en weergegeven op uw FastComments SAML-configuratiepagina:

**SP Entity ID / Audience**
- Format: `https://fastcomments.com/saml/{your-tenant-id}`
- Dit identificeert uw FastComments-instantie op unieke wijze

**Assertion Consumer Service (ACS) URL**
- Format: `https://fastcomments.com/saml/callback/{your-tenant-id}`
- Waar uw IdP SAML-responses na authenticatie naartoe stuurt

**SP Metadata URL** *(indien ondersteund door uw IdP)*
- Format: `https://fastcomments.com/saml/metadata/{your-tenant-id}`
- Biedt volledige SAML-configuratie in XML-formaat

**SAML Login URL**
- Format: `https://fastcomments.com/saml/login/{your-tenant-id}`
- Directe link om SAML-authenticatie te starten

### Vereiste SAML-attributen

Configureer uw identiteitsprovider zodat deze deze attributen meestuurt in SAML-responses:

#### Essentiële attributen

**E-mailadres** *(Vereist)*
- **Attribuurnaam**: `email`, `emailAddress`, or `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/emailaddress`
- **Doel**: Unieke gebruikersidentificatie en meldingen
- **Formaat**: Geldig e-mailadres

#### Optionele attributen

**Voornaam**
- **Attribuurnamen**: `firstName`, `givenName`, or `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/givenname`
- **Doel**: Weergavenaam van de gebruiker

**Achternaam**
- **Attribuurnamen**: `lastName`, `surname`, or `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/surname`
- **Doel**: Weergavenaam van de gebruiker

**Rollen** *(Belangrijk voor toegangscontrole)*
- **Attribuurnamen**: `roles`, `groups`, `memberOf`, or custom attribute names
- **Doel**: Toewijzing van FastComments-rollen en machtigingen
- **Formaat**: Array van rol-strings of door komma's gescheiden waarden

### Veelvoorkomende configuraties van identiteitsproviders

#### Microsoft Azure AD

1. **Enterprise-applicatie toevoegen**
   - Zoek naar "FastComments" of maak een aangepaste SAML-applicatie
   - Gebruik de SP-informatie die door FastComments wordt geleverd

2. **Configureer attributen**
   - E-mail: `user.mail` of `user.userprincipalname`
   - Voornaam: `user.givenname`
   - Achternaam: `user.surname`
   - Rollen: `user.assignedroles` of groepen uit de directory

#### Okta

1. **Maak SAML-app aan**
   - Gebruik "Create New App" en selecteer SAML 2.0
   - Configureer met FastComments SP-informatie

2. **Attribuutverklaringen**
   - E-mail: `user.email`
   - Voornaam: `user.firstName`
   - Achternaam: `user.lastName`
   - Rollen: `user.groups` of aangepaste attributen

#### Google Workspace

1. **Voeg SAML-applicatie toe**
   - Ga naar Apps > Web- en mobiele apps > App toevoegen > Aangepaste SAML-app toevoegen
   - Configureer met FastComments SP-informatie

2. **Attribuuttoewijzing**
   - E-mail: Primair e-mailadres
   - Voornaam: Voornaam
   - Achternaam: Achternaam
   - Rollen: Groepen of aangepaste attributen

#### Active Directory Federation Services (ADFS)

1. **Relying Party Trust toevoegen**
   - Gebruik de FastComments-metadata-URL of handmatige configuratie
   - Configureer SP-informatie zoals geleverd

2. **Claim-regels**
   - E-mail: E-mailadres-claim
   - Naam: Name ID-claim
   - Rollen: Groepslidmaatschap of aangepaste claims

### Flexibiliteit van attribuutnamen

FastComments accepteert rolinformatie van meerdere attribuutnamen om verschillende IdP-configuraties te ondersteunen:

- `roles`
- `groups`
- `memberOf`
- `role`
- `group`
- `http://schemas.microsoft.com/ws/2008/06/identity/claims/role`
- `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/role`

Deze flexibiliteit zorgt voor compatibiliteit met verschillende identiteitsproviders zonder specifieke naamgevingsconventies voor attributen te vereisen.

### Het testen van uw configuratie

Na het configureren van uw identiteitsprovider:

1. Sla de IdP-configuratie op
2. Test met een speciale testgebruikersaccount
3. Controleer of attributen correct worden verzonden
4. Controleer of rollen correct zijn toegewezen
5. Zorg dat de authenticatiestroom succesvol wordt voltooid

De meeste identiteitsproviders bieden SAML-testtools om de configuratie te valideren voordat u deze voor productgebruikers in gebruik neemt.