Ovaj vodič objašnjava kako dodati FastComments u Moodle 4.x tečaj nakon što je administrator sajta registrirao alat i postavio da se prikazuje u odabiru aktivnosti. Ako FastComments još nije registriran, prvo pogledajte vodič za registraciju u Moodlu.

#### Otvorite tečaj u načinu uređivanja

1. Prijavite se u Moodle kao nastavnik s mogućnošću uređivanja (ili viši) za tečaj.
2. Otvorite tečaj.
3. Uključite **Način uređivanja** koristeći prekidač u gornjem desnom kutu zaglavlja tečaja.

Moodle 4.x zamijenio je stari padajući izbornik "Dodaj aktivnost ili resurs" koji je koristio 3.x dijaloškim prozorom za odabir aktivnosti preko cijelog ekrana. Moodle 4.5 zadržava isti odabir, ali dodaje red s oznakama/favoritima na vrhu, pa je pričvršćivanje FastComments-a jednom korisno za brži pristup u kasnijim sekcijama.

#### Dodajte FastComments aktivnost

1. Pomaknite se do odjeljka tečaja (teme ili tjedna) u kojem rasprava pripada.
2. Kliknite **Dodaj aktivnost ili resurs** na dnu tog odjeljka.
3. U dijalogu za odabir odaberite **FastComments**. Ako ga ne vidite, prijeđite na odjeljak o problemima dolje.

Otvara se obrazac za postavke aktivnosti. Polja koja su važna:

- **Activity name** (obavezno). Prikazuje se na stranici tečaja i u ocjenjivaču. Primjer: `Week 3 Discussion`.
- **Activity description**. Neobavezni uvodni tekst koji se prikazuje iznad nit.
- **Show description on course page**. Označite ako želite da opis bude vidljiv bez ulaska u aktivnost.
- **Preconfigured tool**. Postavljeno na `FastComments` (automatski odabrano kada se pokrene iz odabira). Ne mijenjajte.
- **Launch container**. Postavite na **Novi prozor**. Pogledajte odjeljak o problemima zašto "Isti prozor" u nekim Moodle instalacijama ne radi.
- **Tool URL**, **Public key**, **Shared secret**, **Custom parameters**. Ostavite prazno. Dinamička registracija je to obradila na razini sajta.

Pomaknite se do dna i kliknite **Save and return to course** (ili **Save and display** da odmah otvorite aktivnost).

Aktivnost se pojavljuje kao red u odjeljku s ikonom FastComments. Studenti kliknu red da otvore nit komentara.

#### Ugradite FastComments inline s uređivačem

Za nit unutar Stranice (Page), poglavlja Knjige (Book), Lekcije ili bilo kojeg drugog resursa koji koristi Atto ili TinyMCE uređivač:

1. Otvorite resurs u načinu uređivanja.
2. Postavite kursor na mjesto gdje treba biti nit.
3. U alatnoj traci uređivača kliknite gumb **LTI** / **Vanjski alat**. U Atto uređivaču označeno je kao "Insert LTI Advantage content". U TinyMCE-u (zadano u Moodle 4.3+) nalazi se u izborniku **Više** kao **External tools**.
4. Odaberite **FastComments** s popisa alata.
5. FastComments otvara dijalog za dubinsko povezivanje. Potvrdite naslov niti i kliknite **Embed**.
6. Uređivač umeće LTI rezervni blok. Spremite resurs.

Svaka ugrađena instanca je zasebna nit ključana na ID stavke dubinske veze, pa Stranica s tri FastComments ugrađivanja dobije tri neovisne niti.

#### Ograniči pristup i postavke grupa

Standardne postavke aktivnosti u Moodlu primjenjuju se na FastComments aktivnosti:

- **Common module settings** > **Group mode**. Postavljanje na **Separate groups** ili **Visible groups** samo po sebi ne dijeli FastComments na niti po grupama. Način rada grupa u Moodlu samo filtrira ocjenjivač i popis članova. Da biste pokrenuli zasebnu nit po grupi, dodajte jednu FastComments aktivnost po grupi i koristite **Restrict access** da biste ograničili svaku.
- **Restrict access** > **Add restriction**. Podržava standardne Moodle uvjete: **Date**, **Grade**, **Group**, **Grouping**, **User profile**, i ugniježdene skupove ograničenja. Koristite **Group** za zaključavanje FastComments aktivnosti za jednu grupu.
- **Activity completion**. Postavite na **Students must view this activity to complete it** ako želite praćenje dovršenja. FastComments trenutno ne prijavljuje događaj dovršenja natrag u Moodle izvan pokretanja.

#### Preslikavanje uloga

FastComments čita LTI `roles` tvrdnju koju Moodle šalje pri svakom pokretanju i preslikava je kako slijedi:

- Moodle **Manager** ili **Site administrator** -> FastComments **admin**
- Moodle **Editing teacher** ili **Non-editing teacher** -> FastComments **moderator**
- Moodle **Student** -> FastComments **commenter**
- Moodle **Guest** -> samo za čitanje

Administratori mogu izbrisati bilo koji komentar, zabraniti korisnike i uređivati postavke niti. Moderatori mogu brisati i odobravati komentare unutar niti koju su pokrenuli. Prilagođene Moodle uloge nasljeđuju preslikavanje arhetipa od kojeg su klonirane.

#### Što studenti vide

Studenti kliknu FastComments aktivnost (ili se pomaknu do ugrađenog bloka unutar Stranice ili Knjige). Moodle šalje njihov identitet u FastComments putem LTI pokretanja:

- Nema zaslona za prijavu. FastComments ih prijavljuje koristeći Moodle račun.
- Njihovo prikazano ime, e-pošta i avatar dolaze iz Moodla.
- Nit je ograničena na `(Moodle site, course, resource link ID)`, pa ista aktivnost duplicirana u drugom tečaju dobije novu nit.
- Ugniježđeni odgovori, glasovanje i obavijesti rade isto kao u samostalnoj FastComments niti.

#### Moodle - poznati problemi

**FastComments nedostaje u odabiru aktivnosti.** Administrator sajta je registrirao alat, ali nije postavio **Tool configuration usage** na **Show in activity chooser and as a preconfigured tool**. Popravite to pod **Administracija sajta** > **Dodaci** > **Moduli aktivnosti** > **Vanjski alat** > **Manage tools** > ikona zupčanika na FastComments pločici.

**Pokretanje ne uspijeva ili prikazuje prazni okvir kada je postavljeno na "Isti prozor".** Moodle sejske kolačiće koristi `SameSite=Lax` prema zadanim postavkama, a neki preglednici ih uklanjaju na cross-site POST koji LTI 1.3 koristi za povratak iz FastComments. Postavite **Launch container** na **Novi prozor** u aktivnosti. Ovo je strogi zahtjev za ugrađeni FastComments unutar Stranice ili Knjige, budući da put pokretanja ugrađen u uređivač uvijek otvara novi prozor.

**`iss` tvrdnja je URL Moodle sajta, a ne tenant ID.** FastComments koristi URL Moodle sajta (vrijednost konfiguracije `wwwroot`) kao LTI izdavatelja. Ako se vaša Moodle instanca premjesti na novu domenu ili promijenite `wwwroot`, postojeće FastComments niti ostaju vezane za starog izdavatelja i neće se podudarati s novim pokretanjima. Ponovo registrirajte alat za novu URL adresu i po potrebi migrirajte niti putem FastComments administratorskog sučelja.

**Backup i vraćanje aktivnosti.** Izrada sigurnosne kopije tečaja i vraćanje u novi tečaj stvara nove ID-jeve resource linkova, pa obnovljene FastComments aktivnosti počinju s praznim nitima. Izvorni tečaj zadržava izvorne niti. To je namjeravano ponašanje, a ne greška.

**TinyMCE zadano u Moodle 4.5.** Moodle 4.5 isporučuje TinyMCE kao zadani uređivač za nove instalacije. Gumb Vanjskog alata nalazi se u izborniku **Više** (`...`) umjesto u glavnoj alatnoj traci. Stariji sajtovi koji su nadograđeni s 4.1 zadržavaju Atto osim ako administrator nije promijenio zadano.