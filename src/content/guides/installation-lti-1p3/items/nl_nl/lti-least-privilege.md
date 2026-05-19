De FastComments LTI 1.3-integratie volgt het principe van minste privilege: het gebruikt alleen de launch-claims die nodig zijn om de gebruiker te identificeren, reacties aan de juiste cursus en bron te koppelen en rolgebaseerde machtigingen toe te passen.

De rest van deze pagina brengt elke claim in kaart die de integratie gebruikt, elke LTI Advantage-service die niet wordt opgevraagd en elke categorie gegevens die niet wordt verzameld. Beoordelaars van beveiliging en inkoop kunnen antwoorden rechtstreeks uit de onderstaande tabellen overnemen.

## Data Elements Received From the LMS

Every LTI 1.3 launch carries a signed JWT from the LMS. FastComments extracts the following claims from that JWT and uses nothing else:

| Veld | LTI-claim | Doel | Vereist | Opgeslagen |
|-------|-----------|---------|----------|--------|
| Gebruikersidentificatie | `sub` | Identificeert de gebruiker consistent over launches zodat dezelfde persoon naar dezelfde FastComments SSO-gebruiker wordt geresolved | Ja | Ja, als onderdeel van een stabiele interne SSO-ID |
| Weergavenaam | `name` | Toeschrijving die naast de opmerkingen van de gebruiker wordt weergegeven | Ja (valt terug op "LMS-gebruiker" als afwezig) | Ja |
| Email | `email` | Accountmatching, meldingen, moderatie, ondersteuningscorrespondentie | Optioneel (de integratie werkt ook zonder) | Ja wanneer opgegeven |
| Avatar-URL | `picture` | Weergegeven bij de opmerkingen van de gebruiker | Optioneel | Alleen de URL; FastComments downloadt of host de afbeelding niet opnieuw |
| Rollen | `https://purl.imsglobal.org/spec/lti/claim/roles` | Bepaalt of de gebruiker beheerder, docent (moderator) of leerling is | Ja | Afgeleide `isAdmin` / `isModerator`-vlaggen op de SSO-sessie |
| Cursuscontext | `https://purl.imsglobal.org/spec/lti/claim/context` (`id`, `title`) | Koppelt de reactiedraad aan de juiste LMS-cursus | Ja | Ja, als onderdeel van de opgeloste pagina-identificatie |
| Resource link | `https://purl.imsglobal.org/spec/lti/claim/resource_link` (`id`) | Koppelt reacties aan de juiste activiteit of toolplaatsing binnen de cursus | Ja wanneer aanwezig | Ja, als onderdeel van de opgeloste pagina-identificatie |
| Deployment ID | `https://purl.imsglobal.org/spec/lti/claim/deployment_id` | Leid de launch naar de juiste FastComments-tenantconfiguratie | Ja | Ja, op het FastComments LTI-configuratierecord |

## Claims and Scopes Declared at Registration

During LTI 1.3 Dynamic Registration, FastComments registers itself with `scope: ""` (no additional OAuth scopes) and declares only these OpenID Connect claims:

`iss`, `sub`, `name`, `email`, `picture`

It registers two message types:

- `LtiResourceLinkRequest` - de standaard cursuslaunch naar FastComments.
- `LtiDeepLinkingRequest` - stelt docenten in staat om de FastComments-tool binnen een cursus te plaatsen.

No additional access tokens are requested from the LMS.

## LTI Advantage Services Not Requested

| Service / scope | Requested? | Reason |
|------------------|------------|--------|
| Names and Role Provisioning Services (NRPS) | Nee | De integratie heeft geen cursusrooster nodig; gebruikersidentiteit wordt bij elke launch meegeleverd |
| Assignment and Grade Services (AGS) - lineitem, score, result scopes | Nee | De integratie werkt niet met het cijferregister |
| Deep Linking beyond the standard placement return | No additional data | Deep linking wordt alleen gebruikt voor plaatsing door docenten; er wordt geen cursusinhoud opgelijst |

## Data Not Collected

Beyond LTI itself, FastComments does not request or receive the following from the LMS or user:

| Categorie | Verzameld? |
|----------|------------|
| Studentcijfers | Nee |
| Inzendingen van opdrachten | Nee |
| Aanwezigheidsregistraties | Nee |
| Volledige cursusroosters | Nee |
| Overheidsidentificatoren | Nee |
| Geboortedatum | Nee |
| Postadres of telefoonnummer | Nee |
| Financiële informatie | Nee |
| Inloggegevens van LMS-beheerders | Nee |

## Access Boundaries

- FastComments ontvangt alleen gegevens binnen een geautoriseerde LTI 1.3-launch die is ondertekend met de geregistreerde sleutels van het LMS. De integratie roept het LMS niet terug aan voor aanvullende informatie.
- Launch-tokens zijn eenmalig en kortstondig. Herhaalde of verlopen tokens worden geweigerd.
- LMS-beheerders bepalen waar de tool binnen hun platform wordt ingezet. D2L Brightspace ondersteunt bijvoorbeeld per-deployment org-unit-scoping en per-deployment beveiligingsinstellingen, waarmee beheerders de tool kunnen beperken tot specifieke cursussen of org-eenheden in plaats van deze wereldwijd beschikbaar te maken. Moodle, Blackboard, Sakai en Schoology bieden in hun LTI 1.3-implementaties equivalente per-deployment controles.

## Storage and Retention

FastComments bewaart LTI-afgeleide gegevens voor de duur van de actieve reactiedienst en volgens door de klant geconfigureerde retentie-instellingen. Reactiegegevens worden opgeslagen in productieopslag die versleuteld is in rust. Bij accountbeëindiging of schriftelijk verzoek tot verwijdering verwijdert of anonimiseert FastComments klantgegevens overeenkomstig de toepasselijke overeenkomst.

For full storage and data-handling details, see the <a href="https://fastcomments.com/privacy-policy" target="_blank">Privacybeleid van FastComments</a>.

## Review Cadence

Any new LTI feature that would require additional claims, scopes, or LTI Advantage services is reviewed before release to confirm the requested access is necessary and proportionate to the feature being shipped.

## Short Statement for Security Questionnaires

> FastComments past minste privilege en gegevensminimalisatie toe op zijn LTI 1.3-integratie. De integratie gebruikt alleen de LTI-launchclaims die nodig zijn om de gebruiker te authenticeren (`sub`, `name`, `email`, `picture`), hun rol te bepalen en de cursus en bron te identificeren waartoe reacties behoren. FastComments vraagt geen Names and Role Provisioning Services, Assignment and Grade Services, cijferboekgegevens, aanwezigheid, volledige roosters of LMS-beheerders-toegang op. LMS-beheerders behouden de controle over welke org-eenheden, cursussen en deployments de tool beschikbaar is.