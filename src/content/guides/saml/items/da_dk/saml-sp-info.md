Når SAML er aktiveret i FastComments, genererer systemet automatisk Service Provider (SP)-oplysninger, som du skal konfigurere i din identitetsudbyder.

### Adgang til Service Provider-oplysninger

SP-oplysningerne vises på din SAML-konfigurationsside efter aktivering af SAML-godkendelse. Disse oplysninger indeholder alle detaljer, som din identitetsudbyder har brug for for at etablere SAML-tillidsforholdet.

### Service Provider-endepunkter

#### SP Entity ID / Audience
**Formål**: Identificerer entydigt din FastComments-instans som en service provider  
**Format**: `https://fastcomments.com/saml/{your-tenant-id}`  
**Brug**: Konfigurer dette som Entity ID eller Audience i din IdP

Denne identifikator sikrer, at SAML-responser er tiltænkt din specifikke FastComments-tenant og forhindrer, at SAML-responser accepteres af andre instanser.

#### Assertion Consumer Service (ACS) URL
**Formål**: Endepunktet hvor din IdP sender SAML-responser efter brugerautentificering  
**Format**: `https://fastcomments.com/saml/callback/{your-tenant-id}`  
**Brug**: Konfigurer dette som ACS URL eller Reply URL i din IdP

Dette er stedet, som brugere omdirigeres til efter succesfuld autentificering med din identitetsudbyder, sammen med SAML-assertionen, der indeholder brugeroplysninger.

#### SP Metadata URL
**Formål**: Leverer komplet SAML-konfiguration i standard XML-format  
**Format**: `https://fastcomments.com/saml/metadata/{your-tenant-id}`  
**Brug**: Nogle IdP'er kan automatisk importere konfiguration ved hjælp af denne URL

Metadata-URL'en indeholder alle nødvendige SP-oplysninger i XML-format, hvilket gør det nemt at konfigurere kompatible identitetsudbydere automatisk.

#### SAML Login URL
**Formål**: Direkte link til at starte SAML-godkendelse for din tenant  
**Format**: `https://fastcomments.com/saml/login/{your-tenant-id}`  
**Brug**: Link brugere direkte til SAML-godkendelse eller test flowet

Du kan bruge denne URL til at teste SAML-godkendelse eller give brugerne et direkte link til at logge ind via SAML.

### Understøttelse af SAML-bindinger

FastComments understøtter følgende SAML-bindinger:

#### HTTP-POST Binding
- **Primær metode**: Den mest almindelige binding for SAML-responser  
- **Sikkerhed**: SAML-responsen sendes via HTTP POST til ACS URL  
- **Brug**: Anbefales til produktionsudrulninger

#### HTTP-Redirect Binding
- **Alternativ metode**: SAML-respons sendt via HTTP-redirect  
- **Begrænsninger**: Begrænset payload-størrelse på grund af URL-længdebegrænsninger  
- **Brug**: Understøttet, men HTTP-POST foretrækkes

### Name ID-politik

FastComments konfigurerer følgende Name ID-politik i SAML-anmodninger:

- **Standardformat**: `urn:oasis:names:tc:SAML:1.1:nameid-format:emailAddress`  
- **Alternative formater**: Persistent, Transient, Unspecified (kan konfigureres)  
- **Krav**: E-mailadressen bruges som den primære brugeridentifikator

### SAML-anmodningsattributter

Når SAML-godkendelse initieres, sender FastComments anmodninger med disse karakteristika:

#### Signering af anmodninger
- **Status**: Valgfri (kan konfigureres)  
- **Algoritme**: Matcher den konfigurerede signaturalgoritme  
- **Certifikat**: Bruger tenant-specifikt certifikat, hvis anmodningssignering er aktiveret

#### Anmodede attributter
FastComments anmoder om følgende attributter i SAML AuthnRequests:

- **E-mail**: Påkrævet for brugeridentifikation  
- **Fornavn**: Valgfri til visningsformål  
- **Efternavn**: Valgfri til visningsformål  
- **Roller/Grupper**: Valgfri til adgangskontrol og tilladelser

### Kopiering af SP-oplysninger

SAML-konfigurationssiden tilbyder klikbare felter, der automatisk kopierer SP-oplysninger til dit udklipsholder:

1. Klik på et hvilket som helst SP-oplysningsfelt (Entity ID, ACS URL osv.)  
2. Værdien kopieres automatisk til dit udklipsholder  
3. Indsæt værdien i din identitetsudbyders konfiguration  
4. En kort markering indikerer, at kopieringen lykkedes

Dette gør det nemt at overføre SP-oplysningerne nøjagtigt til din IdP uden tastefejl.

### SP-certifikatoplysninger

#### Certifikatets anvendelse
- **Formål**: Krypterer kommunikation og bekræfter SP-identitet  
- **Rotering**: Certifikater administreres automatisk af FastComments  
- **Adgang**: Offentlige certifikater er tilgængelige via metadata-URL'en

#### Certifikatdetaljer
- **Algoritme**: RSA-2048 eller højere  
- **Gyldighed**: Certifikater fornyes automatisk før udløb  
- **Distribution**: Tilgængelig gennem standard SAML-metadata

### Fejlfinding af SP-konfiguration

Hvis din identitetsudbyder rapporterer problemer med SP-oplysninger:

1. **Verificer URL'er**: Sørg for, at alle URL'er bruger HTTPS og indeholder den korrekte tenant-ID  
2. **Tjek metadata**: Brug metadata-URL'en til at verificere konfigurationen  
3. **Test forbindelse**: Sørg for, at din IdP kan nå FastComments-endepunkterne  
4. **Valider format**: Bekræft, at din IdP understøtter SP-oplysningsformatet

Almindelige problemer inkluderer:
- Forkert tenant-ID i URL'er  
- Netværksforbindelsesproblemer mellem IdP og FastComments  
- IdP forventer andre URL-formater eller yderligere konfigurationsmuligheder