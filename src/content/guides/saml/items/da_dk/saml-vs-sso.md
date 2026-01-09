FastComments tilbyder både SSO- og SAML-godkendelse. At forstå forskellene hjælper dig med at vælge den rigtige tilgang for din organisation.

### Simple/Secure SSO-løsninger

FastComments tilbyder to forskellige SSO-flow til autentificering i kommentar-widgetten via dit websted.
Dette er forskelligt fra SAML og kræver ikke SAML. I stedet kræver Simple SSO blot at sende et objekt til
kommentar-widgetten, hvor Secure SSO gør dette plus hasher payloaden med en API-nøgle.

SAML, derimod, autentificerer brugeren til hele produktet (baseret på deres tilladelser) *as well as* kommentar-widgetten
(hvis de har cookies fra tredjepart aktiveret for vores domæne).

### SAML-godkendelse

SAML er en enterprise-klasses godkendelsesprotokol, der giver mere robust sikkerhed og integrationsmuligheder:

- **Implementation**: Kræver Identity Provider (IdP)-konfiguration og certifikatudveksling
- **Security**: Bruger signerede XML-assertions og understøtter kryptering
- **Use Case**: Ideel til virksomheder med eksisterende SAML-infrastruktur (Active Directory, Okta, osv.)
- **Setup Complexity**: Mere involveret - kræver IdP-konfiguration og certifikatstyring
- **Enterprise Features**: Avanceret kortlægning af roller, centraliseret brugeradministration, revisionsspor

### Hvornår du skal vælge SAML

Overvej SAML-godkendelse, hvis din organisation:

- Bruger allerede en SAML-kompatibel identitetsudbyder (Okta, Azure AD, ADFS osv.)
- Kræver enterprise-grade sikkerhed og overholdelse
- Har brug for centraliseret brugeradministration og adgangskontrol
- Har flere applikationer, der bruger SAML til godkendelse
- Kræver detaljerede revisionsspor og sikkerhedsrapporter

### Hvornår du skal vælge Simple eller Secure SSO

Vores widget-centrerede SSO-løsninger kan være tilstrækkelige, hvis du:

- Har et tilpasset autentificeringssystem
- Har brug for hurtig implementering med minimal opsætning
- Ikke har behov for integration med enterprise identity providers
- Ønsker at kontrollere brugerdata direkte fra din applikation
- Har enklere sikkerhedskrav

Simple og Secure SSO bruges ofte til onlineportaler, blogs osv., hvor brugeren allerede har en konto *gennem dit websted eller din app*
men ikke nødvendigvis bruger SAML.