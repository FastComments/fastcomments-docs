Ovaj vodič objašnjava dodavanje FastComments u Moodle 4.x kurs nakon što je administrator sajta registrovao alat i postavio da se prikazuje u izboru aktivnosti. Ako FastComments još nije registrovan, prvo pogledajte vodič za registraciju u Moodleu.

#### Otvorite kurs u režimu uređivanja

1. Prijavite se u Moodle kao Nastavnik sa pravom uređivanja (ili viši) za taj kurs.
2. Otvorite kurs.
3. Uključite **Režim uređivanja** koristeći prekidač u gornjem desnom uglu zaglavlja kursa.

Moodle 4.x je zamijenio stari padajući izbornik "Add an activity or resource" koji je koristio 3.x sa fullscreen dijalogom za izbor aktivnosti. Moodle 4.5 zadržava isti izbor, ali dodaje red sa označenim/favoritima na vrhu, pa će zakačivanje FastComments jednom olakšati pristup u kasnijim odjeljcima.

#### Dodavanje FastComments aktivnosti

1. Skrolajte do odjeljka kursa (teme ili sedmice) gdje pripada diskusija.
2. Kliknite **Add an activity or resource** na dnu tog odjeljka.
3. U dijalogu za izbor, izaberite **FastComments**. Ako ga ne vidite, preskočite do odjeljka o problemima ispod.

Otvori se obrazac za postavke aktivnosti. Polja koja su važna:

- **Naziv aktivnosti** (obavezno). Prikazuje se na stranici kursa i u knjizi ocjena. Primjer: `Week 3 Discussion`.
- **Opis aktivnosti**. Opcioni uvodni tekst koji se prikazuje iznad niti komentara.
- **Prikaži opis na stranici kursa**. Označite ovo ako želite da opis bude vidljiv bez klika na aktivnost.
- **Preconfigured tool**. Postavljeno na `FastComments` (automatski odabrano kada se pokrene iz izbornika). Ne mijenjajte.
- **Launch container**. Postavite na **Novi prozor**. Pogledajte odjeljak o problemima zašto "Isti prozor" puca u nekim Moodle instalacijama.
- **Tool URL**, **Public key**, **Shared secret**, **Custom parameters**. Ostavite prazno. Dinamička registracija je ovo riješila na nivou sajta.

Skrolajte do dna i kliknite **Save and return to course** (ili **Save and display** da odmah otvorite aktivnost).

Aktivnost se pojavljuje kao red u odjeljku sa FastComments ikonom. Studenti kliknu na red da otvore nit komentara.

#### Umetanje FastComments inline u editor

Za nit unutar Page, Book poglavlja, Lesson, ili bilo kojeg drugog resursa koji koristi Atto ili TinyMCE editor:

1. Otvorite resurs u režimu uređivanja.
2. Postavite kursor na mjesto gdje nit treba da se pojavi.
3. Na traci alata editora kliknite dugme **LTI** / **External tool**. U Atto je označeno "Insert LTI Advantage content". U TinyMCE (zadano u Moodle 4.3+) nalazi se pod menijem **More** kao **External tools**.
4. Iz liste alata izaberite **FastComments**.
5. FastComments otvara picker za deep-linking. Potvrdite naslov niti i kliknite **Embed**.
6. Editor umeće placeholder LTI bloka. Sačuvajte resurs.

Svaka ugrađena instanca je posebna nit vezana za deep-link content item ID, pa Page sa tri FastComments ugradnje dobija tri nezavisne niti.

#### Ograničavanje pristupa i podešavanja grupa

Standardne Moodle postavke aktivnosti primjenjuju se na FastComments aktivnosti:

- **Common module settings** > **Group mode**. Postavljanje na **Separate groups** ili **Visible groups** samo po sebi neće razdvojiti FastComments u niti po grupama. Moodle-ov način rada s grupama samo filtrira knjigu ocjena i listu članova. Da biste imali zasebnu nit po grupi, dodajte po jednu FastComments aktivnost za svaku grupu i koristite **Restrict access** da ograničite svaku od njih.
- **Restrict access** > **Add restriction**. Podržava standardne Moodle uslove: **Date**, **Grade**, **Group**, **Grouping**, **User profile**, i ugniježdene skupove ograničenja. Koristite **Group** da zaključate FastComments aktivnost za jednu grupu.
- **Activity completion**. Postavite na **Students must view this activity to complete it** ako želite praćenje završetka aktivnosti. FastComments trenutno ne šalje događaj završetka natrag u Moodle izvan samog pokretanja.

#### Mapiranje uloga

FastComments čita LTI `roles` claim koji Moodle šalje pri svakom pokretanju i mapira ga na sljedeći način:

- Moodle **Manager** ili **Site administrator** -> FastComments **admin**
- Moodle **Editing teacher** ili **Non-editing teacher** -> FastComments **moderator**
- Moodle **Student** -> FastComments **commenter**
- Moodle **Guest** -> samo za čitanje

Administratori mogu brisati bilo koji komentar, zabranjivati korisnike i uređivati postavke niti. Moderatori mogu brisati i odobravati komentare unutar niti u koju su ušli. Prilagođene Moodle uloge nasljeđuju mapiranje arhetipa od kojeg su klonirane.

#### Šta studenti vide

Studenti kliknu FastComments aktivnost (ili skrolaju do ugrađenog bloka unutar Page ili Book). Moodle šalje njihov identitet FastComments putem LTI pokretanja:

- Nema ekrana za prijavu. FastComments ih prijavljuje koristeći Moodle nalog.
- Njihovo prikazano ime, email i avatar dolaze iz Moodla.
- Nit je ograničena na (Moodle sajt, kurs, resource link ID), pa ista aktivnost duplicirana u drugom kursu dobija novu nit.
- Ugniježđeni odgovori, glasanje i notifikacije rade isto kao i u samostalnoj FastComments niti.

#### Moodle problemi (gotchas)

**FastComments nedostaje u izboru aktivnosti.** Administrator sajta je registrovao alat ali nije podesio **Tool configuration usage** na **Show in activity chooser and as a preconfigured tool**. Ispravite ovo pod **Site administration** > **Plugins** > **Activity modules** > **External tool** > **Manage tools** > ikona zupčanika na FastComments pločici.

**Pokretanje ne uspijeva ili prikazuje prazan okvir kada je postavljeno na "Isti prozor".** Moodle-ove session kolačiće po defaultu koriste `SameSite=Lax`, i neki browseri ih uklanjaju pri cross-site POST-u koji LTI 1.3 koristi za povratak iz FastComments. Postavite **Launch container** na **Novi prozor** za aktivnost. Ovo je strogo zahtijevanje za ugrađeni FastComments unutar Page ili Book, budući da put pokretanja iz editora uvijek otvara novi prozor.

**`iss` claim je URL Moodle sajta, a ne tenant ID.** FastComments koristi Moodle URL sajta (vrijednost konfiguracije `wwwroot`) kao LTI issuer. Ako se vaša Moodle instanca preseli na novi domen ili promijenite `wwwroot`, postojeće FastComments niti ostaju vezane za stari issuer i neće se poklapati sa novim pokretanjima. Ponovo registrujte alat za novi URL i, ako je potrebno, migrirajte niti kroz FastComments administraciju.

**Backup i restore aktivnosti.** Pravljenje backup-a kursa i njegovo vraćanje u novi kurs stvara nove resource link ID-ove, tako da vraćene FastComments aktivnosti počinju sa praznim nitima. Originalni kurs zadržava originalne niti. Ovo je namjeravano ponašanje, a ne greška.

**Moodle 4.5 TinyMCE po defaultu.** Moodle 4.5 dolazi sa TinyMCE kao zadanim editorom za nove instalacije. Dugme External tool se nalazi pod menijem **More** (`...`) umjesto na glavnoj traci alata. Stariji sajtovi koji su nadograđeni sa 4.1 zadržavaju Atto osim ako administrator nije promijenio zadano.