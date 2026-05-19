FastComments LTI 1.3 integracija slijedi princip najmanjih ovlasti: koristi samo one launch tvrdnje potrebne za identifikaciju korisnika, pridruživanje komentara ispravnom tečaju i resursu te primjenu ovlasti temeljenih na ulozi.

Ostatak ove stranice mapira svako tvrdnju koju integracija troši, svaku LTI Advantage uslugu koju ne traži i svaku kategoriju podataka koju ne prikuplja. Preglednici sigurnosti i nabave mogu izravno preuzeti odgovore iz tablica u nastavku.

## Podaci primljeni od LMS-a

Svako LTI 1.3 lansiranje nosi potpisani JWT od LMS‑a. FastComments izvlači sljedeće tvrdnje iz tog JWT‑a i ne koristi ništa drugo:

| Polje | LTI tvrdnja | Svrha | Obavezno | Pohranjeno |
|-------|-------------|-------|----------|------------|
| Identifikator korisnika | `sub` | Dosljedno identificira korisnika između pokretanja tako da ista osoba odgovara istom FastComments SSO korisniku | Da | Da, kao dio stabilnog internog SSO ID‑a |
| Prikazano ime | `name` | Pripis prikazan pored korisnikovih komentara | Da (u nedostatku se koristi "Korisnik LMS‑a") | Da |
| E‑pošta | `email` | Usklađivanje računa, obavijesti, moderacija, korespondencija podrške | Opcionalno (integracija radi i bez njega) | Da, ako je dostavljena |
| URL avatara | `picture` | Prikazuje se uz korisnikove komentare | Opcionalno | Samo URL; FastComments ne preuzima niti ponovno hosta sliku |
| Uloge | `https://purl.imsglobal.org/spec/lti/claim/roles` | Određuje je li korisnik administrator, instruktor (moderator) ili polaznik | Da | Izvedene `isAdmin` / `isModerator` zastavice na SSO sesiji |
| Kontekst tečaja | `https://purl.imsglobal.org/spec/lti/claim/context` (`id`, `title`) | Povezuje nit komentara s ispravnim LMS tečajem | Da | Da, kao dio razriješenog identifikatora stranice |
| Poveznica resursa | `https://purl.imsglobal.org/spec/lti/claim/resource_link` (`id`) | Povezuje komentare sa ispravnom aktivnošću ili postavkom alata unutar tečaja | Da kada je prisutno | Da, kao dio razriješenog identifikatora stranice |
| ID implementacije | `https://purl.imsglobal.org/spec/lti/claim/deployment_id` | Usmjerava lansiranje na ispravnu konfiguraciju FastComments najmoprimca | Da | Da, u FastComments LTI zapisu konfiguracije |

## Tvrdnje i opsezi deklarirani pri registraciji

Tijekom LTI 1.3 Dinamičke registracije, FastComments se registrira s `scope: ""` (bez dodatnih OAuth opsega) i deklarira samo ove OpenID Connect tvrdnje:

`iss`, `sub`, `name`, `email`, `picture`

Registrira dvije vrste poruka:

- `LtiResourceLinkRequest` - standardno lansiranje tečaja u FastComments.
- `LtiDeepLinkingRequest` - omogućava instruktorima postavljanje FastComments alata unutar tečaja.

Iz LMS‑a se ne zahtijevaju dodatni pristupni tokeni.

## LTI Advantage usluge koje nisu zatražene

| Usluga / opseg | Zahtijevano? | Razlog |
|----------------|--------------|--------|
| Names and Role Provisioning Services (NRPS) | Ne | Integracija ne treba popis sudionika; identitet korisnika dolazi sa svakim lansiranjem |
| Assignment and Grade Services (AGS) - lineitem, score, result scopes | Ne | Integracija nije povezana s dnevnikom ocjena |
| Deep Linking beyond the standard placement return | Ne - nema dodatnih podataka | Deep linking se koristi samo za postavljanje alata od strane instruktora; sadržaj tečaja se ne nabraja |

## Podaci koji se ne prikupljaju

Osim samog LTI‑ja, FastComments ne traži niti prima sljedeće od LMS‑a ili korisnika:

| Kategorija | Prikuplja se? |
|------------|---------------|
| Ocjene studenata | Ne |
| Predaja zadaća | Ne |
| Evidencija prisustva | Ne |
| Puni popisi sudionika | Ne |
| Državni identifikatori | Ne |
| Datum rođenja | Ne |
| Poštanska adresa ili broj telefona | Ne |
| Financijske informacije | Ne |
| Podaci za prijavu LMS administratora | Ne |

## Granice pristupa

- FastComments prima podatke samo unutar ovlaštenog LTI 1.3 lansiranja potpisanog registriranim ključevima LMS‑a. Integracija se ne obraća LMS‑u za dodatne informacije.
- Tokeni lansiranja su jednokratni i kratkotrajni. Ponovno upotrijebljeni ili istekli tokeni se odbacuju.
- Administratori LMS‑a kontroliraju gdje je alat implementiran unutar njihove platforme. D2L Brightspace, na primjer, podržava ograničavanje po org‑jednici za svaku implementaciju i sigurnosne postavke po implementaciji, što omogućava administratorima da ograniče alat na određene tečajeve ili org‑jednice umjesto da ga učine dostupnim globalno. Moodle, Blackboard, Sakai i Schoology nude ekvivalentne kontrole po implementaciji u svojim LTI 1.3 implementacijama.

## Pohrana i zadržavanje

FastComments zadržava podatke izvedene iz LTI‑ja tijekom trajanja aktivne usluge komentiranja i u skladu s postavkama zadržavanja koje konfigurira kupac. Podaci o komentarima pohranjuju se u produkcijsko spremište šifrirano u mirovanju. Pri raskidu računa ili pisanom zahtjevu za brisanjem, FastComments briše ili anonimizira podatke kupca u skladu s primjenjivim ugovorom.

Za potpune detalje o pohrani i rukovanju podacima, vidi <a href="https://fastcomments.com/privacy-policy" target="_blank">Politika privatnosti FastComments</a>.

## Ritam revizije

Svaka nova LTI značajka koja bi zahtijevala dodatne tvrdnje, opsege ili LTI Advantage usluge pregledava se prije izdanja kako bi se potvrdilo da je traženi pristup potreban i razmjeran značajci koja se isporučuje.

## Kratka izjava za upitnike o sigurnosti

> FastComments primjenjuje najmanje ovlasti i minimizaciju podataka za svoju LTI 1.3 integraciju. Integracija koristi samo LTI launch tvrdnje potrebne za autentikaciju korisnika (`sub`, `name`, `email`, `picture`), određivanje njihove uloge i identifikaciju tečaja i resursa kojem komentari pripadaju. FastComments ne traži Names and Role Provisioning Services, Assignment and Grade Services, podatke iz dnevnika ocjena, evidenciju prisustva, pune popise sudionika ili administrativni pristup LMS‑u. Administratori LMS‑a zadržavaju kontrolu nad tim u kojim org‑jedinicama, tečajevima i implementacijama je alat dostupan.