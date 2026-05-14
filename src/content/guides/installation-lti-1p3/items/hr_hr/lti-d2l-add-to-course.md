Ova stranica objašnjava dodavanje FastComments u Brightspace kolegij nakon što je administrator registrirao alat i stvorio implementaciju. Ako alat još nije registriran, prvo pogledajte D2L vodič za registraciju.

<div class="screenshot white-bg">
    <div class="title">FastComments ugrađen kao tema jedinice u Brightspace</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-d2l-comments-in-unit.png" alt="FastComments pokrenut unutar Brightspace jedinice, prikazuje ugniježdene komentare i birač @-spomena" />
</div>

Brightspace isporučuje dva iskustva za izradu sadržaja: **Classic Content** i **New Content Experience** (također nazvano **Lessons**). Oba omogućuju FastComments, ali se putanje u izborniku razlikuju. Svaki odjeljak u nastavku pokriva oba gdje se razilaze.

#### Pronađite alat FastComments

Alat FastComments pojavljuje se na dva mjesta unutar uređivača sadržaja kolegija:

1. U odabiraču aktivnosti, do kojeg se dolazi putem gumba **Add Existing** modula/jednice (u starijim verzijama Brightspace označeno **Add Existing Activities**). U trenutnim izdanjima Brightspace FastComments se prikazuje izravno u odabiraču; starije verzije ga umeću pod podizbornik **External Learning Tools**. Bilo koja putanja dodaje FastComments kao samostalnu temu.
2. Dijalog **Insert Stuff** unutar HTML uređivača, pod **LTI Advantage**. Ovo ugrađuje FastComments inline u HTML temu putem LTI deep linking toka.

Ako se FastComments ne pojavljuje ni u jednom od odabirača, implementacija nije omogućena za organizacijsku jedinicu (org unit) koja drži kolegij. Zamolite svog Brightspace administratora da otvori **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments tool > **View Deployments**, otvori implementaciju i doda organizacijsku jedinicu kolegija (ili roditeljsku org jedinicu) pod **Org Units**.

#### Dodavanje FastComments kao teme u modulu

Classic Content:

1. Otvorite kolegij i kliknite **Content** u navigacijskoj traci.
2. Odaberite modul koji bi trebao sadržavati raspravu (ili ga stvorite putem **Add a module**).
3. Kliknite **Add Existing** (stariji Brightspace: **Add Existing Activities** > **External Learning Tools**).
4. U odabiraču kliknite **FastComments**. Brightspace stvara temu u modulu i vraća vas na prikaz sadržaja.
5. Kliknite novu temu. Preimenujte je u nešto opisno poput `FastComments Discussion` koristeći uređivač naslova unutar retka.

New Content Experience (Lessons):

1. Otvorite kolegij i kliknite **Content**.
2. Otvorite jedinicu i lekciju koja bi trebala sadržavati raspravu.
3. Kliknite **Add** > **Existing Activity** i odaberite **FastComments** (stariji Brightspace: ugniježđeno pod **External Learning Tools**).
4. Aktivnost se dodaje u lekciju.
5. Kliknite naslov aktivnosti da biste ga preimenovali.

Prvi put kada bilo koji korisnik (predavač ili student) otvori temu, FastComments inicijalizira nit za taj resource link. Nit je vezana uz ID resource linka, pa promjena naziva ili premještanje teme ne mijenja koju nit će se učitati.

#### Ugradite FastComments inline u HTML temu

Koristite ovaj tok kada želite da se komentari prikazuju ispod čitanja, videa ili drugog sadržaja unutar iste stranice teme, umjesto kao zasebna tema.

1. Otvorite ili stvorite HTML temu u modulu/lekciji.
2. Kliknite **Edit HTML** da otvorite Brightspace HTML uređivač.
3. Postavite kursor na mjesto gdje bi se trebala pojaviti nit komentara.
4. Kliknite gumb **Insert Stuff** (ikona slagalice u alatnoj traci uređivača).
5. U dijalogu Insert Stuff skrolajte do **LTI Advantage** i kliknite **FastComments**.
6. FastComments otvara deep linking odabirač. Potvrdite mjesto (zadane opcije rade za rasprave o sadržaju); kliknite **Insert** ili **Continue**.
7. Brightspace se vraća u HTML uređivač s rezervnim blokom koji predstavlja LTI lansiranje. Kliknite **Save and Close** na temi.

Kad se tema učita, Brightspace zamjenjuje rezervni blok iframeom koji automatski pokreće FastComments putem LTI-ja. Studenti vide nit rasprave inline.

Jedna HTML tema može sadržavati više deep-linked FastComments ugradnji. Svaka ugradnja dobiva svoju nit jer svaki deep link proizvodi jedinstveni resource link ID.

#### Tema modula vs Inline Quicklink

Odaberite pristup **tema modula** kada:

- Rasprava je primarna aktivnost za taj korak u modulu.
- Želite da tema bude vidljiva u Brightspace-ovom sadržaju, praćenju dovršenosti i Class Progressu.

Odaberite pristup **inline ugradnje** kada:

- Komentari trebaju stajati ispod drugog sadržaja na istoj stranici.
- Ne želite zaseban predmet koji se prati dovršenost u sadržaju.

#### Vidljivost, Draft i uvjeti objave

Nova FastComments tema je prema zadanim postavkama vidljiva studentima. Da biste je sakrili dok je postavljate:

1. U uređivaču sadržaja kliknite naslov teme (Classic) ili izbornik s tri točke na aktivnosti (New Content Experience).
2. Postavite status na **Draft** (Classic) ili isključite **Visibility** (New Content Experience).

Draft teme su nevidljive studentima. Predavači i asistenti i dalje ih vide s oznakom "Draft".

Da biste ograničili temu na određenu grupu ili odjeljak:

1. Otvorite temu.
2. Kliknite izbornik naslova teme > **Edit Properties In-place** (Classic) ili **Edit** > **Restrictions** (New Content Experience).
3. Pod **Release Conditions**, kliknite **Create**.
4. Odaberite **Group enrollment** ili **Section enrollment**, odaberite grupu/odjeljak i spremite.

Uvjeti objave se zbrajaju s FastComments-ovim mapiranjem uloga. Studenti koji ne mogu vidjeti temu neće dobiti LTI lansiranje.

#### Što studenti vide pri prvom pokretanju

Kada student klikne temu (ili učita HTML temu s ugradnjom):

1. Brightspace izvodi LTI 1.3 lansiranje u pozadini.
2. FastComments prima studentovo ime, email, URL avatara i LMS ulogu, te ih automatski prijavljuje. Nema FastComments zahtjeva za prijavom.
3. Nit komentara za taj resource link se prikazuje unutar Brightspace iframea.

Mapiranje uloga pri lansiranju:

- Brightspace `Administrator` postaje FastComments **admin** za nit (puni pristup moderiranju, brisanju, zabrani i konfiguraciji).
- Brightspace `Instructor` postaje FastComments **moderator** (pinanje, skrivanje, brisanje, zabrana).
- Sve ostale uloge (`Learner`, `TeachingAssistant`, itd.) postaju standardni komentatori.

Komentari se pripisuju studentskom Brightspace računu. Ako student izmijeni svoje ime ili avatar u Brightspaceu, sljedeće LTI lansiranje sinhronizira promjenu.

#### Visina iframea i promjena veličine

FastComments šalje postMessage `org.imsglobal.lti.frameResize` pri svakom renderiranju niti i pri promjenama sadržaja (novi komentar, proširi odgovore). Brightspace sluša tu poruku i prilagođava visinu iframea tako da nit nije odsječena i ne prikazuje unutarnji scrollbar.

Ako iframe ostaje na fiksnoj maloj visini:

- Potvrdite da je kolegij učitan preko HTTPS-a. Brightspace-ov postMessage slušatelj odbacuje frameove s miješanim sadržajem.
- Potvrdite da nijedno proširenje preglednika ne blokira postMessage kanal.
- Za inline ugradnje u HTML temi, okolni HTML ne smije omotavati iframe u kontejner fiksne visine. Uklonite bilo koji inline `style="height: ..."` iz roditeljskog elementa.

#### Specifične zamke Brightspace-a

**Alat se ne prikazuje u odabiraču Add Existing.** Implementacija nije omogućena za org jedinicu ovog kolegija. Administrator mora dodati org jedinicu (ili roditelja) na popis Org Units implementacije. Sama registracija alata nije dovoljna; implementacija određuje koji kolegiji vide alat.

**`deployment_id` mismatch pri lansiranju.** FastComments TOFU-piniraj prvi `deployment_id` koji vidi za registraciju. Ako administrator izbriše izvornu implementaciju i stvori novu, lansiranja iz nove implementacije se odbijaju s greškom neusklađenosti implementacije. Rješenje je ponovno registrirati FastComments (generirajte novu registration URL (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">preuzmite ga ovdje</a>) i ponovno pokrenite Dynamic Registration); stari zapis konfiguracije se zamjenjuje.

**Alat se pokreće, ali prikazuje "Invalid LTI launch".** Kolegij je u drugačijoj strukturi tenancy/organizacije nego što implementacija pokriva, ili je implementacija onemogućena nakon registracije. Ponovno provjerite **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments > prekidač **Enabled** i popis org jedinica implementacije.

**Imena i uloge nedostaju unutar FastComments.** Brightspace šalje LTI lansiranja s Names and Role Provisioning Services (NRPS) tvrdnjama. Ako je kolegij nadograđen s starije LTI 1.1 veze, lansiranje nema `name` i `email` claims. Ponovo dodajte FastComments temu putem **Add Existing** (nemojte migrirati staru vezu) tako da lansiranje koristi LTI 1.3.

**Ugradnja prikazuje zaslon za prijavu umjesto automatskog SSO-a.** HTML tema je umetnuta kao običan `<iframe>` koji pokazuje na FastComments umjesto putem **Insert Stuff** > **LTI Advantage**. Obični iframeovi preskaču LTI lansiranje i dovode korisnike na javnu FastComments stranicu. Izbrišite iframe i ponovo umetnite putem toka Insert Stuff.