Deze gids behandelt veelvoorkomende SAML-authenticatieproblemen en hun oplossingen.

### Certificaat- en beveiligingsproblemen

#### Ongeldig certificaatfout

**Symptomen**:
- "Certificate validation failed" fout
- Gebruikers kunnen SAML-authenticatie niet voltooien
- SAML-responses worden afgewezen

**Veelvoorkomende oorzaken**:
- Certificaatformaat is onjuist
- Certificaat is verlopen
- Verkeerd certificaat is geleverd
- Extra tekens of witruimte in certificaat

**Oplossingen**:
1. **Controleer certificaatformaat**:
   - Zorg dat het certificaat de `-----BEGIN CERTIFICATE-----` en `-----END CERTIFICATE-----` markeringen bevat
   - Verwijder eventuele extra witruimte of regeleinden
   - Kopieer het certificaat rechtstreeks uit IdP-metadata of configuratie

2. **Controleer certificaatgeldigheid**:
   - Controleer of het certificaat niet is verlopen
   - Bevestig dat het certificaat voor de juiste IdP is
   - Gebruik online certificaatvalidators om het formaat te controleren

3. **Download certificaat opnieuw**:
   - Download een nieuw certificaat van de IdP
   - Gebruik de IdP-metadata-URL indien beschikbaar
   - Bevestig dat het certificaat overeenkomt met de huidige IdP-configuratie

#### Handtekeningverificatie mislukt

**Symptomen**:
- Fouten bij validatie van SAML-assertatiehandtekeningen
- Authenticatie mislukt na IdP-aanmelding
- "Invalid signature" foutmeldingen

**Oplossingen**:
1. **Algoritme-onverenigbaarheid**:
   - Controleer of het handtekeningsalgoritme in FastComments overeenkomt met de IdP
   - Probeer verschillende handtekeningsalgoritmen (SHA-256, SHA-1, SHA-512)
   - Verifieer dat het digest-algoritme overeenkomt met de IdP-configuratie

2. **Certificaatproblemen**:
   - Zorg dat het juiste ondertekeningscertificaat is geconfigureerd
   - Verifieer dat het certificaat overeenkomt met de privésleutel die door de IdP wordt gebruikt
   - Controleer op certificaatstrotatie in de IdP

### Configuratieproblemen

#### Verkeerde Entity ID of ACS-URL

**Symptomen**:
- IdP meldt "Unknown Service Provider"
- SAML-responses gaan naar verkeerde endpoint
- Authenticatie wordt niet voltooid

**Oplossingen**:
1. **Controleer SP-informatie**:
   - Kopieer de exacte Entity ID uit de FastComments-configuratie
   - Zorg dat de ACS-URL het volgende formaat heeft: `https://fastcomments.com/saml/callback/{tenant-id}`
   - Controleer op typefouten in tenant-ID

2. **IdP-configuratie**:
   - Werk de IdP bij met de juiste SP Entity ID
   - Configureer de juiste ACS/Reply URL
   - Controleer IdP-bindinginstellingen (HTTP-POST heeft de voorkeur)

#### Ontbrekende of onjuiste attributen

**Symptomen**:
- Gebruikers worden aangemaakt zonder juiste rollen
- Ontbrekende gebruikersprofielinformatie
- "Email required" fouten

**Oplossingen**:
1. **E-mailattribuut**:
   - Zorg dat de IdP het e-mailattribuut verzendt
   - Controleer de naam van het attribuutmapen (email, emailAddress, etc.)
   - Verifieer dat de e-mailwaarde een geldig e-mailadres is

2. **Rolattributen**:
   - Bevestig dat de IdP rol-/groepsinformatie verzendt
   - Controleer of de rolattribuutnamen overeenkomen met de verwachtingen van FastComments
   - Verifieer dat rolwaarden exact overeenkomen met de rolnamen in FastComments

3. **Attribuutformaat**:
   - Test zowel array- als komma-gescheiden rolformaten
   - Zorg dat attribuutwaarden geen extra witruimte bevatten
   - Controleer hoofdlettergevoeligheid in rolnamen

### Problemen met de authenticatiestroom

#### Redirect-lus

**Symptomen**:
- Browser wordt eindeloos doorgestuurd tussen FastComments en IdP
- Authenticatie wordt nooit voltooid
- Meerdere redirects zichtbaar in de developer tools van de browser

**Oplossingen**:
1. **Controleer SP-configuratie**:
   - Verifieer dat de Entity ID exact overeenkomt met de IdP-configuratie
   - Zorg dat de ACS-URL correct is geconfigureerd in de IdP
   - Controleer op eind-slashes in URL's

2. **Sessiekwesties**:
   - Wis browsercookies en probeer opnieuw
   - Test in een incognito/private browservenster
   - Controleer instellingen voor sessieverlooptijd

#### Toegang geweigerd na authenticatie

**Symptomen**:
- SAML-authenticatie is succesvol
- Gebruiker wordt doorgestuurd naar FastComments
- "Access denied" of permissiefout weergegeven

**Oplossingen**:
1. **Roltoewijzing**:
   - Verifieer dat de gebruiker de juiste rollen heeft in de IdP
   - Controleer of het rolattribuut wordt verzonden in de SAML-response
   - Bevestig dat rolnamen exact overeenkomen met de vereisten van FastComments

2. **Pakketbeperkingen**:
   - Controleer of het account een Flex- of Pro-plan heeft
   - Controleer of de SAML-functie is ingeschakeld voor het pakket
   - Neem contact op met support als het pakket SAML bevat maar de functie niet beschikbaar is

### IdP-specifieke problemen

#### Microsoft Azure AD

**Veelvoorkomende problemen**:
- App-roltoewijzingen worden niet in tokens weergegeven
- Claims worden niet correct verzonden
- Vereisten voor gebruikers toewijzing

**Oplossingen**:
- Controleer gebruikers toewijzing aan de FastComments-applicatie
- Verifieer dat app-rollen correct zijn geconfigureerd
- Zorg dat claimsmapping de vereiste attributen bevat

#### Okta

**Veelvoorkomende problemen**:
- Groepsfilters werken niet correct
- Attribuutverklaringen verkeerd geconfigureerd
- Problemen met applicatietoewijzing

**Oplossingen**:
- Bekijk de configuratie van attribuutverklaringen
- Controleer groeps-toewijzing en filterregels
- Verifieer dat de applicatie is toegewezen aan de juiste gebruikers/groepen

#### Google Workspace

**Veelvoorkomende problemen**:
- Aangepaste attributen worden niet correct gemapt
- Groepslidmaatschap wordt niet verzonden
- Fouten in SAML-applicatieconfiguratie

**Oplossingen**:
- Configureer een aangepast schema voor rolattributen
- Controleer de doorstroming van groepslidmaatschap
- Verifieer de attribuutmapping van de SAML-applicatie

### Netwerk- en verbindingsproblemen

#### Time-outfouten

**Symptomen**:
- Authenticatieproces loopt vast door time-out
- "Request timeout" of soortgelijke fouten
- Trage authenticatiestroom

**Oplossingen**:
1. **Netwerkconnectiviteit**:
   - Controleer of firewallregels FastComments-communicatie toestaan
   - Verifieer DNS-resolutie voor fastcomments.com
   - Test netwerkconnectiviteit van IdP naar FastComments

2. **Prestatieproblemen**:
   - Controleer reactietijden van de IdP
   - Verifieer dat validatie van certificaatketens niet traag is
   - Houd rekening met netwerkvertraging tussen IdP en gebruikers

#### SSL/TLS-problemen

**Symptomen**:
- Certificaatwaarschuwingen tijdens authenticatie
- SSL-handshakefouten
- "Secure connection failed" fouten

**Oplossingen**:
- Zorg dat alle SAML-endpoints HTTPS gebruiken
- Controleer certificaatgeldigheid voor alle betrokken domeinen
- Verifieer TLS-versiecompatibiliteit

### Debugging en logging

#### Debug-informatie inschakelen

1. **Browser Developer Tools**:
   - Bewaak het Network-tabblad tijdens de SAML-stroom
   - Controleer de Console op JavaScript-fouten
   - Onderzoek SAML POST-verzoeken (indien zichtbaar)

2. **IdP-logging**:
   - Schakel SAML-debugging in je IdP in
   - Bekijk IdP-logs voor details van SAML-aanvragen/responses
   - Controleer op problemen met attribuutmapping

#### Veelvoorkomende logmeldingen

**FastComments Logs**:
- "SAML config not found" - SAML niet ingeschakeld of verkeerd geconfigureerd
- "Invalid certificate" - Certificaatvalidatie mislukt
- "Missing email attribute" - Vereiste e-mail niet verstrekt in SAML-response

**IdP Logs**:
- "Unknown service provider" - Entity ID komt niet overeen
- "Invalid ACS URL" - Assertion Consumer Service URL onjuist
- "User not assigned" - Gebruiker heeft geen toegang tot SAML-applicatie

### Hulp krijgen

#### Informatie om te verzamelen

Wanneer je contact opneemt met support, verstrek:
- Exacte foutmeldingen en tijdstempels
- SAML-configuratiedetails (zonder gevoelige gegevens)
- IdP-type en versie
- Stappen om het probleem te reproduceren
- Browser- en netwerkinformatie

#### FastComments Support

Voor SAML-gerelateerde problemen:
1. Gebruik het [support portal](https://fastcomments.com/auth/my-account/help)
2. Voeg tenant-ID en e-mailadressen van getroffen gebruikers toe
3. Geef foutmeldingen en configuratiedetails
4. Specificeer IdP-type en configuratiebenadering

#### IdP Support

Voor IdP-specifieke problemen:
- Raadpleeg de IdP-documentatie voor SAML-troubleshooting
- Gebruik de supportkanalen van de IdP voor configuratieproblemen
- Maak gebruik van IdP-communityforums voor veelvoorkomende problemen

### Preventietips

#### Best practices

1. **Grondig testen**:
   - Test configuratiewijzigingen in een niet-productieomgeving
   - Verifieer met meerdere testgebruikers
   - Documenteer werkende configuraties

2. **Regelmatig monitoren**:
   - Stel monitoring in voor SAML-authenticatiefouten
   - Controleer certificaatvervaldata
   - Houd IdP-configuratiewijzigingen in de gaten

3. **Documentatie**:
   - Houd documentatie bij van SAML-configuratie
   - Documenteer eventuele aangepaste configuraties of workarounds
   - Bewaar contactgegevens van IdP-beheerders

#### Proactief onderhoud

1. **Certificaatbeheer**:
   - Houd certificaatvervaldata in de gaten
   - Plan procedures voor certificaatstrotatie
   - Test certificaatupdates vóór het verlopen

2. **Configuratiereviews**:
   - Beoordeel regelmatig de SAML-configuratie
   - Verifieer dat IdP-configuratie actueel blijft
   - Werk documentatie bij wanneer wijzigingen worden aangebracht