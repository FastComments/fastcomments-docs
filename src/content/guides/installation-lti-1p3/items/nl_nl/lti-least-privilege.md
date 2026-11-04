De FastComments LTI 1.3-integratie volgt het principe van minimale privileges: het gebruikt alleen de launch-claims die nodig zijn om de gebruiker te identificeren, opmerkingen aan de juiste cursus en resource te koppelen, en rolgebaseerde rechten toe te passen.

De rest van deze pagina brengt elke claim die de integratie verbruikt, elke LTI Advantage-service die niet wordt aangevraagd, en elke categorie gegevens die niet wordt verzameld in kaart. Beveiligings- en inkoopreviewers kunnen antwoorden rechtstreeks uit de tabellen hieronder halen.

## Gegevenselementen ontvangen van het LMS

Elke LTI 1.3-launch bevat een ondertekende JWT van het LMS. FastComments haalt de volgende claims uit die JWT en gebruikt niets anders:

| Veld | LTI-claim | Doel | Vereist | Opgeslagen |
|-------|-----------|---------|----------|--------|
| Gebruikersidentificatie | `sub` | Identificeert de gebruiker consistent over launches heen, zodat dezelfde persoon wordt gemapt naar dezelfde FastComments SSO-gebruiker | Ja | Ja, als onderdeel van een stabiel intern SSO-ID |
| Weergavenaam | `name` | Naamsvermelding naast de opmerkingen van de gebruiker | Ja (valt terug op "LMS User" indien afwezig) | Ja |
| E-mail | `email` | Accountkoppeling, notificaties, moderatie, supportcorrespondentie | Optioneel (de integratie werkt ook zonder) | Ja, indien verstrekt |
| Avatar-URL | `picture` | Weergegeven bij de opmerkingen van de gebruiker | Optioneel | Alleen de URL; FastComments downloadt of herbergt de afbeelding niet |
| Rollen | `https://purl.imsglobal.org/spec/lti/claim/roles` | Bepaalt of de gebruiker administrator, instructor (moderator) of leerling is | Ja | Afgeleide `isAdmin` / `isModerator`-vlaggen op de SSO-sessie |
| Cursuscontext | `https://purl.imsglobal.org/spec/lti/claim/context` (`id`, `title`) | Koppelt de commentardraad aan de juiste LMS-cursus | Ja | Ja, als onderdeel van de opgeloste pagina-identificatie |
| Resource link | `https://purl.imsglobal.org/spec/lti/claim/resource_link` (`id`) | Koppelt opmerkingen aan de juiste activiteit of toolplaatsing binnen de cursus | Ja, indien aanwezig | Ja, als onderdeel van de opgeloste pagina-identificatie |
| Deployment ID | `https://purl.imsglobal.org/spec/lti/claim/deployment_id` | Routeert de launch naar de juiste FastComments tenant-configuratie | Ja | Ja, op het FastComments LTI-configuratierecord |

## Claims en scopes gedeclareerd bij registratie

Tijdens LTI 1.3 Dynamic Registration registreert FastComments zichzelf met `scope: ""` (geen aanvullende OAuth-scopes) en declareert alleen deze OpenID Connect-claims:

`iss`, `sub`, `name`, `email`, `picture`

Het registreert twee berichttypen:

- `LtiResourceLinkRequest` - de standaard cursuslaunch naar FastComments.
- `LtiDeepLinkingRequest` - stelt instructeurs in staat de FastComments-tool binnen een cursus te plaatsen.

Er worden geen aanvullende access tokens aangevraagd bij het LMS.

## LTI Advantage-services die niet worden aangevraagd

| Service / scope | Aangevraagd? | Reden |
|------------------|------------|--------|
| Names and Role Provisioning Services (NRPS) | Nee | De integratie heeft geen cursusrooster nodig; de gebruikersidentiteit komt mee met elke launch |
| Assignment and Grade Services (AGS) - lineitem-, score- en result-scopes | Nee | De integratie is niet gradebook-bewust |
| Deep Linking voorbij de standaard plaatsing-terugkeer | Geen aanvullende gegevens | Deep linking wordt alleen gebruikt voor het plaatsen van de tool door instructeurs; er wordt geen cursusinhoud opgehaald |

## Niet-verzamelde gegevens

Naast LTI zelf vraagt of ontvangt FastComments het volgende niet van het LMS of de gebruiker:

| Categorie | Verzameld? |
|----------|------------|
| Cijfers van studenten | Nee |
| Inzendingen van opdrachten | Nee |
| Aanwezigheidsregistraties | Nee |
| Volledige cursusroosters | Nee |
| Overheidsidentificatiegegevens | Nee |
| Geboortedatum | Nee |
| Postadres of telefoonnummer | Nee |
| Financiële informatie | Nee |
| LMS-administratorreferenties | Nee |

## Toegangsgrenzen

- FastComments ontvangt alleen gegevens binnen een geautoriseerde LTI 1.3-launch die is ondertekend met de geregistreerde sleutels van het LMS. De integratie roept het LMS niet terug voor aanvullende informatie.
- Launch-tokens zijn eenmalig en kortlevend. Herhaalde of verlopen tokens worden geweigerd.
- LMS-administrators bepalen waar de tool binnen hun platform wordt ingezet. D2L Brightspace ondersteunt bijvoorbeeld per-deployment org-unit scoping en per-deployment beveiligingsinstellingen, waarmee administrators de tool kunnen beperken tot specifieke cursussen of org units in plaats van deze globaal beschikbaar te maken. Moodle, Blackboard, Sakai en Schoology bieden equivalente per-deployment controles in hun LTI 1.3-implementaties.

## Opslag en retentie

FastComments bewaart LTI-afgeleide gegevens voor de duur van de actieve commentaardienst en volgens door de klant geconfigureerde retentie-instellingen. Commentaargegevens worden opgeslagen in versleutelde productieopslag (encrypted-at-rest). Bij beëindiging van het account of op schriftelijk verzoek tot verwijdering verwijdert of anonimiseert FastComments klantgegevens volgens de toepasselijke overeenkomst.

Voor volledige details over opslag en gegevensverwerking, zie het <a href="https://fastcomments.com/privacy-policy" target="_blank">FastComments Privacybeleid</a>.

## Reviewfrequentie

Elke nieuwe LTI-functie die aanvullende claims, scopes of LTI Advantage-services vereist, wordt vóór release beoordeeld om te bevestigen dat de aangevraagde toegang noodzakelijk en evenredig is aan de functie die wordt uitgebracht.

## Korte verklaring voor beveiligingsvragenlijsten

> FastComments past minimale privileges en dataminimalisatie toe op zijn LTI 1.3-integratie. De integratie gebruikt alleen de LTI launch-claims die nodig zijn om de gebruiker te authenticeren (`sub`, `name`, `email`, `picture`), de rol te bepalen, en de cursus en resource te identificeren waartoe de opmerkingen behoren. FastComments vraagt geen Names and Role Provisioning Services, Assignment and Grade Services, gradebook-gegevens, aanwezigheidsregistraties, volledige roosters of LMS-administratieve toegang aan. LMS-administrators behouden de controle over in welke org units, cursussen en deployments de tool beschikbaar is.