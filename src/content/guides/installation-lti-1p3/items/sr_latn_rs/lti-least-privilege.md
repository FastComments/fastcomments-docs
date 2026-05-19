---
The FastComments LTI 1.3 integracija sledi princip najmanjih privilegija: koristi samo launch claim-ove potrebne da identifikuje korisnika, prikači komentare na odgovarajući kurs i resurs, i primeni dozvole zasnovane na ulozi.

Ostatak ove stranice mapira svaki claim koji integracija koristi, svaki LTI Advantage servis koji ne zahteva, i svaku kategoriju podataka koju ne prikuplja. Recenzenti za bezbednost i nabavku mogu direktno preuzeti odgovore iz tabela ispod.

## Data Elements Received From the LMS

Svako LTI 1.3 pokretanje nosi potpisani JWT od LMS-a. FastComments izdvajaju sledeće claim-ove iz tog JWT-a i ne koriste ništa drugo:

| Polje | LTI claim | Svrha | Potrebno | Skladišteno |
|-------|-----------|--------|----------|-------------|
| User identifier | `sub` | Dosledno identifikuje korisnika preko pokretanja tako da ista osoba odgovara istom FastComments SSO korisniku | Yes | Yes, as part of a stable internal SSO ID |
| Display name | `name` | Pripis prikazan pored korisnikovih komentara | Yes (falls back to "LMS User" if absent) | Yes |
| Email | `email` | Poklapanje naloga, notifikacije, moderacija, korespondencija podrške | Optional (the integration works without it) | Yes when provided |
| Avatar URL | `picture` | Prikazano na korisnikovim komentarima | Optional | URL only; FastComments does not download or rehost the image |
| Roles | `https://purl.imsglobal.org/spec/lti/claim/roles` | Određuje da li je korisnik administrator, instruktor (moderator) ili polaznik | Yes | Derived `isAdmin` / `isModerator` flags on the SSO session |
| Course context | `https://purl.imsglobal.org/spec/lti/claim/context` (`id`, `title`) | Povezuje nit komentara sa odgovarajućim LMS kursom | Yes | Yes, as part of the resolved page identifier |
| Resource link | `https://purl.imsglobal.org/spec/lti/claim/resource_link` (`id`) | Povezuje komentare sa odgovarajućom aktivnošću ili pozicijom alata unutar kursa | Yes when present | Yes, as part of the resolved page identifier |
| Deployment ID | `https://purl.imsglobal.org/spec/lti/claim/deployment_id` | Usmerava pokretanje na odgovarajuću FastComments tenant konfiguraciju | Yes | Yes, on the FastComments LTI configuration record |

## Claims and Scopes Declared at Registration

Tokom LTI 1.3 Dynamic Registration, FastComments se registruje sa `scope: ""` (bez dodatnih OAuth scope-ova) i deklariše samo ove OpenID Connect claim-ove:

`iss`, `sub`, `name`, `email`, `picture`

Registrovana su dva tipa poruka:

- `LtiResourceLinkRequest` - standardno pokretanje kursa u FastComments.
- `LtiDeepLinkingRequest` - omogućava instruktorima da postave FastComments alat unutar kursa.

Nema zahtevanih dodatnih access token-a od LMS-a.

## LTI Advantage Services Not Requested

| Service / scope | Requested? | Reason |
|------------------|------------|--------|
| Names and Role Provisioning Services (NRPS) | No | The integration does not need a course roster; user identity arrives with each launch |
| Assignment and Grade Services (AGS) - lineitem, score, result scopes | No | The integration is not gradebook-aware |
| Deep Linking beyond the standard placement return | No additional data | Deep Linking is used only for instructor placement of the tool; no course content is enumerated |

## Data Not Collected

Pored samog LTI-ja, FastComments ne zahteva niti prima sledeće od LMS-a ili korisnika:

| Kategorija | Prikuplja se? |
|----------|------------|
| Student grades | No |
| Assignment submissions | No |
| Attendance records | No |
| Full course rosters | No |
| Government identifiers | No |
| Date of birth | No |
| Postal address or phone number | No |
| Financial information | No |
| LMS administrator credentials | No |

## Access Boundaries

- FastComments prima podatke samo unutar autorizovanog LTI 1.3 pokretanja potpisanog registrovanim ključevima LMS-a. Integracija ne poziva LMS radi dodatnih informacija.
- Launch tokeni su jednokratni i kratkotrajni. Ponovo upotrebljeni ili istekli tokeni se odbacuju.
- Administratori LMS-a kontrolišu gde je alat postavljen unutar njihove platforme. D2L Brightspace, na primer, podržava per-deployment org-unit scoping i per-deployment security settings, što omogućava administratorima da ograniče alat na određene kurseve ili org unit-e umesto da ga učine dostupnim globalno. Moodle, Blackboard, Sakai i Schoology nude ekvivalentne kontrole po deploymentu u svojim LTI 1.3 implementacijama.

## Storage and Retention

FastComments zadržava LTI-izvedene podatke tokom trajanja aktivne usluge komentarisanja i u skladu sa podešavanjima zadržavanja koja konfiguriše kupac. Podaci o komentarima se čuvaju u produkcijskom skladištu šifrovanom u mirovanju. Po prestanku naloga ili na pisani zahtev za brisanje, FastComments briše ili anonimizuje podatke kupca u skladu sa važećim ugovorom.

Za potpune detalje o skladištenju i rukovanju podacima, pogledajte <a href="https://fastcomments.com/privacy-policy" target="_blank">Politika privatnosti FastComments</a>.

## Review Cadence

Svaka nova LTI funkcija koja bi zahtevala dodatne claim-ove, scope-ove, ili LTI Advantage servise se pregleda pre objavljivanja kako bi se potvrdilo da je traženi pristup neophodan i proporcionalan funkciji koja se isporučuje.

## Short Statement for Security Questionnaires

> FastComments primenjuje princip najmanjih privilegija i minimizaciju podataka za svoju LTI 1.3 integraciju. Integracija koristi samo LTI launch claim-ove potrebne za autentifikaciju korisnika (`sub`, `name`, `email`, `picture`), određivanje njihove uloge, i identifikovanje kursa i resursa kome komentari pripadaju. FastComments ne traži Names and Role Provisioning Services, Assignment and Grade Services, podatke iz knjige ocena, evidenciju prisustva, potpune spiskove učesnika, niti administratorski pristup LMS-u. Administratori LMS-a zadržavaju kontrolu nad tim u kojim org unit-ima, kursevima i deployment-ima je alat dostupan.
---