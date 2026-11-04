FastComments LTI 1.3-integrationen følger princippet om mindst mulige privilegium: den bruger kun de launch-claims, der er nødvendige for at identificere brugeren, koble kommentarer til det korrekte kursus og den korrekte ressource, samt anvende rollebaserede tilladelser.

Resten af denne side kortlægger hver claim, som integrationen forbruger, hver LTI Advantage-tjeneste, den ikke anmoder om, og hver kategori af data, den ikke indsamler. Sikkerheds- og indkøbsgennemgribere kan hente svar direkte fra tabellerne nedenfor.

## Dataelementer Modtaget Fra LMS'en

Hvert LTI 1.3-launch indeholder en signeret JWT fra LMS'en. FastComments udtrækker følgende claims fra denne JWT og bruger intet andet:

| Felt | LTI-claim | Formål | Påkrævet | Gemt |
|-------|-----------|---------|----------|--------|
| Brugeridentifikator | `sub` | Identificerer brugeren konsistent på tværs af launches, så samme person matcher samme FastComments SSO-bruger | Ja | Ja, som en del af et stabilt internt SSO-ID |
| Vist navn | `name` | Navngivelse, der vises ved siden af brugerens kommentarer | Ja (falder tilbage til "LMS User", hvis det mangler) | Ja |
| E-mail | `email` | Kontomatching, notifikationer, moderation, supportkorrespondance | Valgfri (integrationen fungerer uden den) | Ja, når den angives |
| Avatar-URL | `picture` | Viser på brugerens kommentarer | Valgfri | Kun URL; FastComments downloader eller genhoster ikke billedet |
| Roller | `https://purl.imsglobal.org/spec/lti/claim/roles` | Afgør, om brugeren er administrator, instruktør (moderator) eller studerende | Ja | Afledte `isAdmin` / `isModerator`-flag på SSO-sessionen |
| Kursuskontekst | `https://purl.imsglobal.org/spec/lti/claim/context` (`id`, `title`) | Kobler kommentartråden til det korrekte LMS-kursus | Ja | Ja, som en del af den opløste sideidentifikator |
| Ressourcelink | `https://purl.imsglobal.org/spec/lti/claim/resource_link` (`id`) | Kobler kommentarer til den korrekte aktivitet eller værktøjsplacering i kurset | Ja, når til stede | Ja, som en del af den opløste sideidentifikator |
| Deployment-ID | `https://purl.imsglobal.org/spec/lti/claim/deployment_id` | Dirigerer launchet til den korrekte FastComments-tenant-konfiguration | Ja | Ja, på FastComments LTI-konfigurationsposten |

## Claims og Scopes Deklareret ved Registrering

Under LTI 1.3 Dynamic Registration registrerer FastComments sig selv med `scope: ""` (ingen yderligere OAuth-scopes) og deklarerer kun disse OpenID Connect-claims:

`iss`, `sub`, `name`, `email`, `picture`

Den registrerer to beskedtyper:

- `LtiResourceLinkRequest` - standardkursus-launchet ind i FastComments.
- `LtiDeepLinkingRequest` - giver instruktører mulighed for at placere FastComments-værktøjet i et kursus.

Der anmodes ikke om yderligere access tokens fra LMS'en.

## LTI Advantage-tjenester, Der Ikke Anmodes Om

| Tjeneste / scope | Anmodes om? | Årsag |
|------------------|------------|--------|
| Names and Role Provisioning Services (NRPS) | Nej | Integrationen behøver ikke et kursusoverblik; brugeridentiteten følger med hvert launch |
| Assignment and Grade Services (AGS) - lineitem-, score- og result-scopes | Nej | Integrationen er ikke klasseseddel-bevidst |
| Deep Linking ud over standardplaceringens retur | Ingen yderligere data | Deep linking bruges kun til instruktørers placering af værktøjet; intet kursusindhold opregnes |

## Data, Der Ikke Indsamles

Ud over selve LTI anmoder FastComments ikke om eller modtager følgende fra LMS'en eller brugeren:

| Kategori | Indsamlet? |
|----------|------------|
| Studerendes karakterer | Nej |
| Opgaveafleveringer | Nej |
| Fremmøderegistreringer | Nej |
| Fuld kursusoverblik | Nej |
| Offentlige identifikatorer | Nej |
| Fødselsdato | Nej |
| Postadresse eller telefonnummer | Nej |
| Økonomiske oplysninger | Nej |
| LMS-administratorlegitimation | Nej |

## Adgangsgrænser

- FastComments modtager kun data inden for et autoriseret LTI 1.3-launch signeret af LMS'ens registrerede nøgler. Integrationen kalder ikke tilbage til LMS'en for yderligere oplysninger.
- Launch-tokens er til engangsbrug og kortlivede. Genafspillede eller udløbne tokens afvises.
- LMS-administratorer styrer, hvor værktøjet udrulles på deres platform. D2L Brightspace understøtter f.eks. per-deployment org-unit-scoping og per-deployment sikkerhedsindstillinger, hvilket giver administratorer mulighed for at begrænse værktøjet til specifikke kurser eller org units i stedet for at gøre det globalt tilgængeligt. Moodle, Blackboard, Sakai og Schoology tilbyder tilsvarende per-deployment-kontroller i deres LTI 1.3-implementeringer.

## Opbevaring og Tilbageholdelse

FastComments tilbageholder LTI-afledte data i den periode, hvor kommentartjenesten er aktiv, og i henhold til kundekonfigurerede tilbageholdelsesindstillinger. Kommentardata gemmes i krypteret produktionslager (encrypted-at-rest). Ved opsigelse af kontoen eller en skriftlig anmodning om sletning sletter eller anonymiserer FastComments kundedata i overensstemmelse med den gældende aftale.

For fulde detaljer om opbevaring og datahåndtering, se <a href="https://fastcomments.com/privacy-policy" target="_blank">FastComments' Privatlivspolitik</a>.

## Gennemgangsfrekvens

Enhver ny LTI-funktion, der ville kræve yderligere claims, scopes eller LTI Advantage-tjenester, gennemgås før udgivelse for at bekræfte, at den anmodede adgang er nødvendig og proportional med den funktion, der udsendes.

## Kort Erklæring til Sikkerhedsspørgeskemaer

> FastComments anvender mindst mulige privilegium og dataminimering på sin LTI 1.3-integration. Integrationen bruger kun de LTI-launch-claims, der er nødvendige for at autentificere brugeren (`sub`, `name`, `email`, `picture`), bestemme deres rolle og identificere det kursus og den ressource, som kommentarerne hører til. FastComments anmoder ikke om Names and Role Provisioning Services, Assignment and Grade Services, klasseseddeldata, fremmøde, fulde kursusoverblik eller LMS-administratoradgang. LMS-administratorer bevarer kontrollen over, hvilke org units, kurser og deployments værktøjet er tilgængeligt i.