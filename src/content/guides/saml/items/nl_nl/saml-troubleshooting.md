Deze gids behandelt veelvoorkomende SAML-authenticatieproblemen en hun oplossingen.

### Certificaat- en Beveiligingsproblemen

#### Ongeldige Certificaatfout

**Symptomen**:
- Foutmelding "Certificaatvalidatie mislukt"
- Gebruikers kunnen SAML-authenticatie niet voltooien
- SAML-responses worden geweigerd

**Veelvoorkomende Oorzaken**:
- Certificaatformaat is onjuist
- Certificaat is verlopen
- Verkeerd certificaat is verstrekt
- Extra tekens of witruimte in certificaat

**Oplossingen**:
1. **Controleer Certificaatformaat**:
   - Zorg ervoor dat het certificaat de `-----BEGIN CERTIFICATE-----` en `-----END CERTIFICATE-----` markers bevat
   - Verwijder eventuele extra witruimte of regeleinden
   - Kopieer het certificaat rechtstreeks vanuit de IdP-metadata of configuratie

2. **Controleer Certificaatgeldigheid**:
   - Controleer of het certificaat niet is verlopen
   - Bevestig dat het certificaat voor de juiste IdP is
   - Gebruik online certificaatvalidatoren om het formaat te controleren

3. **Download Certificaat Opnieuw**:
   - Download een vers certificaat van de IdP
   - Gebruik de IdP-metadata-URL indien beschikbaar
   - Bevestig dat het certificaat overeenkomt met de huidige IdP-configuratie

#### Handtekeningverificatie Mislukt

**Symptomen**:
- Validatiefouten bij SAML-assertion-handtekeningen
- Authenticatie mislukt na IdP-login
- Foutmeldingen "Ongeldige handtekening"

**Oplossingen**:
1. **Algoritme-mismatch**:
   - Controleer of het handtekeningalgoritme in FastComments overeenkomt met de IdP
   - Probeer verschillende handtekeningalgoritmen (SHA-256, SHA-1, SHA-512)
   - Controleer of het digest-algoritme overeenkomt met de IdP-configuratie

2. **Certificaatproblemen**:
   - Zorg ervoor dat het juiste ondertekeningcertificaat is geconfigureerd
   - Controleer of het certificaat overeenkomt met de privésleutel die door de IdP wordt gebruikt
   - Controleer op certificaatrotatie in de IdP

### Configuratieproblemen

#### Verkeerde Entity ID of ACS URL

**Symptomen**:
- IdP meldt "Onbekende Service Provider"
- SAML-responses gaan naar het verkeerde endpoint
- Authenticatie wordt niet voltooid

**Oplossingen**:
1. **Controleer SP-informatie**:
   - Kopieer de exacte Entity ID uit de FastComments-configuratie
   - Zorg ervoor dat de ACS URL overeenkomt met het formaat: `https://fastcomments.com/saml/callback/{tenant-id}`
   - Controleer op typefouten in de tenant ID

2. **IdP-configuratie**:
   - Werk de IdP bij met de juiste SP Entity ID
   - Configureer de juiste ACS/Reply URL
   - Controleer de IdP-binding-instellingen (HTTP-POST heeft de voorkeur)

#### Ontbrekende of Onjuiste Attributen

**Symptomen**:
- Gebruikers worden aangemaakt zonder juiste rollen
- Ontbrekende gebruikersprofielinformatie
- Foutmeldingen "E-mail vereist"

**Oplossingen**:
1. **E-mailattribuut**:
   - Zorg ervoor dat de IdP het e-mailattribuut verstuurt
   - Controleer de attribuutnaamtoewijzing (email, emailAddress, enz.)
   - Controleer of de e-mailwaarde een geldig e-mailadres is

2. **Rollenattributen**:
   - Bevestig dat de IdP rol/groep-informatie verstuurt
   - Controleer of de rolattribuutnamen overeenkomen met wat FastComments verwacht
   - Controleer of de rolwaarden exact overeenkomen met de FastComments-rolnamen

3. **Attribuutformaat**:
   - Test zowel array- als komma-gescheiden rolformaten
   - Zorg ervoor dat attribuutwaarden geen extra witruimte bevatten
   - Controleer op hoofdlettergevoeligheid in rolnamen

### Authenticatiestroomproblemen

#### Redirect-lus

**Symptomen**:
- Browser redirectt eindeloos tussen FastComments en de IdP
- Authenticatie wordt nooit voltooid
- Meerdere redirects zichtbaar in de ontwikkelaarstools van de browser

**Oplossingen**:
1. **Controleer SP-configuratie**:
   - Controleer of de Entity ID exact overeenkomt met de IdP-configuratie
   - Zorg ervoor dat de ACS URL correct is geconfigureerd in de IdP
   - Controleer op afsluitende slashes in URL's

2. **Sessieproblemen**:
   - Wis browsercookies en probeer opnieuw
   - Test in een incognito/privé-browservenster
   - Controleer op instellingen voor sessietimeout

#### Toegang Geweigerd na Authenticatie

**Symptomen**:
- SAML-authenticatie slaagt
- Gebruiker wordt doorgestuurd naar FastComments
- "Toegang geweigerd" of machtigingsfout wordt weergegeven

**Oplossingen**:
1. **Roltoewijzing**:
   - Controleer of de gebruiker de juiste rollen heeft in de IdP
   - Controleer of het rolattribuut wordt verstuurd in de SAML-response
   - Bevestig dat de rolnamen exact overeenkomen met de FastComments-vereisten

2. **Pakketbeperkingen**:
   - Controleer of het account een Flex- of Pro-pakket heeft
   - Controleer of de SAML-functie is ingeschakeld voor het pakket
   - Neem contact op met support als het pakket SAML bevat maar de functie niet beschikbaar is

### Identiteitsproviderspecifieke Problemen

#### Microsoft Azure AD

**Veelvoorkomende Problemen**:
- App-rotoewijzingen worden niet weerspiegeld in tokens
- Claims worden niet correct verstuurd
- Vereisten voor gebruikerstoewijzing

**Oplossingen**:
- Controleer de gebruikerstoewijzing aan de FastComments-applicatie
- Controleer of app-rollen correct zijn geconfigureerd
- Zorg ervoor dat de claimstoewijzing de vereiste attributen bevat

#### Okta

**Veelvoorkomende Problemen**:
- Groepsfilters werken niet correct
- Attribuutstatements verkeerd geconfigureerd
- Problemen met applicatietoewijzing

**Oplossingen**:
- Controleer de configuratie van attribuutstatements
- Controleer groepstoewijzing en filterregels
- Controleer of de applicatie is toegewezen aan de juiste gebruikers/groepen

#### Google Workspace

**Veelvoorkomende Problemen**:
- Aangepaste attributen worden niet correct toegewezen
- Groepslidmaatschap wordt niet verstuurd
- Configuratiefouten in de SAML-applicatie

**Oplossingen**:
- Configureer een aangepast schema voor rolattributen
- Controleer de propagatie van groepslidmaatschap
- Controleer de attribuuttoewijzing van de SAML-applicatie

### Netwerk- en Verbindingsproblemen

#### Time-outfouten

**Symptomen**:
- Authenticatieproces verloopt op time-out
- Foutmeldingen "Request timeout" of vergelijkbaar
- Trage authenticatiestroom

**Oplossingen**:
1. **Netwerkconnectiviteit**:
   - Controleer of firewallregels communicatie met FastComments toestaan
   - Controleer de DNS-resolutie voor fastcomments.com
   - Test de netwerkconnectiviteit van de IdP naar FastComments

2. **Prestatieproblemen**:
   - Controleer de responstijden van de IdP
   - Controleer of de validatie van de certificaatketen niet traag is
   - Houd rekening met netwerklatentie tussen de IdP en gebruikers

#### SSL/TLS-problemen

**Symptomen**:
- Certificaatwaarschuwingen tijdens authenticatie
- SSL-handshake-fouten
- Foutmeldingen "Beveiligde verbinding mislukt"

**Oplossingen**:
- Zorg ervoor dat alle SAML-endpoints HTTPS gebruiken
- Controleer de certificaatgeldigheid voor alle betrokken domeinen
- Controleer de TLS-versiecompatibiliteit

### Debuggen en Loggen

#### Debuginformatie Inschakelen

1. **Browserontwikkelaarstools**:
   - Monitor het Netwerk-tabblad tijdens de SAML-stroom
   - Controleer de Console op JavaScript-fouten
   - Onderzoek SAML POST-requests (indien zichtbaar)

2. **IdP-logboeken**:
   - Schakel SAML-debugging in uw IdP in
   - Bekijk de IdP-logboeken voor SAML-request/response-details
   - Controleer op problemen met attribuuttoewijzing

#### Veelvoorkomende Logboekberichten

**FastComments-logboeken**:
- "SAML config not found" - SAML niet ingeschakeld of verkeerd geconfigureerd
- "Invalid certificate" - Certificaatvalidatie mislukt
- "Missing email attribute" - Vereiste e-mail niet verstrekt in de SAML-response

**IdP-logboeken**:
- "Unknown service provider" - Entity ID-mismatch
- "Invalid ACS URL" - Assertion Consumer Service URL onjuist
- "User not assigned" - Gebruiker heeft geen toegang tot de SAML-applicatie

### Hulp Krijgen

#### Te Verzamelen Informatie

Wanneer u contact opneemt met support, verstrek dan:
- Exacte foutmeldingen en tijdstempels
- SAML-configuratiedetails (zonder gevoelige gegevens)
- IdP-type en -versie
- Stappen om het probleem te reproduceren
- Browser- en netwerkinformatie

#### FastComments Support

Voor SAML-gerelateerde problemen:
1. Gebruik het [supportportaal](https://fastcomments.com/auth/my-account/help)
2. Vermeld de tenant ID en de e-mailadressen van betrokken gebruikers
3. Verstrek foutmeldingen en configuratiedetails
4. Geef het IdP-type en de configuratiebenadering op

#### IdP Support

Voor IdP-specifieke problemen:
- Raadpleeg de IdP-documentatie voor SAML-probleemoplossing
- Gebruik de IdP-supportkanalen voor configuratieproblemen
- Maak gebruik van IdP-communityforums voor veelvoorkomende problemen

### Preventietips

#### Beste Praktijken

1. **Test Grondig**:
   - Test configuratiewijzigingen in een niet-productieomgeving
   - Verifieer met meerdere testgebruikers
   - Documenteer werkende configuraties

2. **Monitor Regelmatig**:
   - Stel monitoring in voor SAML-authenticatiefouten
   - Bekijk de vervaldatums van certificaten
   - Monitor op wijzigingen in de IdP-configuratie

3. **Documentatie**:
   - Onderhoud documentatie van de SAML-configuratie
   - Documenteer eventuele aangepaste configuraties of workarounds
   - Bewaar contactgegevens van IdP-beheerders

#### Proactief Onderhoud

1. **Certificaatbeheer**:
   - Monitor de vervaldatums van certificaten
   - Plan certificaatrotatieprocedures
   - Test certificaatupdates vóór de vervaldatum

2. **Configuratiebeoordelingen**:
   - Beoordeel regelmatig de SAML-configuratie
   - Controleer of de IdP-configuratie actueel blijft
   - Werk de documentatie bij wanneer wijzigingen worden doorgevoerd