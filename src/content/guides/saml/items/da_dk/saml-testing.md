Test af din SAML-konfiguration sikrer, at godkendelse fungerer korrekt, før den rulles ud til produktionsbrugere.

### Tjekliste før test

Før du tester SAML-godkendelse, bekræft:

- ✅ SAML er aktiveret i FastComments
- ✅ Alle nødvendige felter er udfyldt (IdP URL, certifikat)
- ✅ Identitetsudbyderen er konfigureret med FastComments SP-oplysninger
- ✅ Testbruger findes i din IdP
- ✅ Testbrugeren har de rette roller tildelt

### Testmetoder

#### Metode 1: Direkte SAML-login-URL

1. **Hent SAML-login-URL**:
   - Kopier fra din SAML-konfigurationsside
   - Format: `https://fastcomments.com/saml/login/{your-tenant-id}`

2. **Test godkendelse**:
   - Åbn SAML-login-URL'en i et inkognito-/privat browservindue
   - Du bør blive omdirigeret til din identitetsudbyder
   - Log ind med testoplysninger
   - Bekræft, at du omdirigeres tilbage til FastComments

#### Metode 2: Adgang til admin-dashboardet

1. **Gå til FastComments**:
   - Gå til [FastComments admin-dashboard](https://fastcomments.com/auth/my-account)
   - Se efter SAML-loginmulighed eller brug SAML-login-URL'en

2. **Gennemfør godkendelsesflowet**:
   - Godkend via din identitetsudbyder
   - Bekræft adgang til de relevante adminfunktioner baseret på tildelte roller

#### Metode 3: Test af widget-integration

Til test af SAML med kommentarwidgets:

1. **Indlejr widget**: Brug FastComments-widget'en på en testside
2. **Godkendelse**: Klik på login og vælg SAML-muligheden (hvis tilgængelig)
3. **Verifikation**: Bekræft, at brugeren vises som godkendt i widget'en

### Hvad der skal verificeres under test

#### Godkendelsesflow

**Succesfuld omdirigering**:
- Brugeren omdirigeres til IdP's login-side
- IdP's login-side indlæses korrekt
- Ingen certifikat- eller SSL-fejl opstår

**IdP-godkendelse**:
- Brugeren kan logge ind med deres IdP-legitimationsoplysninger
- Multi-faktor-godkendelse fungerer (hvis konfigureret)
- Ingen godkendelsesfejl fra IdP

**Returnering til FastComments**:
- Brugeren omdirigeres tilbage til FastComments efter succesfuld IdP-login
- Ingen valideringsfejl af SAML-assertion
- Brugeren får adgang til de relevante FastComments-funktioner

#### Brugeroplysninger

**Grundlæggende profildata**:
- E-mailadresse indfanges korrekt
- Fornavn og efternavn vises, hvis de er angivet
- Brugerprofil oprettes eller opdateres

**Rollertilskrivning**:
- Administrative roller er korrekt tildelt
- Brugeren har adgang til forventede adminfunktioner
- Tilladelser matcher de tildelte roller

#### Validering af SAML-respons

**Certifikatverifikation**:
- SAML-responsens signatur valideres med succes
- Ingen certifikatvalideringsfejl i logfiler
- Respons accepteres som ægte

**Atributbehandling**:
- Påkrævede attributter (email) er til stede
- Valgfrie attributter behandles korrekt
- Rolleattributter fortolkes og anvendes korrekt

### Test af forskellige scenarier

#### Standard brugerflow

1. **Ny bruger**:
   - Første SAML-login
   - Oprettelse af konto
   - Tildeling af basisrettigheder

2. **Eksisterende bruger**:
   - Tilbagevendende brugerlogin
   - Profilopdateringer
   - Rolleændringer

#### Test af administrativ adgang

1. **Adminroller**:
   - Testbrugere med `fc-admin-admin`-rollen
   - Bekræft adgang til admin-dashboardet
   - Bekræft administrative beføjelser

2. **Specialiserede roller**:
   - Test adgang for `fc-moderator` til moderationsfunktioner
   - Test adgang for `fc-analytics-admin` til analysefunktioner
   - Test adgang for `fc-billing-admin` til faktureringsfunktioner

#### Fejlscenarier

1. **Ugyldige certifikater**:
   - Test med udløbne eller forkerte certifikater
   - Bekræft korrekt fejlhåndtering

2. **Manglende attributter**:
   - Test SAML-responser uden den påkrævede emailattribut
   - Bekræft elegant fejlhåndtering

3. **Netværksproblemer**:
   - Test med forbindelsesproblemer
   - Bekræft timeout-håndtering

### Fejlfinding af testproblemer

#### Almindelige godkendelsesproblemer

**Omdirigeringsloop**:
- Kontrollér, at SP Entity ID matcher IdP-konfigurationen
- Bekræft, at ACS URL er korrekt konfigureret
- Bekræft, at SAML-bindingindstillingerne stemmer overens

**Certifikatfejl**:
- Sørg for, at certifikatet indeholder BEGIN/END-markører
- Bekræft, at certifikatet ikke er udløbet
- Tjek for ekstra mellemrum eller formateringsproblemer

**Attributproblemer**:
- Bekræft, at emailattributten sendes
- Bekræft, at rolleattributter bruger korrekt navngivning
- Tjek attributformat (array vs. komma-adskilt)

#### Fejlsøgningsværktøjer

**Browserudviklerværktøjer**:
- Overvåg netværksanmodninger under SAML-flowet
- Tjek for HTTP-fejl eller omdirigeringer
- Undersøg SAML POST-data (hvis synligt)

**IdP-testværktøjer**:
- De fleste IdP'er tilbyder SAML-testgrænseflader
- Brug IdP-værktøjer til at validere SAML-responsformatet
- Test attributkonfigurationen, før du sender til FastComments

**FastComments-support**:
- Aktivér debug-logning under test
- Gem fejlmeddelelser og tidsstempler
- Kontakt support med specifikke fejldetaljer

### Bedste praksis for test

#### Opsætning af testmiljø

1. **Dedikerede testbrugere**:
   - Opret specifikke testkonti i din IdP
   - Tildel forskellige rollekombinationer
   - Brug letgenkendelige test-e-mailadresser

2. **Isoleret test**:
   - Brug inkognito-/private browservinduer
   - Ryd cookies mellem tests
   - Test med forskellige brugerkonti

3. **Dokumentation**:
   - Registrér testscenarier og resultater
   - Dokumentér eventuelle nødvendige konfigurationsændringer
   - Notér detaljer om succesfuld konfiguration

#### Validering før produktion

1. **Omfattende test**:
   - Test alle rollekombinationer
   - Bekræft kanttilfælde og fejlsituationer
   - Bekræft, at ydeevnen er acceptabel

2. **Brugeraccept**:
   - Få slutbrugere til at teste godkendelsesflowet
   - Indsaml feedback om brugeroplevelsen
   - Bekræft, at workflowet opfylder kravene

3. **Sikkerhedsreview**:
   - Bekræft, at certifikatvalidering fungerer
   - Bekræft, at rollefordelinger er sikre
   - Test håndhævelse af adgangskontrol

### Udrulning i produktion

Efter succesfuld test:

1. **Gradvis udrulning**: Overvej først at udrulle SAML til en delmængde af brugerne
2. **Overvågning**: Overvåg succesrater for godkendelse og fejl i logfiler
3. **Supportforberedelse**: Forbered supportteamet på SAML-relaterede spørgsmål
4. **Dokumentation**: Udarbejd brugerdokumentation for SAML-loginprocessen