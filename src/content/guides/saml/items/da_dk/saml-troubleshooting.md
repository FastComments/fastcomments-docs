Denne vejledning dækker almindelige SAML-autentificeringsproblemer og deres løsninger.

### Certificate and Security Issues

#### Invalid Certificate Error

**Symptoms**:
- "Certificate validation failed" fejl
- Brugere kan ikke gennemføre SAML-autentificering
- SAML-respons bliver afvist

**Common Causes**:
- Certifikatformatet er forkert
- Certifikatet er udløbet
- Forkert certifikat blev leveret
- Ekstra tegn eller mellemrum i certifikatet

**Solutions**:
1. **Verify Certificate Format**:
   - Sørg for at certifikatet indeholder `-----BEGIN CERTIFICATE-----` og `-----END CERTIFICATE-----` markører
   - Fjern eventuelle ekstra mellemrum eller linjeskift
   - Kopiér certifikatet direkte fra IdP-metadata eller konfiguration

2. **Check Certificate Validity**:
   - Bekræft at certifikatet ikke er udløbet
   - Bekræft at certifikatet er for den korrekte IdP
   - Brug online certifikatvalideringsværktøjer til at kontrollere formatet

3. **Re-download Certificate**:
   - Download et frisk certifikat fra IdP
   - Brug IdP-metadata-URL, hvis tilgængelig
   - Bekræft at certifikatet matcher den aktuelle IdP-konfiguration

#### Signature Verification Failed

**Symptoms**:
- SAML-assertions signaturvalideringsfejl
- Autentificering fejler efter IdP-login
- "Invalid signature" fejlmeddelelser

**Solutions**:
1. **Algorithm Mismatch**:
   - Tjek at signaturalgoritmen i FastComments matcher IdP
   - Prøv forskellige signaturalgoritmer (SHA-256, SHA-1, SHA-512)
   - Bekræft at digest-algoritmen matcher IdP-konfigurationen

2. **Certificate Issues**:
   - Sørg for at det korrekte signeringscertifikat er konfigureret
   - Bekræft at certifikatet svarer til den private nøgle, der bruges af IdP
   - Tjek for certifikatrotation i IdP

### Configuration Issues

#### Wrong Entity ID or ACS URL

**Symptoms**:
- IdP rapporterer "Unknown Service Provider"
- SAML-respons sendes til forkert endpoint
- Autentificering fuldføres ikke

**Solutions**:
1. **Verify SP Information**:
   - Kopiér den nøjagtige Entity ID fra FastComments-konfigurationen
   - Sørg for at ACS URL matcher formatet: `https://fastcomments.com/saml/callback/{tenant-id}`
   - Tjek for tastefejl i tenant ID

2. **IdP Configuration**:
   - Opdater IdP med korrekt SP Entity ID
   - Konfigurer korrekt ACS/Reply URL
   - Bekræft IdP binding-indstillinger (HTTP-POST foretrækkes)

#### Missing or Incorrect Attributes

**Symptoms**:
- Brugere oprettes uden korrekte roller
- Manglende brugerprofiloplysninger
- "Email required" fejl

**Solutions**:
1. **Email Attribute**:
   - Sørg for at IdP sender email-attribut
   - Tjek navnemapping for attributten (email, emailAddress, osv.)
   - Bekræft at email-værdien er en gyldig e-mailadresse

2. **Role Attributes**:
   - Bekræft at IdP sender rolle-/gruppeoplysninger
   - Tjek at rolleafternavne matcher FastComments forventninger
   - Bekræft at rolleværdierne matcher FastComments rolleanavne præcist

3. **Attribute Format**:
   - Test både array- og komma-separerede rolleformater
   - Sørg for at attributværdier ikke har ekstra mellemrum
   - Tjek for store/små-bogstavsfølsomhed i rollenavne

### Authentication Flow Issues

#### Redirect Loop

**Symptoms**:
- Browser omdirigerer uendeligt mellem FastComments og IdP
- Autentificering gennemføres aldrig
- Flere redirects vist i browserens udviklerværktøjer

**Solutions**:
1. **Check SP Configuration**:
   - Bekræft at Entity ID matcher IdP-konfigurationen præcist
   - Sørg for at ACS URL er korrekt konfigureret i IdP
   - Tjek for afsluttende skråstreger i URL'er

2. **Session Issues**:
   - Ryd browsercookies og prøv igen
   - Test i inkognito/private browservindue
   - Tjek session timeout-indstillinger

#### Access Denied After Authentication

**Symptoms**:
- SAML-autentificering lykkes
- Bruger omdirigeres til FastComments
- "Access denied" eller fejl i tilladelser vises

**Solutions**:
1. **Role Assignment**:
   - Bekræft at brugeren har passende roller i IdP
   - Tjek at rolleattributten sendes i SAML-responsen
   - Bekræft at rollenavne matcher FastComments krav præcist

2. **Package Limitations**:
   - Bekræft at kontoen har Flex- eller Pro-plan
   - Tjek at SAML-funktionen er aktiveret for pakken
   - Kontakt support, hvis pakken inkluderer SAML, men funktionen ikke er tilgængelig

### Identity Provider Specific Issues

#### Microsoft Azure AD

**Common Issues**:
- App-rolle-tildelinger afspejles ikke i tokens
- Claims sendes ikke korrekt
- Krav om bruger-tilknytning

**Solutions**:
- Tjek bruger-tilknytning til FastComments-applikationen
- Bekræft at applikationsroller er korrekt konfigureret
- Sørg for at claim-mapping inkluderer de krævede attributter

#### Okta

**Common Issues**:
- Gruppefiltre virker ikke korrekt
- Attributudtalelser er fejlkodet
- Problemer med applikationstildeling

**Solutions**:
- Gennemgå konfigurationen af attributudtalelser
- Tjek gruppe-tildeling og filtreringsregler
- Bekræft at applikationen er tildelt de relevante brugere/grupper

#### Google Workspace

**Common Issues**:
- Tilpassede attributter kortlægges ikke korrekt
- Gruppetilhørsforhold sendes ikke
- Fejl i SAML-applikationskonfiguration

**Solutions**:
- Konfigurer tilpasset skema for rolleattributter
- Tjek udbredelse af gruppemedlemskab
- Bekræft SAML-applikationens attributmapping

### Network and Connectivity Issues

#### Timeout Errors

**Symptoms**:
- Autentificeringsprocessen overskrider tiden
- "Request timeout" eller lignende fejl
- Langsom autentificeringsflow

**Solutions**:
1. **Network Connectivity**:
   - Tjek firewallregler for at sikre at FastComments-kommunikation er tilladt
   - Bekræft DNS-opløsning for fastcomments.com
   - Test netværksforbindelsen fra IdP til FastComments

2. **Performance Issues**:
   - Tjek IdP-responstider
   - Bekræft at validering af certifikatkæden ikke er langsom
   - Overvej netværkslatens mellem IdP og brugere

#### SSL/TLS Issues

**Symptoms**:
- Certifikatadvarsler under autentificering
- SSL-handshake-fejl
- "Secure connection failed" fejl

**Solutions**:
- Sørg for at alle SAML-endpoints bruger HTTPS
- Tjek certifikatets gyldighed for alle involverede domæner
- Bekræft kompatibilitet med TLS-versioner

### Debugging and Logging

#### Enabling Debug Information

1. **Browser Developer Tools**:
   - Overvåg Network-fanen under SAML-flowet
   - Tjek Console for JavaScript-fejl
   - Undersøg SAML POST-forespørgsler (hvis synlige)

2. **IdP Logging**:
   - Aktivér SAML-fejlsøgning i din IdP
   - Gennemgå IdP-logs for SAML-forespørgsels/-responsdetaljer
   - Tjek for problemer med attributmapping

#### Common Log Messages

**FastComments Logs**:
- "SAML config not found" - SAML ikke aktiveret eller forkert konfigureret
- "Invalid certificate" - Certifikatvalidering mislykkedes
- "Missing email attribute" - Krævet email ikke leveret i SAML-respons

**IdP Logs**:
- "Unknown service provider" - Entity ID stemmer ikke overens
- "Invalid ACS URL" - Assertion Consumer Service URL er forkert
- "User not assigned" - Brugeren mangler adgang til SAML-applikationen

### Getting Help

#### Information to Gather

Når du kontakter support, oplys:
- Præcise fejlmeddelelser og tidsstempler
- SAML-konfigurationsdetaljer (uden følsomme data)
- IdP-type og version
- Trin for at reproducere problemet
- Browser- og netværksinformation

#### FastComments Support

For SAML-relaterede problemer:
1. Brug [supportportalen](https://fastcomments.com/auth/my-account/help)
2. Inkludér tenant ID og berørte brugeres e-mails
3. Angiv fejlmeddelelser og konfigurationsdetaljer
4. Angiv IdP-type og konfigurationsmetode

#### IdP Support

For IdP-specifikke problemer:
- Konsulter IdP-dokumentation for SAML-fejlsøgning
- Brug IdP-supportkanaler til konfigurationsproblemer
- Benyt IdP-community-fora for almindelige problemer

### Prevention Tips

#### Best Practices

1. **Test Thoroughly**:
   - Test konfigurationsændringer i et ikke-produktionsmiljø
   - Bekræft med flere testbrugere
   - Dokumentér fungerende konfigurationer

2. **Monitor Regularly**:
   - Opsæt overvågning for SAML-autentificeringsfejl
   - Gennemgå certifikatudløbsdatoer
   - Overvåg for ændringer i IdP-konfigurationen

3. **Documentation**:
   - Vedligehold dokumentation af SAML-konfigurationen
   - Dokumentér eventuelle tilpassede konfigurationer eller workarounds
   - Opdater kontaktoplysninger for IdP-administratorer

#### Proactive Maintenance

1. **Certificate Management**:
   - Overvåg certifikatudløbsdatoer
   - Planlæg procedurer for certifikatrotation
   - Test certifikatopdateringer før udløb

2. **Configuration Reviews**:
   - Gennemgå SAML-konfiguration regelmæssigt
   - Bekræft at IdP-konfigurationen forbliver ajour
   - Opdater dokumentation når ændringer foretages