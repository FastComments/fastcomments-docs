---
FastComments LTI 1.3 integracija sledi načelu najmanjše privilegiranosti: uporablja le zagonske trditve, potrebne za identifikacijo uporabnika, pripenjanje komentarjev k pravilnemu predmetu in viru ter uporabo dovoljenj na podlagi vlog.

Preostanek te strani prikaže vsako trditev, ki jo integracija uporablja, vsako LTI Advantage storitev, ki je ne zahteva, in vsako kategorijo podatkov, ki jih ne zbira. Preverjevalci varnosti in nabave lahko odgovore neposredno prekopirajo iz spodnjih tabel.

## Elementi podatkov, pridobljeni iz LMS

Vsak LTI 1.3 zagon prenaša podpisan JWT iz LMS. FastComments iz tega JWT izlušči naslednje trditve in ne uporablja ničesar drugega:

| Field | LTI claim | Purpose | Required | Stored |
|-------|-----------|---------|----------|--------|
| User identifier | `sub` | Identificira uporabnika dosledno med zagoni, tako da ista oseba ustreza istemu FastComments SSO uporabniku | Da | Da, kot del stabilnega notranjega SSO ID |
| Display name | `name` | Pripis, prikazan ob uporabnikovih komentarjih | Da (falls back to "LMS User" if absent) | Da |
| Email | `email` | Ujemanje računov, obvestila, moderacija, podpora | Neobvezno (integracija deluje brez njega) | Da, če je posredovano |
| Avatar URL | `picture` | Prikazan ob uporabnikovih komentarjih | Neobvezno | Samo URL; FastComments slike ne prenese ali ponovno ne gosti |
| Roles | `https://purl.imsglobal.org/spec/lti/claim/roles` | Določa, ali je uporabnik administrator, inštruktor (moderator) ali učeči se | Da | Izvedene zastavice `isAdmin` / `isModerator` na SSO seji |
| Course context | `https://purl.imsglobal.org/spec/lti/claim/context` (`id`, `title`) | Poveže nit komentarjev s pravilnim predmetom v LMS | Da | Da, kot del razrešenega identifikatorja strani |
| Resource link | `https://purl.imsglobal.org/spec/lti/claim/resource_link` (`id`) | Poveže komentarje z ustrezno aktivnostjo ali postavitvijo orodja znotraj predmeta | Da, kadar je prisotno | Da, kot del razrešenega identifikatorja strani |
| Deployment ID | `https://purl.imsglobal.org/spec/lti/claim/deployment_id` | Usmeri zagon na pravilno konfiguracijo FastComments najemnika | Da | Da, v zapisu konfiguracije FastComments LTI |

## Trditve in obsegi, deklarirani ob registraciji

Med dinamično registracijo LTI 1.3 se FastComments registrira z `scope: ""` (brez dodatnih OAuth obsegov) in deklarira samo te OpenID Connect trditve:

`iss`, `sub`, `name`, `email`, `picture`

Registrira dva tipa sporočil:

- `LtiResourceLinkRequest` - standardni zagon predmeta v FastComments.
- `LtiDeepLinkingRequest` - omogoča inštruktorjem, da vstavijo orodje FastComments v predmet.

Dodatnih dostopnih žetonov iz LMS ni zaprošenih.

## LTI Advantage storitve, ki niso zahtevane

| Service / scope | Requested? | Reason |
|------------------|------------|--------|
| Names and Role Provisioning Services (NRPS) | Ne | Integracija ne potrebuje seznama udeležencev predmeta; identiteta uporabnika prispe z vsakim zagonom |
| Assignment and Grade Services (AGS) - lineitem, score, result scopes | Ne | Integracija ne sodeluje z ocenami |
| Deep Linking beyond the standard placement return | Ni dodatnih podatkov | Deep linking se uporablja samo za namestitev orodja s strani inštruktorja; nobena vsebina predmeta se ne izpiše |

## Podatki, ki niso zbrani

Poleg samega LTI FastComments ne zahteva in ne prejme naslednjega iz LMS ali od uporabnika:

| Category | Collected? |
|----------|------------|
| Student grades | Ne |
| Assignment submissions | Ne |
| Attendance records | Ne |
| Full course rosters | Ne |
| Government identifiers | Ne |
| Date of birth | Ne |
| Postal address or phone number | Ne |
| Financial information | Ne |
| LMS administrator credentials | Ne |

## Meje dostopa

- FastComments prejema podatke le znotraj pooblaščenega LTI 1.3 zagona, podpisanega z registriranimi ključi LMS. Integracija ne kliče nazaj v LMS za dodatne informacije.
- Zagonski žetoni so enkratne uporabe in kratkotrajni. Ponovno predvajani ali potekli žetoni so zavrnjeni.
- LMS skrbniki nadzorujejo, kje je orodje nameščeno znotraj njihove platforme. D2L Brightspace, na primer, podpira določanje obsega po namestitvi za org-enote in varnostne nastavitve po namestitvi, kar omogoča skrbnikom, da omejijo orodje na določene predmete ali org-enote namesto da bi ga naredili globalno dostopnega. Moodle, Blackboard, Sakai in Schoology ponujajo enakovredne kontrole po namestitvi v svojih implementacijah LTI 1.3.

## Shranjevanje in zadrževanje

FastComments hrani podatke, izpeljane iz LTI, za obdobje aktivne storitve komentiranja in v skladu s nastavitvami zadrževanja, ki jih nastavi stranka. Podatki komentarjev so shranjeni v produkcijskem pomnilniku, šifriranem v mirovanju. Ob prekinitvi računa ali pisni zahtevi za izbris FastComments izbriše ali anonimizira podatke stranke v skladu z veljavnim dogovorom.

Za podrobne informacije o shranjevanju in ravnanju s podatki glejte <a href="https://fastcomments.com/privacy-policy" target="_blank">Politika zasebnosti FastComments</a>.

## Frekvenca pregledov

Vsaka nova LTI funkcija, ki bi zahtevala dodatne trditve, obsege ali LTI Advantage storitve, je pred izdajo pregledana, da se potrdi, da je zahtevan dostop potreben in sorazmeren glede na funkcijo, ki se bo izdala.

## Kratek zapis za vprašalnike o varnosti

> FastComments uporablja načelo najmanjšega privilegija in minimizacijo podatkov pri svoji LTI 1.3 integraciji. Integracija uporablja le LTI zagonske trditve, potrebne za overjanje uporabnika (`sub`, `name`, `email`, `picture`), določitev njihove vloge in identifikacijo predmeta ter vira, kateremu pripadajo komentarji. FastComments ne zahteva Names and Role Provisioning Services, Assignment and Grade Services, podatkov iz dnevnika ocen, prisotnosti, popolnih seznamov udeležencev ali administrativnega dostopa do LMS. LMS skrbniki obdržijo nadzor nad tem, v katerih org-enotah, predmetih in namestitvah je orodje na voljo.
---