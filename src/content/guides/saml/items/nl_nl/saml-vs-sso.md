FastComments biedt zowel SSO- als SAML-authenticatie. Het begrijpen van de verschillen helpt u de juiste aanpak voor uw organisatie te kiezen.

### Eenvoudige/Beveiligde SSO-implementaties

FastComments biedt twee verschillende SSO-flows om in te loggen in de commentaarwidget via uw site.
Dit is anders dan SAML en vereist geen SAML. In plaats daarvan vereist Simple SSO simpelweg het doorgeven van een object aan
de commentaarwidget, waarbij Secure SSO dit doet plus het hashen van de payload met een API-sleutel.

### SAML-authenticatie

SAML is een authenticatieprotocol op ondernemingsniveau dat robuustere beveiligings- en integratiemogelijkheden biedt:

- **Implementation**: Vereist Identity Provider (IdP)-configuratie en certificaatuitwisseling
- **Security**: Maakt gebruik van ondertekende XML-asserties en ondersteunt versleuteling
- **Use Case**: Ideaal voor ondernemingen met bestaande SAML-infrastructuur (Active Directory, Okta, enz.)
- **Setup Complexity**: Meer complex - vereist IdP-configuratie en certificaatbeheer
- **Enterprise Features**: Geavanceerde roltoewijzing, gecentraliseerd gebruikersbeheer, auditsporen

### Wanneer SAML kiezen

Overweeg SAML-authenticatie als uw organisatie:

- Gebruikt al een SAML-compatibele identity provider (Okta, Azure AD, ADFS, enz.)
- Vereist beveiliging en compliance op ondernemingsniveau
- Heeft gecentraliseerd gebruikersbeheer en toegangscontrole nodig
- Heeft meerdere applicaties die SAML voor authenticatie gebruiken
- Vereist gedetailleerde auditsporen en beveiligingsrapportage

### Wanneer Simple of Secure SSO kiezen

Onze op widgets gerichte SSO-oplossingen kunnen voldoende zijn als u:

- Een aangepast authenticatiesysteem heeft
- Snelle implementatie met minimale configuratie nodig heeft
- Geen integratie met een enterprise identity provider vereist
- Gebruikersgegevens direct vanuit uw applicatie wilt beheren
- Eenvoudigere beveiligingseisen heeft

Simple and Secure SSO worden vaak gebruikt voor online portals, blogs, enz., waar de gebruiker al een account heeft *via uw site of app*
maar niet noodzakelijk SAML gebruikt.