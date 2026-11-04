FastComments LTI 1.3 integracija sledi princip najmanjih privilegija: koristi samo launch claim podatke potrebne za identifikaciju korisnika, povezivanje komentara sa ispravnim kursom i resursom, i primenu dozvola zasnovanih na ulogama.

Ostatak ove stranice mapira svaki claim koji integracija koristi, svaku LTI Advantage uslugu koju ne zahteva, i svaku kategoriju podataka koje ne prikuplja. Recenzenti bezbednosti i nabavke mogu preuzeti odgovore direktno iz tabela ispod.

## Elementi Podataka Primljeni od LMS-a

Svako LTI 1.3 lansiranje nosi potpisani JWT od LMS-a. FastComments izdvaja sledeće claim podatke iz tog JWT-a i ne koristi ništa drugo:

| Polje | LTI claim | Svrha | Obavezno | Čuva se |
|-------|-----------|---------|----------|--------|
| Identifikator korisnika | `sub` | Identifikuje korisnika dosledno kroz lansiranja tako da ista osoba bude povezana sa istim FastComments SSO korisnikom | Da | Da, kao deo stabilnog internog SSO ID-ja |
| Ime za prikaz | `name` | Autorstvo prikazano pored komentara korisnika | Da (vraća se na "LMS User" ako nedostaje) | Da |
| Email | `email` | Uparivanje naloga, notifikacije, moderacija, korespondencija podrške | Opciono (integracija radi i bez njega) | Da kada je dostavljen |
| URL avatara | `picture` | Prikazuje se na komentarima korisnika | Opciono | Samo URL; FastComments ne preuzima ni ne re-hostuje sliku |
| Uloge | `https://purl.imsglobal.org/spec/lti/claim/roles` | Određuje da li je korisnik administrator, instruktor (moderator) ili učenik | Da | Izvedeni `isAdmin` / `isModerator` flagovi na SSO sesiji |
| Kontekst kursa | `https://purl.imsglobal.org/spec/lti/claim/context` (`id`, `title`) | Povezuje nit komentara sa ispravnim LMS kursom | Da | Da, kao deo razrešenog identifikatora stranice |
| Link resursa | `https://purl.imsglobal.org/spec/lti/claim/resource_link` (`id`) | Povezuje komentare sa ispravnom aktivnošću ili plasmanom alata unutar kursa | Da kada je prisutan | Da, kao deo razrešenog identifikatora stranice |
| Deployment ID | `https://purl.imsglobal.org/spec/lti/claim/deployment_id` | Usmerava lansiranje ka ispravnoj FastComments tenant konfiguraciji | Da | Da, na FastComments LTI konfiguracionom zapisu |

## Claim Podaci i Opsezi Deklarisani pri Registraciji

Tokom LTI 1.3 Dynamic Registration, FastComments registruje sebe sa `scope: ""` (bez dodatnih OAuth opsega) i deklariše samo ove OpenID Connect claim podatke:

`iss`, `sub`, `name`, `email`, `picture`

Registruje dva tipa poruka:

- `LtiResourceLinkRequest` - standardno lansiranje kursa u FastComments.
- `LtiDeepLinkingRequest` - omogućava instruktorima da plasiraju FastComments alat unutar kursa.

Nisu zahtevani dodatni access tokeni od LMS-a.

## LTI Advantage Usluge koje Nisu Zahtevane

| Usluga / opseg | Zahtevana? | Razlog |
|------------------|------------|--------|
| Names and Role Provisioning Services (NRPS) | Ne | Integracija ne treba spisak kursa; identitet korisnika stiže sa svakim lansiranjem |
| Assignment and Grade Services (AGS) - lineitem, score, result opsezi | Ne | Integracija nije svesna evidencije ocena |
| Deep Linking van standardnog povratka plasmana | Bez dodatnih podataka | Deep linking se koristi samo za plasman alata od strane instruktora; sadržaj kursa se ne nabraja |

## Podaci koji se Ne Prikupljaju

Van samog LTI-ja, FastComments ne zahteva ni ne prima sledeće od LMS-a ili korisnika:

| Kategorija | Prikupljeno? |
|----------|------------|
| Ocene studenata | Ne |
| Predaje zadataka | Ne |
| Evidencije prisustva | Ne |
| Kompletni spiskovi kursa | Ne |
| Državni identifikatori | Ne |
| Datum rođenja | Ne |
| Poštana adresa ili broj telefona | Ne |
| Finansijske informacije | Ne |
| Administratorski kredencijali LMS-a | Ne |

## Granice Pristupa

- FastComments prima podatke samo unutar autorizovanog LTI 1.3 lansiranja potpisanog registrovanim ključevima LMS-a. Integracija ne poziva nazad LMS za dodatne informacije.
- Launch tokeni su jednokratni i kratkotrajni. Replicirani ili istekli tokeni se odbacuju.
- LMS administratori kontrolišu gde je alat raspoređen unutar svoje platforme. D2L Brightspace, na primer, podržava per-deployment org-unit scoping i per-deployment bezbednosne postavke, što omogućava administratorima da ograniče alat na konkretne kurseve ili org jedinice umesto da ga učine globalno dostupnim. Moodle, Blackboard, Sakai i Schoology nude ekvivalentne per-deployment kontrole u svojim LTI 1.3 implementacijama.

## Čuvanje i Zadržavanje

FastComments zadržava LTI-izvedene podatke tokom trajanja aktivne usluge komentara i prema postavkama zadržavanja koje konfiguriše kupac. Podaci komentara se čuvaju u produkcionom skladištu šifrovanom u mirovanju. Po prestanku naloga ili pisanom zahtevu za brisanje, FastComments briše ili anonimizuje podatke kupca prema važećem ugovoru.

Za kompletne detalje o čuvanju i rukovanju podacima, pogledajte <a href="https://fastcomments.com/privacy-policy" target="_blank">FastComments Politiku Privatnosti</a>.

## Kadencija Pregleda

Svaka nova LTI funkcija koja bi zahtevala dodatne claim podatke, opsege ili LTI Advantage usluge se pregleda pre objavljivanja kako bi se potvrdilo da je traženi pristup neophodan i srazmeran funkciji koja se isporučuje.

## Kratka Izjava za Bezbednosne Upitnike

> FastComments primenjuje najmanje privilegije i minimizaciju podataka na svoju LTI 1.3 integraciju. Integracija koristi samo LTI launch claim podatke potrebne za autentifikaciju korisnika (`sub`, `name`, `email`, `picture`), određivanje njihove uloge, i identifikaciju kursa i resursa kojima komentari pripadaju. FastComments ne zahteva Names and Role Provisioning Services, Assignment and Grade Services, podatke o ocenama, prisustvo, kompletne spiskove, ni administrativni pristup LMS-u. LMS administratori zadržavaju kontrolu nad org jedinicama, kursevima i deploymentima u kojima je alat dostupan.