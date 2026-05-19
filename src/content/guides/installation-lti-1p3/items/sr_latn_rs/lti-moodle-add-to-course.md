Ovaj vodič objašnjava dodavanje FastComments u Moodle 4.x kurs nakon što je administrator sajta registrovao alat i postavio da se prikazuje u izboru aktivnosti. Ako FastComments još nije registrovan, prvo pogledajte vodič za registraciju Moodlea.

#### Open the Course in Edit Mode

1. Prijavite se u Moodle kao Editing Teacher (ili viši) za taj kurs.
2. Otvorite kurs.
3. Uključite **Edit mode** koristeći prekidač u gornjem desnom uglu zaglavlja kursa.

Moodle 4.x je zamenio stari padajući meni "Add an activity or resource" koji je koristio 3.x punoprekrivačkim dijalogom za izbor aktivnosti. Moodle 4.5 zadržava isti izbor, ali dodaje red sa zvezdicama/omiljenim na vrhu, tako da zakačivanje FastComments-a jednom olakšava pristup u kasnijim odeljcima.

#### Add the FastComments Activity

1. Skrolujte do odeljka kursa (tema ili nedelja) gde diskusija pripada.
2. Kliknite **Add an activity or resource** na dnu tog odeljka.
3. U dijalogu za izbor izaberite **FastComments**. Ako ga ne vidite, pređite na odeljak sa problemima ispod.

Otvara se forma za podešavanja aktivnosti. Polja koja su bitna:

- **Activity name** (required). Prikazuje se na stranici kursa i u gradebook-u. Primer: `Week 3 Discussion`.
- **Activity description**. Opcioni uvodni tekst koji se prikazuje iznad niti komentara.
- **Show description on course page**. Označite ako želite da opis bude vidljiv bez otvaranja aktivnosti.
- **Preconfigured tool**. Podesite na `FastComments` (automatski izabrano kada se pokreće iz izbornika). Nemojte menjati.
- **Launch container**. Podesite na **New window**. Pogledajte odeljak sa problemima da biste razumeli zašto "Same window" ponekad pravi problem u nekim Moodle instalacijama.
- **Tool URL**, **Public key**, **Shared secret**, **Custom parameters**. Ostavite prazno. Dynamic Registration je ovo rešio na nivou sajta.

Skrolujte na dno i kliknite **Save and return to course** (ili **Save and display** da odmah otvorite aktivnost).

Aktivnost se pojavljuje kao red u odeljku sa FastComments ikonom. Studenti kliknu red da otvore nit komentara.

#### Embed FastComments Inline with the Editor

Za nit unutar Page, Book poglavlja, Lesson-a ili bilo kog drugog resursa koji koristi Atto ili TinyMCE editor:

1. Otvorite resurs u režimu uređivanja.
2. Postavite kursor na mesto gde treba da se pojavi nit.
3. U traci alata editora kliknite dugme **LTI** / **External tool**. U Atto-u je označeno kao "Insert LTI Advantage content". U TinyMCE (podrazumevano u Moodle 4.3+) nalazi se u meniju **More** kao **External tools**.
4. Izaberite **FastComments** iz liste alata.
5. FastComments otvara picker za deep-linking. Potvrdite naslov niti i kliknite **Embed**.
6. Editor ubacuje LTI placeholder blok. Sačuvajte resurs.

Svaka ugrađena instanca predstavlja posebnu nit koja je identifikovana deep-link content item ID-jem, tako da Page sa tri FastComments ugrađivanja ima tri nezavisne niti.

#### Restrict Access and Group Settings

Standardna Moodle podešavanja aktivnosti važe i za FastComments aktivnosti:

- **Common module settings** > **Group mode**. Podesite li ovo na **Separate groups** ili **Visible groups** ne deli automatski FastComments na niti po grupama. Moodle-ov group mode samo filtrira gradebook i listu članova. Da biste imali posebnu nit po grupi, dodajte po jednu FastComments aktivnost za svaku grupu i koristite **Restrict access** da ograničite svaku od njih.
- **Restrict access** > **Add restriction**. Podržava standardne Moodle uslove: **Date**, **Grade**, **Group**, **Grouping**, **User profile**, i ugnježdene skupove ograničenja. Koristite **Group** da zaključate FastComments aktivnost za jednu grupu.
- **Activity completion**. Podesite na **Students must view this activity to complete it** ako želite praćenje izvršenja aktivnosti. FastComments trenutno ne šalje događaj o izvršenju nazad u Moodle osim prilikom launch-a.

#### Role Mapping

FastComments čita LTI `roles` claim koji Moodle šalje pri svakom launch-u i mapira ga na sledeći način:

- Moodle **Manager** ili **Site administrator** -> FastComments **admin**
- Moodle **Editing teacher** ili **Non-editing teacher** -> FastComments **moderator**
- Moodle **Student** -> FastComments **commenter**
- Moodle **Guest** -> samo za čitanje

Administratori mogu da brišu bilo koji komentar, zabrane korisnike i uređuju podešavanja niti. Moderatori mogu da brišu i odobravaju komentare unutar niti u koju su pokrenuti. Prilagođene Moodle uloge nasleđuju mapiranje arhetipa od kojeg su klonirane.

#### What Students See

Studenti kliknu FastComments aktivnost (ili skroluju do ugrađenog bloka unutar Page ili Book). Moodle im šalje identitet FastComments putem LTI launch-a:

- Nema ekrana za prijavu. FastComments ih prijavljuje koristeći Moodle nalog.
- Njihovo prikazno ime, email i avatar dolaze iz Moodlea.
- Nit je ograničena na `(Moodle site, course, resource link ID)`, tako da ista aktivnost duplicirana u drugom kursu dobija novu nit.
- Threaded odgovori, glasanje i notifikacije rade isto kao i u samostalnoj FastComments niti.

#### Lock Down Public Access (Recommended)

Po defaultu, FastComments podaci o komentarima su javno čitljivi. Svako ko pogodi URL niti ili API endpoint može videti komentare, čak i van Moodlea. Za diskusije u kursu gotovo sigurno želite da ograničite pregled samo na upisane studente.

Otvorite svoju <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">widget customization page</a> i kreirajte pravilo sa omogućenom opcijom **Require SSO To View Comments**, zatim postavite nivo bezbednosti na **Secure SSO** tako da niti mogu da se učitavaju samo preko potpisanog LTI launch-a.

Pogledajte [Protecting Comment Threads With Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) za kompletan vodič, uključujući kako da ograničite pravilo na jednu domenu ili stranicu.

#### Moodle Gotchas

**FastComments missing from the activity chooser.** Administrator sajta je registrovao alat, ali nije postavio **Tool configuration usage** na **Show in activity chooser and as a preconfigured tool**. Ispravite to pod **Site administration** > **Plugins** > **Activity modules** > **External tool** > **Manage tools** > ikonica zupčanika na FastComments pločici.

**Launch fails or shows a blank frame when set to "Same window".** Moodle sesione kolačiće podrazumevano postavlja sa `SameSite=Lax`, i neki pregledači ih uklanjaju pri cross-site POST zahtevu koji LTI 1.3 koristi za povratak iz FastComments-a. Podesite **Launch container** na **New window** za aktivnost. Ovo je striktan zahtev za ugrađeni FastComments unutar Page ili Book, jer putanja pokretanja iz editora uvek otvara novi prozor.

**The `iss` claim is the Moodle site URL, not a tenant ID.** FastComments koristi Moodle site URL (vrednost `wwwroot` u konfiguraciji) kao LTI issuer. Ako vaš Moodle premesti na novi domen ili promenite `wwwroot`, postojeće FastComments niti ostaju vezane za stari issuer i neće se poklapati sa novim launch-evima. Ponovo registrujte alat za novi URL i po potrebi migrirajte niti kroz FastComments administraciju.

**Activity backup and restore.** Pravljenje backup-a kursa i njegovo vraćanje u novi kurs kreira nove resource link ID-je, tako da obnovljene FastComments aktivnosti počinju sa praznim nitima. Originalni kurs zadržava originalne niti. Ovo je očekovano ponašanje, a ne greška.

**Moodle 4.5 TinyMCE default.** Moodle 4.5 dolazi sa TinyMCE kao podrazumevanim editorom za nove instalacije. Dugme External tool se nalazi u meniju **More** (`...`) umesto u glavnoj traci. Stariji sajtovi koji su nadograđeni sa 4.1 zadržavaju Atto osim ako administrator nije promenio podrazumevano.