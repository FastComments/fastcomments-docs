Ovaj vodič objašnjava dodavanje FastComments u Moodle 4.x kolegij nakon što je administrator stranice registrirao alat i postavio ga da se prikazuje u izborniku aktivnosti. Ako FastComments još nije registriran, prvo pogledajte vodič za registraciju Moodlea.

#### Otvorite kolegij u načinu uređivanja

1. Prijavite se u Moodle kao Nastavnik s pravom uređivanja (ili viša uloga) za taj kolegij.
2. Otvorite kolegij.
3. Uključite **Način uređivanja** koristeći prekidač u gornjem desnom kutu zaglavlja kolegija.

Moodle 4.x zamijenio je stari padajući izbornik "Dodaj aktivnost ili resurs" koji je koristio Moodle 3.x punopločnim dijalogom za odabir aktivnosti. Moodle 4.5 zadržava isti odabir, ali dodaje red s označenim/zadanim favoritima na vrhu, pa učvršćivanje FastComments-a jednom olakšava kasniji pristup u sljedećim sekcijama.

#### Dodajte FastComments aktivnost

1. Pomaknite se do odjeljka kolegija (tema ili tjedan) gdje rasprava pripada.
2. Kliknite **Dodaj aktivnost ili resurs** na dnu tog odjeljka.
3. U dijalogu za odabir odaberite **FastComments**. Ako ga ne vidite, prijeđite na odjeljak o problemima u nastavku.

Forma za postavke aktivnosti se otvara. Polja koja su bitna:

- **Naziv aktivnosti** (obavezno). Prikazuje se na stranici kolegija i u knjizi ocjena. Primjer: `Week 3 Discussion`.
- **Opis aktivnosti**. Opcionalni uvodni tekst koji se prikazuje iznad niti komentara.
- **Prikaži opis na stranici kolegija**. Označite ako želite da opis bude vidljiv bez otvaranja aktivnosti.
- **Prethodno konfiguriran alat**. Postavljeno na `FastComments` (automatski odabrano kada se pokreće iz izbornika). Ne mijenjajte.
- **Način pokretanja**. Postavite na **Novi prozor**. Pogledajte odjeljak o problemima zašto "Isti prozor" ponekad ne radi u nekim Moodle instalacijama.
- **Tool URL**, **Public key**, **Shared secret**, **Custom parameters**. Ostavite prazno. Dinamička registracija je to riješila na razini stranice.

Pomaknite se prema dnu i kliknite **Spremi i vrati se na kolegij** (ili **Spremi i prikaži** za odmah otvaranje aktivnosti).

Aktivnost se pojavljuje kao red u odjeljku s ikonom FastComments-a. Studenti kliknu red za otvaranje niti komentara.

#### Umetnite FastComments ugrađeno u editor

Za nit unutar Stranice, Poglavlja knjige, Lekcije ili bilo kojeg drugog resursa koji koristi Atto ili TinyMCE editor:

1. Otvorite resurs u načinu uređivanja.
2. Postavite kursor gdje nit treba biti.
3. U alatnoj traci editora kliknite gumb **LTI** / **Vanjski alat**. U Attu je označen "Umetni LTI Advantage sadržaj". U TinyMCE-u (zadano u Moodle 4.3+) nalazi se pod izbornikom **Više** kao **Vanjski alati**.
4. Odaberite **FastComments** s popisa alata.
5. FastComments otvara odabir dubokog povezivanja. Potvrdite naslov niti i kliknite **Umetni**.
6. Editor umeće LTI rezervirani blok. Spremite resurs.

Svaka ugrađena instanca je zasebna nit identificirana ID-jem stavke sadržaja dubokog povezivanja, pa Stranica s tri FastComments ugradnje dobije tri neovisne niti.

#### Ograničavanje pristupa i postavke grupa

Standardne postavke aktivnosti Moodlea vrijede za FastComments aktivnosti:

- **Opće postavke modula** > **Način rada grupa**. Postavljanje na **Odvojene grupe** ili **Vidljive grupe** samo po sebi ne dijeli FastComments u niti po grupama. Način rada grupa u Moodlu samo filtrira knjigu ocjena i popis članova. Za vođenje odvojene niti po grupi dodajte jednu FastComments aktivnost po grupi i koristite **Ograniči pristup** da ograničite svaku.
- **Ograniči pristup** > **Dodaj ograničenje**. Podržava standardne Moodle uvjete: **Datum**, **Ocjena**, **Grupa**, **Grupiranje**, **Korisnički profil** i ugniježđene skupove ograničenja. Koristite **Grupa** za zaključavanje FastComments aktivnosti za jednu grupu.
- **Završetak aktivnosti**. Postavite na **Studenti moraju pogledati ovu aktivnost da bi je dovršili** ako želite praćenje dovršenosti. FastComments trenutno ne šalje događaj dovršenja natrag Moodlu izvan pokretanja.

#### Mapiranje uloga

FastComments čita LTI `roles` tvrdnju koju Moodle šalje pri svakom pokretanju i preslikava je na sljedeći način:

- Moodle **Manager** ili **Site administrator** -> FastComments **admin**
- Moodle **Editing teacher** ili **Non-editing teacher** -> FastComments **moderator**
- Moodle **Student** -> FastComments **commenter**
- Moodle **Guest** -> samo za čitanje

Admini mogu brisati bilo koji komentar, zabranjivati korisnike i uređivati postavke niti. Moderatori mogu brisati i odobravati komentare unutar niti u koju su pokrenuli alat. Prilagođene Moodle uloge nasljeđuju mapiranje arhetipa iz kojeg su klonirane.

#### Što studenti vide

Studenti kliknu FastComments aktivnost (ili se pomaknu do ugrađenog bloka unutar Stranice ili Knjige). Moodle šalje njihov identitet FastCommentsu putem LTI pokretanja:

- Nema zaslona za prijavu. FastComments ih prijavljuje koristeći Moodle račun.
- Njihovo prikazno ime, e-pošta i avatar dolaze iz Moodlea.
- Nit je ograničena na `(Moodle site, course, resource link ID)`, pa ista aktivnost duplicirana u drugom kolegiju dobiva novu nit.
- Događaji odgovaranja u nitima, glasanje i obavijesti rade isto kao u samostalnoj FastComments niti.

#### Ograničite javni pristup (Preporučeno)

Po zadanim postavkama, podaci komentara FastComments-a su javno čitljivi. Bilo tko tko pogodi URL niti ili API endpoint može vidjeti komentare, čak i izvan Moodlea. Za rasprave u kolegiju gotovo sigurno želite ograničiti pregled samo na upisane studente.

Otvorite svoju <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">stranicu za prilagodbu widgeta</a> i kreirajte pravilo s omogućenom opcijom **Require SSO To View Comments**, zatim postavite razinu sigurnosti na **Secure SSO** tako da se niti mogu učitavati samo kroz potpisano LTI pokretanje.

Pogledajte [Zaštita niti komentara pomoću jednokratne prijave (SSO)](/guide-customizations-and-configuration.html#sso-require-to-view-comments) za kompletan vodič, uključujući kako ograničiti pravilo na jednu domenu ili stranicu.

#### Moodle poteškoće

**FastComments nedostaje u izborniku aktivnosti.** Administrator stranice je registrirao alat ali nije postavio **Tool configuration usage** na **Show in activity chooser and as a preconfigured tool**. Ispravite to pod **Administracija stranice** > **Dodaci** > **Moduli aktivnosti** > **Vanjski alat** > **Upravljanje alatima** > ikona zupčanika na pločici FastComments.

**Pokretanje ne uspijeva ili prikazuje prazan okvir kada je postavljeno na "Isti prozor".** Session kolačići Moodlea koriste `SameSite=Lax` prema zadanim postavkama, i neki preglednici ih uklanjaju pri cross-site POST zahtjevu koji LTI 1.3 koristi za povratak iz FastComments-a. Postavite **Način pokretanja** na **Novi prozor** za aktivnost. Ovo je strogi zahtjev za ugrađeni FastComments unutar Stranice ili Knjige, jer put pokretanja ugrađenog u editor uvijek otvara novi prozor.

**`iss` tvrdnja je URL Moodle stranice, a ne ID zakupnika.** FastComments koristi URL Moodle stranice (vrijednost konfiguracije `wwwroot`) kao LTI issuer. Ako se vaša Moodle instanca premjesti na novu domenu ili promijenite `wwwroot`, postojeće FastComments niti ostaju vezane za stari issuer i neće se poklapati s novim pokretanjima. Ponovno registrirajte alat protiv novog URL-a i po potrebi migrirajte niti putem FastComments administracije.

**Backup i vraćanje aktivnosti.** Izrada sigurnosne kopije kolegija i vraćanje u novi kolegij stvara nove resource link ID-je, pa obnovljene FastComments aktivnosti započinju s praznim nitima. Izvorni kolegij zadržava izvornu nit. To je namjerno ponašanje, a ne greška.

**Moodle 4.5 TinyMCE zadano.** Moodle 4.5 dolazi s TinyMCE-om kao zadanim editorom za nove instalacije. Gumb Vanjski alat nalazi se pod izbornikom **Više** (`...`), a ne na glavnoj alatnoj traci. Starije stranice koje su nadograđene s 4.1 zadržavaju Atto osim ako administrator nije promijenio zadano.