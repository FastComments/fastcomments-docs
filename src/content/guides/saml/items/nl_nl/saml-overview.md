SAML (Security Assertion Markup Language) is een op XML gebaseerde open standaard voor het uitwisselen van authenticatie- en autorisatiegegevens tussen partijen, met name tussen een identiteitsprovider (IdP) en een serviceprovider (SP).

### Hoe SAML werkt

SAML maakt single sign-on (SSO) mogelijk door gebruikers één keer te laten authenticeren bij hun identiteitsprovider en daarna toegang te geven tot meerdere applicaties zonder opnieuw inloggegevens in te voeren. Wanneer een gebruiker probeert toegang te krijgen tot FastComments:

1. **Authentication Request**: FastComments stuurt de gebruiker door naar uw identiteitsprovider
2. **User Authentication**: De gebruiker authenticeert zich bij uw IdP (bijv. Active Directory, Okta, Azure AD)
3. **SAML Response**: De IdP stuurt een ondertekende SAML-assertion terug naar FastComments
4. **User Access**: FastComments valideert de assertion en verleent toegang aan de geauthenticeerde gebruiker

### Voordelen van SAML

- **Verbeterde beveiliging**: Gecentraliseerde authenticatie vermindert aan wachtwoorden gerelateerde beveiligingsrisico's
- **Verbeterde gebruikerservaring**: Gebruikers melden zich één keer aan en krijgen naadloos toegang tot meerdere applicaties
- **Naleving**: Helpt te voldoen aan regelgevende vereisten voor toegangscontrole en auditsporen
- **Administratieve controle**: IT-beheerders behouden gecentraliseerd gebruikersbeheer

### SAML 2.0-ondersteuning

FastComments implementeert SAML 2.0, de meest gebruikte versie van de SAML-standaard. Onze implementatie ondersteunt:

- HTTP-POST- en HTTP-Redirect-bindingen
- Ondertekende SAML-responses en assertions
- Versleutelde assertions (optioneel)
- Meerdere handtekening- en digest-algoritmen
- Verschillende name identifier-formaten