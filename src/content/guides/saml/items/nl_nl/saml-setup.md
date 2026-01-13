Het instellen van SAML-authenticatie in FastComments vereist zowel configuratie in uw beheerdersdashboard als configuratie bij uw identiteitsprovider.

### Vereisten

Voordat u SAML configureert, zorgt u ervoor dat u:

- Een FastComments Flex- of Pro-plan (SAML is niet beschikbaar op het Creators-plan)
- Administratieve toegang tot uw FastComments-account
- Administratieve toegang tot uw identiteitsprovider
- De SAML-metadata of certificaatgegevens van uw IdP

### Toegang tot SAML-configuratie

1. Log in op uw [FastComments-beheerdersdashboard](https://fastcomments.com/auth/my-account)
2. Navigeer naar **API/SSO-instellingen** in de linkerzijbalk
3. Klik op de knop **SAML-configuratie**

Als u de knop SAML-configuratie niet ziet, controleer dan of:
- Uw account het vereiste pakket heeft (Flex of Pro)
- U administratieve rechten heeft
- Uw gebruiker de rollen API Admin of Admin Admin heeft

### Basis SAML-configuratie

#### SAML-authenticatie inschakelen

1. Vink het selectievakje **SAML-authenticatie inschakelen** aan
2. Hiermee activeert u SAML voor uw tenant en worden de configuratievelden beschikbaar

#### Vereiste velden

**IdP Single Sign-On URL** *(Vereist)*
- De URL waar gebruikers naartoe worden omgeleid voor authenticatie
- Wordt meestal door uw identiteitsprovider geleverd
- Example: `https://your-company.okta.com/app/fastcomments/sso/saml`

**IdP X.509 Certificate** *(Vereist)*
- Het openbare certificaat van uw identiteitsprovider
- Wordt gebruikt om de authenticiteit van SAML-responses te verifiëren
- Moet het volledige certificaat met BEGIN/END-markeringen bevatten
- Example format:
```
-----BEGIN CERTIFICATE-----
MIICXjCCAcegAwIBAgIBADANBgkqhkiG9w0BAQsFADA...
-----END CERTIFICATE-----
```

#### Optionele velden

**IdP Entity ID / Issuer**
- Identificeert uw identiteitsprovider
- Als het leeg blijft, wordt standaard uw FastComments-URL gebruikt
- Moet overeenkomen met de issuer die in uw IdP is geconfigureerd

### Geavanceerde configuratie

#### Beveiligingsinstellingen

**Signature Algorithm**
- Standaard SHA-256 (aanbevolen)
- Opties: SHA-1, SHA-256, SHA-512
- Moet overeenkomen met de configuratie van uw IdP

**Digest Algorithm**
- Standaard SHA-256 (aanbevolen)
- Wordt gebruikt voor de digest-berekening in SAML-responses
- Moet overeenkomen met de configuratie van uw IdP

**Name ID Format**
- Standaard: e-mailadresformaat
- Bepaalt hoe gebruikersidentificaties worden opgemaakt
- Veelvoorkomende opties: Email Address, Persistent, Transient

#### Encryptie (optioneel)

**Private Key for Decryption**
- Alleen nodig als uw IdP SAML-asserties versleutelt
- Plak uw privésleutel die wordt gebruikt voor ontsleuteling
- De meeste implementaties vereisen geen encryptie van assertions

### Configuratie opslaan

1. Controleer alle instellingen op juistheid
2. Klik op **SAML-configuratie opslaan**
3. Het systeem zal uw configuratie valideren
4. Als het succesvol is, ziet u een bevestigingsbericht

### Volgende stappen

Nadat u uw FastComments SAML-configuratie hebt opgeslagen:

1. Configureer uw identiteitsprovider met behulp van de Service Provider-informatie
2. Test de authenticatiestroom
3. Stel gebruikersrollen en machtigingen in indien nodig

De Service Provider-informatie die nodig is voor de configuratie van uw IdP wordt weergegeven zodra SAML is ingeschakeld.