Ovaj vodič pokriva dodavanje FastComments-a u Moodle 4.x kurs nakon što je administrator sajta registrovao alat i podesio da se prikazuje u izboru aktivnosti. Ako FastComments još nije registrovan, prvo pogledajte vodič za registraciju Moodle-a.

#### Otvorite kurs u režimu uređivanja

1. Prijavite se u Moodle kao Uređivački nastavnik (ili viši) za dati kurs.
2. Otvorite kurs.
3. Uključite **Režim uređivanja** koristeći prekidač u gornjem desnom uglu zaglavlja kursa.

Moodle 4.x je zamenio nasleđeni padajući meni "Add an activity or resource" koji je koristio 3.x sa dijalogom biranja aktivnosti preko celog ekrana. Moodle 4.5 zadržava isti izbornik ali dodaje red sa zvezdicama/omiljenim na vrhu, tako da pinovanjem FastComments-a jednom olakšavate njegovo pronalaženje u kasnijim sekcijama.

#### Dodavanje FastComments aktivnosti

1. Skrolujte do sekcije kursa (teme ili nedelje) gde pripada diskusija.
2. Kliknite **Add an activity or resource** na dnu te sekcije.
3. U dijalogu izbora, izaberite **FastComments**. Ako ga ne vidite, pređite na odeljak o problemima ispod.

Otvara se forma za podešavanje aktivnosti. Polja koja su bitna:

- **Naziv aktivnosti** (obavezno). Prikazuje se na stranici kursa i u dnevniku ocena. Primer: `Week 3 Discussion`.
- **Opis aktivnosti**. Opcioni uvodni tekst koji se prikazuje iznad niti komentara.
- **Prikaži opis na stranici kursa**. Obeležite ako želite da opis bude vidljiv bez klika na aktivnost.
- **Preconfigured tool**. Podesite na `FastComments` (automatski je izabrano kada se pokrene iz izborničkog dijaloga). Ne menjajte.
- **Launch container**. Podesite na **Novi prozor**. Pogledajte odeljak o problemima zašto "Isti prozor" u nekim Moodle instalacijama ne radi.
- **Tool URL**, **Public key**, **Shared secret**, **Custom parameters**. Ostavite prazno. Dinamička registracija je ovo obradila na nivou sajta.

Skrolujte do dna i kliknite **Sačuvaj i vrati se na kurs** (ili **Sačuvaj i prikaži** da odmah otvorite aktivnost).

Aktivnost se pojavljuje kao red u sekciji sa ikonom FastComments. Studenti kliknu na red da otvore nit komentara.

#### Ugradite FastComments direktno u editor

Za nit unutar Page, Book poglavlja, Lesson-a ili bilo kog drugog resursa koji koristi Atto ili TinyMCE editor:

1. Otvorite resurs u režimu uređivanja.
2. Postavite kursor na mesto gde nit treba da se pojavi.
3. U traci alata editora, kliknite na dugme **LTI** / **External tool**. U Atto-u je označeno kao "Insert LTI Advantage content". U TinyMCE (podrazumevano u Moodle 4.3+) nalazi se u meniju **More** kao **External tools**.
4. Izaberite **FastComments** sa liste alata.
5. FastComments otvara picker za deep-linking. Potvrdite naslov niti i kliknite **Umetni**.
6. Editor umeće LTI zamenski blok (placeholder). Sačuvajte resurs.

Svaka ugrađena instanca je zasebna nit koja je vezana za deep-link content item ID, tako da Page sa tri FastComments ugradnje dobija tri nezavisne niti.

#### Ograničite pristup i podešavanja grupa

Standardna Moodle podešavanja aktivnosti primenjuju se na FastComments aktivnosti:

- **Common module settings** > **Group mode**. Podesavanje na **Separate groups** ili **Visible groups** samo po sebi ne deli FastComments na niti po grupama. Režim grupa u Moodle-u samo filtrira dnevnik ocena i listu članova. Da biste pokrenuli odvojenu nit po grupi, dodajte po jednu FastComments aktivnost za svaku grupu i koristite **Restrict access** da ograničite obuhvat svake.
- **Restrict access** > **Add restriction**. Podržava standardne Moodle uslove: **Date**, **Grade**, **Group**, **Grouping**, **User profile**, i ugnježdene skupove ograničenja. Koristite **Group** da zaključate FastComments aktivnost za jednu grupu.
- **Activity completion**. Podesite na **Students must view this activity to complete it** ako želite praćenje završetka aktivnosti. FastComments trenutno ne izveštava Moodle o događaju završetka dalje od samog pokretanja.

#### Mapiranje uloga

FastComments čita LTI `roles` claim koji Moodle šalje pri svakom pokretanju i mapira ga na sledeći način:

- Moodle **Menadžer** ili **Administrator sajta** -> FastComments **admin**
- Moodle **Uređivački nastavnik** ili **Neuređivački nastavnik** -> FastComments **moderator**
- Moodle **Student** -> FastComments **commenter**
- Moodle **Gost** -> samo za čitanje

Administratori mogu obrisati bilo koji komentar, banovati korisnike i uređivati podešavanja niti. Moderatori mogu brisati i odobravati komentare unutar niti u koju su pokrenuti. Prilagođene Moodle uloge nasleđuju mapiranje od arhetipa od kojeg su klonirane.

#### Šta studenti vide

Studenti kliknu FastComments aktivnost (ili se skroluju do ugrađenog bloka unutar Page-a ili Book-a). Moodle šalje njihov identitet FastComments-u preko LTI pokretanja:

- Nema ekrana za prijavu. FastComments ih prijavljuje koristeći Moodle nalog.
- Njihovo prikazno ime, email i avatar dolaze iz Moodle-a.
- Nit je ograničena na `(Moodle site, course, resource link ID)`, tako da ista aktivnost duplicirana u drugom kursu dobija novu nit.
- Ugnježdeni odgovori, glasanje i obaveštenja rade isto kao u samostalnoj FastComments niti.

#### Moodle - česte zamke

**FastComments nedostaje u izboru aktivnosti.** Administrator sajta je registrovao alat ali nije podesio **Tool configuration usage** na **Show in activity chooser and as a preconfigured tool**. Ispravite ovo pod **Site administration** > **Plugins** > **Activity modules** > **External tool** > **Manage tools** > ikonica zupčanika na FastComments pločici.

**Pokretanje ne uspeva ili prikazuje prazan okvir kada je postavljeno na "Isti prozor".** Moodle kolačići sesije koriste `SameSite=Lax` po defaultu, i neki browser-i ih uklanjaju pri cross-site POST zahtevu koji LTI 1.3 koristi za povratak sa FastComments-a. Podesite **Launch container** na **Novi prozor** za tu aktivnost. Ovo je strogo zahtevano za ugrađeni FastComments unutar Page-a ili Book-a, pošto putanja pokretanja iz editora uvek otvara novi prozor.

**The `iss` claim je URL Moodle sajta, a ne tenant ID.** FastComments koristi URL Moodle sajta (vrednost konfiguracije `wwwroot`) kao LTI issuer. Ako se vaš Moodle premesti na novi domen ili promenite `wwwroot`, postojeće FastComments niti ostaju vezane za starog issuer-a i neće se poklapati sa novim pokretanjima. Ponovo registrujte alat za novi URL i po potrebi migrirajte niti kroz FastComments administraciju.

**Backup i vraćanje aktivnosti.** Bekapovanje kursa i njegovo vraćanje u novi kurs kreira nove resource link ID-jeve, tako da vraćene FastComments aktivnosti počinju sa praznim nitima. Originalni kurs zadržava originalne niti. Ovo je namerno ponašanje, a ne greška.

**Moodle 4.5 TinyMCE podrazumevano.** Moodle 4.5 se isporučuje sa TinyMCE kao podrazumevanim editorom za nove instalacije. Dugme External tool se nalazi u meniju **More** (`...`) umesto na glavnoj traci alata. Stariji sajtovi koji su nadograđeni sa 4.1 zadržavaju Atto osim ako administrator nije promenio podrazumevani editor.

---