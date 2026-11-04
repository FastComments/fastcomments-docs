Wanneer SAML is ingeschakeld in FastComments, genereert het systeem automatisch Service Provider (SP)-informatie die je moet configureren in je identity provider.

### Toegang tot Service Provider-informatie

De SP-informatie wordt weergegeven op je SAML-configuratiepagina na het inschakelen van SAML-authenticatie. Deze informatie bevat alle details die je identity provider nodig heeft om de SAML-vertrouwensrelatie tot stand te brengen.

### Service Provider Endpoints

#### SP Entity ID / Audience
**Doel**: Identificeert je FastComments-instantie op unieke wijze als service provider
**Formaat**: `https://fastcomments.com/saml/{your-tenant-id}`
**Gebruik**: Configureer dit als de Entity ID of Audience in je IdP

Deze identifier zorgt ervoor dat SAML-responses bedoeld zijn voor jouw specifieke FastComments-tenant en voorkomt dat SAML-responses door andere instanties worden geaccepteerd.

#### Assertion Consumer Service (ACS) URL
**Doel**: Het endpoint waar je IdP SAML-responses naartoe stuurt na gebruikersauthenticatie
**Formaat**: `https://fastcomments.com/saml/callback/{your-tenant-id}`
**Gebruik**: Configureer dit als de ACS URL of Reply URL in je IdP

Hier worden gebruikers naartoe geleid na succesvolle authenticatie bij je identity provider, samen met de SAML-assertion die gebruikersinformatie bevat.

#### SP Metadata URL
**Doel**: Biedt complete SAML-configuratie in standaard XML-formaat
**Formaat**: `https://fastcomments.com/saml/metadata/{your-tenant-id}`
**Gebruik**: Sommige IdP's kunnen de configuratie automatisch importeren via deze URL

De metadata-URL bevat alle benodigde SP-informatie in XML-formaat, waardoor het eenvoudig is om compatibele identity providers automatisch te configureren.

#### SAML Login URL
**Doel**: Directe link om SAML-authenticatie voor je tenant te starten
**Formaat**: `https://fastcomments.com/saml/login/{your-tenant-id}`
**Gebruik**: Link gebruikers direct naar SAML-authenticatie of test de flow

Je kunt deze URL gebruiken om SAML-authenticatie te testen of gebruikers een directe link te geven om in te loggen via SAML.

### Ondersteuning voor SAML-bindings

FastComments ondersteunt de volgende SAML-bindings:

#### HTTP-POST Binding
- **Primaire methode**: Meest gebruikte binding voor SAML-responses
- **Beveiliging**: De SAML-response wordt verstuurd via HTTP POST naar de ACS URL
- **Gebruik**: Aanbevolen voor productieomgevingen

#### HTTP-Redirect Binding
- **Alternatieve methode**: SAML-response verstuurd via HTTP-redirect
- **Beperkingen**: Beperkte payload-grootte vanwege URL-lengtebeperkingen
- **Gebruik**: Wordt ondersteund, maar HTTP-POST heeft de voorkeur

### Name ID Policy

FastComments configureert het volgende Name ID-beleid in SAML-requests:

- **Standaardformaat**: `urn:oasis:names:tc:SAML:1.1:nameid-format:emailAddress`
- **Alternatieve formaten**: Persistent, Transient, Unspecified (configureerbaar)
- **Vereiste**: Het e-mailadres wordt gebruikt als primaire gebruikersidentificatie

### SAML Request-attributen

Bij het starten van SAML-authenticatie verstuurt FastComments requests met de volgende kenmerken:

#### Request Signing
- **Status**: Optioneel (configureerbaar)
- **Algoritme**: Komt overeen met het geconfigureerde handtekeningalgoritme
- **Certificaat**: Gebruikt een tenantspecifiek certificaat als request signing is ingeschakeld

#### Opgevraagde attributen
FastComments vraagt de volgende attributen op in SAML AuthnRequests:

- **Email**: Vereist voor gebruikersidentificatie
- **First Name**: Optioneel, voor weergavedoeleinden
- **Last Name**: Optioneel, voor weergavedoeleinden
- **Roles/Groups**: Optioneel, voor toegangsbeheer en rechten

### SP-informatie kopiëren

De SAML-configuratiepagina biedt klikbare velden die SP-informatie automatisch naar je klembord kopiëren:

1. Klik op een SP-informatieveld (Entity ID, ACS URL, enz.)
2. De waarde wordt automatisch naar je klembord gekopieerd
3. Plak de waarde in de configuratie van je identity provider
4. Een korte markering geeft aan dat het kopiëren is gelukt

Dit maakt het eenvoudig om de SP-informatie nauwkeurig over te zetten naar je IdP zonder typefouten.

### SP-certificaatinformatie

#### Gebruik van certificaten
- **Doel**: Versleutelt communicatie en verifieert de SP-identiteit
- **Rotatie**: Certificaten worden automatisch beheerd door FastComments
- **Toegang**: Publieke certificaten zijn beschikbaar via de metadata-URL

#### Certificaatdetails
- **Algoritme**: RSA-2048 of hoger
- **Geldigheid**: Certificaten worden automatisch verlengd vóór het verstrijken
- **Distributie**: Beschikbaar via standaard SAML-metadata

### Problemen oplossen met SP-configuratie

Als je identity provider problemen meldt met de SP-informatie:

1. **Controleer URL's**: Zorg ervoor dat alle URL's HTTPS gebruiken en de juiste tenant-ID bevatten
2. **Controleer metadata**: Gebruik de metadata-URL om de configuratie te verifiëren
3. **Test connectiviteit**: Zorg ervoor dat je IdP de FastComments-endpoints kan bereiken
4. **Valideer het formaat**: Bevestig dat je IdP het formaat van de SP-informatie ondersteunt

Veelvoorkomende problemen zijn:
- Onjuiste tenant-ID in URL's
- Netwerkconnectiviteitsproblemen tussen IdP en FastComments
- IdP verwacht andere URL-formaten of aanvullende configuratieopties