Denne guide gennemgår almindelige SAML-godkendelsesproblemer og deres løsninger.

### Certifikat- og sikkerhedsproblemer

#### Ugyldigt certifikat-fejl

**Symptomer**:
- "Certifikatvalidering mislykkedes"-fejl
- Brugere kan ikke fuldføre SAML-godkendelse
- SAML-svar afvises

**Almindelige årsager**:
- Certifikatformatet er forkert
- Certifikatet er udløbet
- Forkert certifikat blev angivet
- Ekstra tegn eller mellemrum i certifikatet

**Løsninger**:
1. **Verificér certifikatformat**:
   - Sørg for, at certifikatet indeholder `-----BEGIN CERTIFICATE-----` og `-----END CERTIFICATE-----` markører
   - Fjern eventuelt ekstra mellemrum eller linjeskift
   - Kopiér certifikatet direkte fra IdP-metadata eller konfiguration

2. **Tjek certifikatets gyldighed**:
   - Verificér, at certifikatet ikke er udløbet
   - Bekræft, at certifikatet er til den korrekte IdP
   - Brug online certifikatvalidatorer til at tjekke formatet

3. **Download certifikatet igen**:
   - Download et friskt certifikat fra IdP'en
   - Brug IdP-metadata-URL, hvis tilgængelig
   - Bekræft, at certifikatet matcher den aktuelle IdP-konfiguration

#### Signaturverifikation mislykkedes

**Symptomer**:
- Fejl ved validering af SAML-assertion-signatur
- Godkendelse mislykkes efter IdP-login
- "Ugyldig signatur"-fejlmeddelelser

**Løsninger**:
1. **Algoritme-mismatch**:
   - Tjek, at signaturalgoritmen i FastComments matcher IdP'en
   - Prøv forskellige signaturalgoritmer (SHA-256, SHA-1, SHA-512)
   - Verificér, at digest-algoritmen matcher IdP-konfigurationen

2. **Certifikatproblemer**:
   - Sørg for, at det korrekte signeringscertifikat er konfigureret
   - Verificér, at certifikatet svarer til den private nøgle, som IdP'en bruger
   - Tjek for certifikatrotation i IdP'en

### Konfigurationsproblemer

#### Forkert Entity ID eller ACS URL

**Symptomer**:
- IdP'en rapporterer "Ukendt service provider"
- SAML-svar sendes til forkert endpoint
- Godkendelsen fuldføres ikke

**Løsninger**:
1. **Verificér SP-oplysninger**:
   - Kopiér den nøjagtige Entity ID fra FastComments-konfigurationen
   - Sørg for, at ACS URL matcher formatet: `https://fastcomments.com/saml/callback/{tenant-id}`
   - Tjek for tastefejl i tenant ID

2. **IdP-konfiguration**:
   - Opdater IdP'en med korrekt SP Entity ID
   - Konfigurer korrekt ACS/Reply URL
   - Verificér IdP-binding-indstillinger (HTTP-POST foretrækkes)

#### Manglende eller forkerte attributter

**Symptomer**:
- Brugere oprettes uden korrekte roller
- Manglende brugerprofiloplysninger
- "Email påkrævet"-fejl

**Løsninger**:
1. **Email-attribut**:
   - Sørg for, at IdP'en sender email-attributten
   - Tjek attributnavnes-mapping (email, emailAddress osv.)
   - Verificér, at email-værdien er en gyldig emailadresse

2. **Rolleattributter**:
   - Bekræft, at IdP'en sender rolle/gruppe-oplysninger
   - Tjek, at rolleattributnavnene matcher FastComments' forventninger
   - Verificér, at rolleværdierne matcher FastComments' rollenavne præcist

3. **Attributformat**:
   - Test både array- og kommasepareret rolleformat
   - Sørg for, at attributværdier ikke har ekstra mellemrum
   - Tjek for versalfølsomhed i rollenavne

### Godkendelsesflow-problemer

#### Redirect-løkke

**Symptomer**:
- Browseren redirecter endeløst mellem FastComments og IdP'en
- Godkendelsen fuldføres aldrig
- Flere redirects vises i browserens udviklerværktøjer

**Løsninger**:
1. **Tjek SP-konfiguration**:
   - Verificér, at Entity ID matcher IdP-konfigurationen præcist
   - Sørg for, at ACS URL er korrekt konfigureret i IdP'en
   - Tjek for afsluttende skråstreger i URL'er

2. **Session-problemer**:
   - Ryd browser-cookies og prøv igen
   - Test i inkognito/privat browservindue
   - Tjek for indstillinger for session-timeout

#### Adgang nægtet efter godkendelse

**Symptomer**:
- SAML-godkendelsen lykkes
- Brugeren redirectes til FastComments
- "Adgang nægtet" eller tilladelsesfejl vises

**Løsninger**:
1. **Rolletildeling**:
   - Verificér, at brugeren har passende roller i IdP'en
   - Tjek, at rolleattributten sendes i SAML-svaret
   - Bekræft, at rollenavnene matcher FastComments' krav præcist

2. **Pakkebegrænsninger**:
   - Verificér, at kontoen har Flex- eller Pro-plan
   - Tjek, at SAML-funktionen er aktiveret for pakken
   - Kontakt support, hvis pakken inkluderer SAML, men funktionen er utilgængelig

### Identitetsudbyder-specifikke problemer

#### Microsoft Azure AD

**Almindelige problemer**:
- App-rolletildelinger afspejles ikke i tokens
- Claims sendes ikke korrekt
- Krav om brugertildeling

**Løsninger**:
- Tjek brugertildeling til FastComments-applikationen
- Verificér, at app-roller er korrekt konfigureret
- Sørg for, at claims-mapping inkluderer påkrævede attributter

#### Okta

**Almindelige problemer**:
- Gruppefiltre fungerer ikke korrekt
- Attributsætninger er forkert konfigureret
- Problemer med applikationstildeling

**Løsninger**:
- Gennemgå attributsætning-konfigurationen
- Tjek gruppetildeling og filtreringsregler
- Verificér, at applikationen er tildelt til passende brugere/grupper

#### Google Workspace

**Almindelige problemer**:
- Brugerdefinerede attributter mappes ikke korrekt
- Gruppemedlemskab sendes ikke
- Fejl i SAML-applikationskonfiguration

**Løsninger**:
- Konfigurer brugerdefineret skema til rolleattributter
- Tjek udbredelse af gruppemedlemskab
- Verificér attribut-mapping i SAML-applikationen

### Netværks- og forbindelsesproblemer

#### Timeout-fejl

**Symptomer**:
- Godkendelsesprocessen får timeout
- "Request timeout" eller lignende fejl
- Langsomt godkendelsesflow

**Løsninger**:
1. **Netværksforbindelse**:
   - Tjek, at firewall-regler tillader FastComments-kommunikation
   - Verificér DNS-opslag for fastcomments.com
   - Test netværksforbindelsen fra IdP'en til FastComments

2. **Ydeevneproblemer**:
   - Tjek IdP'ens svartider
   - Verificér, at validering af certifikatkæden ikke er langsom
   - Overvej netværkslatens mellem IdP og brugere

#### SSL/TLS-problemer

**Symptomer**:
- Certifikatadvarsler under godkendelse
- SSL-handshake-fejl
- "Sikker forbindelse mislykkedes"-fejl

**Løsninger**:
- Sørg for, at alle SAML-endpoints bruger HTTPS
- Tjek certifikatets gyldighed for alle involverede domæner
- Verificér kompatibilitet af TLS-version

### Fejlfinding og logning

#### Aktivering af debug-oplysninger

1. **Browserens udviklerværktøjer**:
   - Overvåg Network-fanen under SAML-flowet
   - Tjek Console for JavaScript-fejl
   - Undersøg SAML POST-requests (hvis synlige)

2. **IdP-logning**:
   - Aktivér SAML-debugging i din IdP
   - Gennemgå IdP-logs for detaljer om SAML-request/response
   - Tjek for problemer med attribut-mapping

#### Almindelige logmeddelelser

**FastComments-logs**:
- "SAML config not found" - SAML er ikke aktiveret eller er forkert konfigureret
- "Invalid certificate" - Certifikatvalidering mislykkedes
- "Missing email attribute" - Påkrævet email blev ikke angivet i SAML-svaret

**IdP-logs**:
- "Unknown service provider" - Entity ID mismatch
- "Invalid ACS URL" - Assertion Consumer Service URL er forkert
- "User not assigned" - Brugeren mangler adgang til SAML-applikationen

### At få hjælp

#### Oplysninger at indsamle

Når du kontakter support, så angiv:
- Nøjagtige fejlmeddelelser og tidsstempler
- SAML-konfigurationsdetaljer (uden følsomme data)
- IdP-type og version
- Trin til at genskabe problemet
- Browser- og netværksoplysninger

#### FastComments-support

For SAML-relaterede problemer:
1. Brug [supportportalen](https://fastcomments.com/auth/my-account/help)
2. Inkluder tenant ID og berørte brugeres emails
3. Angiv fejlmeddelelser og konfigurationsdetaljer
4. Specificér IdP-type og konfigurationstilgang

#### IdP-support

For IdP-specifikke problemer:
- Konsultér IdP-dokumentationen for SAML-fejlfinding
- Brug IdP'ens supportkanaler til konfigurationsproblemer
- Udnyt IdP'ens community-fora til almindelige problemer

### Forebyggelsestips

#### Bedste praksis

1. **Test grundigt**:
   - Test konfigurationsændringer i et ikke-produktionsmiljø
   - Verificér med flere testbrugere
   - Dokumentér fungerende konfigurationer

2. **Overvåg regelmæssigt**:
   - Opsæt overvågning af SAML-godkendelsesfejl
   - Gennemgå certifikaters udløbsdatoer
   - Overvåg for ændringer i IdP-konfigurationen

3. **Dokumentation**:
   - Vedligehold dokumentation af SAML-konfigurationen
   - Dokumentér eventuelle brugerdefinerede konfigurationer eller workarounds
   - Gem kontaktoplysninger for IdP-administratorer

#### Proaktivt vedligehold

1. **Certifikathåndtering**:
   - Overvåg certifikaters udløbsdatoer
   - Planlæg procedurer for certifikatrotation
   - Test certifikatopdateringer før udløb

2. **Konfigurationsgennemgange**:
   - Gennemgå regelmæssigt SAML-konfigurationen
   - Verificér, at IdP-konfigurationen forbliver aktuel
   - Opdater dokumentationen, efterhånden som ændringer foretages