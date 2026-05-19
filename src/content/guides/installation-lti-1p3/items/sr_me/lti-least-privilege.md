The FastComments LTI 1.3 integracija slijedi princip najmanjih privilegija: koristi samo launch iskaze potrebne za identifikaciju korisnika, povezivanje komentara sa ispravnim kursom i resursom, i primjenu dozvola zasnovanih na ulogama.

Ostatak ove stranice mapira svaki iskaz koji integracija koristi, svaku LTI Advantage uslugu koju ne zahtijeva, i svaku kategoriju podataka koju ne prikuplja. Recenzenti za sigurnost i nabavku mogu direktno preuzeti odgovore iz tabela ispod.

## Podaci primljeni od LMS-a

Svaki LTI 1.3 launch nosi potpisani JWT od LMS-a. FastComments izvlači sljedeće iskaze iz tog JWT-a i ne koristi ništa drugo:

| Field | LTI claim | Purpose | Required | Stored |
|-------|-----------|---------|----------|--------|
| User identifier | `sub` | Identifikuje korisnika konzistentno kroz launcheve tako da ista osoba odgovara istom FastComments SSO nalogu | Yes | Yes, as part of a stable internal SSO ID |
| Display name | `name` | Pripis prikazan pored korisnikovih komentara | Yes (falls back to "LMS User" if absent) | Yes |
| Email | `email` | Uparivanje naloga, obavještenja, moderacija, komunikacija podrške | Optional (the integration works without it) | Yes when provided |
| Avatar URL | `picture` | Prikazano na korisnikovim komentarima | Optional | URL only; FastComments does not download or rehost the image |
| Roles | `https://purl.imsglobal.org/spec/lti/claim/roles` | Određuje da li je korisnik administrator, instruktor (moderator) ili učenik | Yes | Derived `isAdmin` / `isModerator` flags on the SSO session |
| Course context | `https://purl.imsglobal.org/spec/lti/claim/context` (`id`, `title`) | Povezuje thread komentara sa ispravnim LMS kursom | Yes | Yes, as part of the resolved page identifier |
| Resource link | `https://purl.imsglobal.org/spec/lti/claim/resource_link` (`id`) | Povezuje komentare sa pravilnom aktivnošću ili pozicijom alata unutar kursa | Yes when present | Yes, as part of the resolved page identifier |
| Deployment ID | `https://purl.imsglobal.org/spec/lti/claim/deployment_id` | Usmjerava launch na ispravnu FastComments tenant konfiguraciju | Yes | Yes, on the FastComments LTI configuration record |

## Iskazi i opsezi deklarisani pri registraciji

Tokom LTI 1.3 Dynamic Registration, FastComments se registruje sa `scope: ""` (bez dodatnih OAuth opsega) i deklarise samo ove OpenID Connect iskaze:

`iss`, `sub`, `name`, `email`, `picture`

Registruje dvije vrste poruka:

- `LtiResourceLinkRequest` - standardni course launch u FastComments.
- `LtiDeepLinkingRequest` - omogućava instruktorima da postave FastComments alat unutar kursa.

Nijedan dodatni pristupni token nije zatražen od LMS-a.

## LTI Advantage usluge koje nisu zatražene

| Service / scope | Requested? | Reason |
|------------------|------------|--------|
| Names and Role Provisioning Services (NRPS) | No | Integraciji ne treba spisak kursa; identitet korisnika stiže sa svakim launchom |
| Assignment and Grade Services (AGS) - lineitem, score, result scopes | No | Integracija nije povezana sa dnevnikom ocjena |
| Deep Linking beyond the standard placement return | No additional data | Deep linking se koristi samo za postavljanje alata od strane instruktora; nijedan sadržaj kursa se ne nabraja |

## Podaci koji se ne prikupljaju

Pored samog LTI-a, FastComments ne zahtijeva niti prima sljedeće od LMS-a ili korisnika:

| Category | Collected? |
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

## Granice pristupa

- FastComments prima podatke samo unutar autorizovanog LTI 1.3 launcha potpisanog ključevima registrovanim kod LMS-a. Integracija ne poziva LMS za dodatne informacije.
- Launch tokeni su jednokratni i kratkotrajni. Ponovo korišćeni ili istekli tokeni se odbijaju.
- LMS administratori kontrolišu gdje je alat raspoređen unutar njihove platforme. D2L Brightspace, na primjer, podržava per-deployment org-unit scoping i per-deployment sigurnosna podešavanja, što omogućava administratorima da ograniče alat na određene kurseve ili org jedinice umjesto da ga učine dostupnim globalno. Moodle, Blackboard, Sakai i Schoology nude ekvivalentne kontrole po implementaciji u svojim LTI 1.3 implementacijama.

## Pohrana i zadržavanje

FastComments čuva LTI-izvedene podatke tokom trajanja aktivne usluge komentarisanja i u skladu sa postavkama zadržavanja koje je konfigurisao kupac. Podaci komentara se pohranjuju u produkcijsko skladište sa enkripcijom u mirovanju. Prilikom raskida naloga ili pismenog zahtjeva za brisanje, FastComments briše ili anonimizuje podatke kupca u skladu sa primjenjivim ugovorom.

Za potpune detalje o pohrani i rukovanju podacima, pogledajte <a href="https://fastcomments.com/privacy-policy" target="_blank">Politika privatnosti FastComments</a>.

## Ritam revizije

Svaka nova LTI funkcija koja bi zahtijevala dodatne iskaze, opsege ili LTI Advantage usluge se pregledava prije objave kako bi se potvrdilo da je zatraženi pristup neophodan i proporcionalan funkcionalnosti koja se isporučuje.

## Kratka izjava za upitnike o sigurnosti

> FastComments primjenjuje princip najmanjih privilegija i minimizaciju podataka u svojoj LTI 1.3 integraciji. Integracija koristi samo LTI launch iskaze potrebne za autentifikaciju korisnika (`sub`, `name`, `email`, `picture`), određivanje njihove uloge, i identifikovanje kursa i resursa kojem komentari pripadaju. FastComments ne zahtijeva Names and Role Provisioning Services, Assignment and Grade Services, podatke iz dnevnika ocjena, prisustvo, potpuno spiskove polaznika, niti administratorski pristup LMS-u. LMS administratori zadržavaju kontrolu nad tim u kojim org jedinicama, kursevima i deploymentima je alat dostupan.