Wanneer SAML is ingeschakeld in FastComments, genereert het systeem automatisch Service Provider (SP)-informatie die u moet configureren in uw identiteitsprovider.

### Toegang tot Service Provider-informatie

De SP-informatie wordt weergegeven op uw SAML-configuratiepagina nadat SAML-authenticatie is ingeschakeld. Deze informatie bevat alle details die uw identiteitsprovider nodig heeft om de SAML-trustrelatie tot stand te brengen.

### Service Provider-eindpunten

#### SP Entity ID / Audience
**Doel**: Identificeert uw FastComments-instantie als serviceprovider
**Formaat**: `https://fastcomments.com/saml/{your-tenant-id}`
**Gebruik**: Configureer dit als de Entity ID of Audience in uw IdP

Deze identifier zorgt ervoor dat SAML-antwoorden bedoeld zijn voor uw specifieke FastComments-tenant en voorkomt dat SAML-antwoorden door andere instanties worden geaccepteerd.

#### Assertion Consumer Service (ACS) URL
**Doel**: Het eindpunt waar uw IdP SAML-antwoorden na gebruikersauthenticatie naartoe stuurt
**Formaat**: `https://fastcomments.com/saml/callback/{your-tenant-id}`
**Gebruik**: Configureer dit als de ACS-URL of Reply URL in uw IdP

Hier worden gebruikers na succesvolle authenticatie bij uw identiteitsprovider naartoe doorgestuurd, samen met de SAML-assertion die gebruikersinformatie bevat.

#### SP Metadata URL
**Doel**: Biedt volledige SAML-configuratie in standaard XML-formaat
**Formaat**: `https://fastcomments.com/saml/metadata/{your-tenant-id}`
**Gebruik**: Sommige IdP's kunnen configuratie automatisch importeren met deze URL

De metadata-URL bevat alle benodigde SP-informatie in XML-formaat, wat het eenvoudig maakt om compatibele identiteitsproviders automatisch te configureren.

#### SAML Login URL
**Doel**: Directe link om SAML-authenticatie voor uw tenant te starten
**Formaat**: `https://fastcomments.com/saml/login/{your-tenant-id}`
**Gebruik**: Leid gebruikers rechtstreeks naar SAML-authenticatie of test de flow

U kunt deze URL gebruiken om SAML-authenticatie te testen of gebruikers een directe link te geven om zich via SAML aan te melden.

### Ondersteunde SAML-bindings

FastComments ondersteunt de volgende SAML-bindings:

#### HTTP-POST-binding
- **Primaire methode**: Meest voorkomende binding voor SAML-antwoorden
- **Beveiliging**: SAML-antwoord wordt via HTTP POST naar de ACS-URL verzonden
- **Gebruik**: Aanbevolen voor productie-implementaties

#### HTTP-Redirect-binding
- **Alternatieve methode**: SAML-antwoord verzonden via HTTP-redirect
- **Beperkingen**: Beperkte payloadgrootte door URL-lengtebeperkingen
- **Gebruik**: Ondersteund maar HTTP-POST heeft de voorkeur

### Name ID-beleid

FastComments configureert het volgende Name ID-beleid in SAML-verzoeken:

- **Standaardformaat**: `urn:oasis:names:tc:SAML:1.1:nameid-format:emailAddress`
- **Alternatieve formaten**: Persistent, Transient, Unspecified (configureerbaar)
- **Vereiste**: Het e-mailadres wordt gebruikt als de primaire gebruikersidentificator

### SAML-verzoekattributen

Bij het initiëren van SAML-authenticatie stuurt FastComments verzoeken met deze eigenschappen:

#### Ondertekening van verzoeken
- **Status**: Optioneel (configureerbaar)
- **Algoritme**: Komt overeen met het geconfigureerde handtekeningalgoritme
- **Certificaat**: Gebruikt tenant-specifiek certificaat als ondertekening van verzoeken is ingeschakeld

#### Gevraagde attributen
FastComments vraagt de volgende attributen op in SAML AuthnRequests:

- **Email**: Vereist voor gebruikersidentificatie
- **First Name**: Optioneel voor weergave
- **Last Name**: Optioneel voor weergave
- **Roles/Groups**: Optioneel voor toegangscontrole en permissies

### SP-informatie kopiëren

De SAML-configuratiepagina biedt klikbare velden die SP-informatie automatisch naar uw klembord kopiëren:

1. Klik op een SP-informatief veld (Entity ID, ACS URL, enz.)
2. De waarde wordt automatisch naar uw klembord gekopieerd
3. Plak de waarde in de configuratie van uw identiteitsprovider
4. Een korte markering geeft aan dat het kopiëren succesvol was

Dit maakt het eenvoudig om de SP-informatie nauwkeurig over te dragen naar uw IdP zonder typefouten.

### SP-certificaata informatie

#### Gebruik van certificaten
- **Doel**: Versleutelt communicatie en verifieert SP-identiteit
- **Rotatie**: Certificaten worden automatisch beheerd door FastComments
- **Toegang**: Publieke certificaten zijn beschikbaar via de metadata-URL

#### Certificaatdetails
- **Algoritme**: RSA-2048 of hoger
- **Geldigheid**: Certificaten worden automatisch vernieuwd vóór expiratie
- **Distributie**: Beschikbaar via standaard SAML-metadata

### Problemen oplossen bij SP-configuratie

Als uw identiteitsprovider problemen meldt met SP-informatie:

1. **Controleer URL's**: Zorg dat alle URL's HTTPS gebruiken en het juiste tenant-ID bevatten
2. **Controleer metadata**: Gebruik de metadata-URL om de configuratie te verifiëren
3. **Test connectiviteit**: Zorg dat uw IdP de FastComments-eindpunten kan bereiken
4. **Valideer formaat**: Bevestig dat uw IdP het SP-informatieformaat ondersteunt

Veelvoorkomende problemen zijn onder andere:
- Onjuist tenant-ID in URL's
- Netwerkconnectiviteitsproblemen tussen IdP en FastComments
- IdP verwacht andere URL-formaten of aanvullende configuratieopties