Når SAML er aktiveret i FastComments, genererer systemet automatisk Service Provider (SP)-information, som du skal konfigurere i din identity provider.

### Adgang til Service Provider-information

SP-informationen vises på din SAML-konfigurationsside, efter du har aktiveret SAML-godkendelse. Denne information indeholder alle de detaljer, din identity provider har brug for til at etablere SAML-tillidsforholdet.

### Service Provider-endpoints

#### SP Entity ID / Audience
**Formål**: Identificerer entydigt din FastComments-instans som service provider
**Format**: `https://fastcomments.com/saml/{your-tenant-id}`
**Anvendelse**: Konfigurer dette som Entity ID eller Audience i din IdP

Denne identifikator sikrer, at SAML-svar er beregnet til din specifikke FastComments-tenant, og forhindrer, at SAML-svar accepteres af andre instanser.

#### Assertion Consumer Service (ACS) URL
**Formål**: Endpointet, hvor din IdP sender SAML-svar efter brugergodkendelse
**Format**: `https://fastcomments.com/saml/callback/{your-tenant-id}`
**Anvendelse**: Konfigurer dette som ACS URL eller Reply URL i din IdP

Det er her, brugere omdirigeres til efter vellykket godkendelse hos din identity provider, sammen med SAML-assertionen, der indeholder brugeroplysninger.

#### SP Metadata URL
**Formål**: Giver komplet SAML-konfiguration i standard-XML-format
**Format**: `https://fastcomments.com/saml/metadata/{your-tenant-id}`
**Anvendelse**: Nogle IdP'er kan automatisk importere konfigurationen via denne URL

Metadata-URL'en indeholder al nødvendig SP-information i XML-format, hvilket gør det nemt automatisk at konfigurere kompatible identity providers.

#### SAML Login URL
**Formål**: Direkte link til at starte SAML-godkendelse for din tenant
**Format**: `https://fastcomments.com/saml/login/{your-tenant-id}`
**Anvendelse**: Link brugere direkte til SAML-godkendelse eller test flowet

Du kan bruge denne URL til at teste SAML-godkendelse eller give brugerne et direkte link til at logge ind via SAML.

### Understøttelse af SAML-bindings

FastComments understøtter følgende SAML-bindings:

#### HTTP-POST Binding
- **Primær metode**: Den mest almindelige binding til SAML-svar
- **Sikkerhed**: SAML-svaret sendes via HTTP POST til ACS-URL'en
- **Anvendelse**: Anbefales til produktionsmiljøer

#### HTTP-Redirect Binding
- **Alternativ metode**: SAML-svar sendt via HTTP-redirect
- **Begrænsninger**: Begrænset payload-størrelse pga. URL-længdebegrænsninger
- **Anvendelse**: Understøttet, men HTTP-POST foretrækkes

### Name ID-politik

FastComments konfigurerer følgende Name ID-politik i SAML-forespørgsler:

- **Standardformat**: `urn:oasis:names:tc:SAML:1.1:nameid-format:emailAddress`
- **Alternative formater**: Persistent, Transient, Unspecified (kan konfigureres)
- **Krav**: E-mailadressen bruges som primær brugeridentifikator

### SAML-forespørgselsattributter

Ved initiering af SAML-godkendelse sender FastComments forespørgsler med disse karakteristika:

#### Forespørgselssignering
- **Status**: Valgfri (kan konfigureres)
- **Algoritme**: Matcher den konfigurerede signaturalgoritme
- **Certifikat**: Bruger tenantspecifikt certifikat, hvis forespørgselssignering er aktiveret

#### Anmodede attributter
FastComments anmoder om følgende attributter i SAML AuthnRequests:

- **Email**: Påkrævet til brugeridentifikation
- **First Name**: Valgfri til visningsformål  
- **Last Name**: Valgfri til visningsformål
- **Roles/Groups**: Valgfri til adgangskontrol og tilladelser

### Kopiering af SP-information

SAML-konfigurationssiden giver klikbare felter, der automatisk kopierer SP-information til din udklipsholder:

1. Klik på et vilkårligt SP-informationsfelt (Entity ID, ACS URL osv.)
2. Værdien kopieres automatisk til din udklipsholder
3. Indsæt værdien i din identity providers konfiguration
4. En kort fremhævelse indikerer, at kopieringen lykkedes

Dette gør det nemt at overføre SP-informationen til din IdP nøjagtigt uden tastefejl.

### SP-certifikatinformation

#### Certifikatanvendelse
- **Formål**: Krypterer kommunikation og verificerer SP-identiteten
- **Rotation**: Certifikater administreres automatisk af FastComments
- **Adgang**: Offentlige certifikater er tilgængelige via metadata-URL'en

#### Certifikatdetaljer
- **Algoritme**: RSA-2048 eller højere
- **Gyldighed**: Certifikater fornyes automatisk inden udløb
- **Distribution**: Tilgængelige via standard SAML-metadata

### Fejlfinding af SP-konfiguration

Hvis din identity provider rapporterer problemer med SP-informationen:

1. **Verificér URL'er**: Sørg for, at alle URL'er bruger HTTPS og indeholder den korrekte tenant-ID
2. **Tjek metadata**: Brug metadata-URL'en til at verificere konfigurationen
3. **Test forbindelsen**: Sørg for, at din IdP kan nå FastComments-endpoints
4. **Validér formatet**: Bekræft, at din IdP understøtter SP-informationsformatet

Almindelige problemer omfatter:
- Forkert tenant-ID i URL'er
- Netværksforbindelsesproblemer mellem IdP og FastComments
- IdP, der forventer andre URL-formater eller yderligere konfigurationsmuligheder