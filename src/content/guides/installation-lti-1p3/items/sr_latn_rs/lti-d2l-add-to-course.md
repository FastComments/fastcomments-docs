Ova stranica objašnjava dodavanje FastComments-a u Brightspace kurs nakon što je administrator registrovao alat i kreirao deployment. Ako alat još nije registrovan, prvo pogledajte D2L vodič za registraciju.

<div class="screenshot white-bg">
    <div class="title">FastComments ugrađen kao tema jedinice u Brightspace</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-d2l-comments-in-unit.png" alt="FastComments pokrenut unutar Brightspace jedinice, prikazujući threaded komentare i birač @-pominjanja" />
</div>

Brightspace isporučuje dva iskustva za kreiranje sadržaja: **Classic Content** i **New Content Experience** (takođe nazvano **Lessons**). Oba izlažu FastComments, ali se putanje kroz menije razlikuju. Svaki odeljak ispod pokriva oba slučaja tamo gde se razlikuju.

#### Pronađite FastComments alat

FastComments alat se pojavljuje na dva mesta unutar uređivača sadržaja kursa:

1. U activity picker-u, do kojeg se dolazi iz dugmeta **Add Existing** modula/jedinice (u starijim verzijama Brightspace-a označeno kao **Add Existing Activities**). FastComments se pojavljuje direktno u picker-u u trenutnim verzijama Brightspace-a; starije verzije ga ugnježdene stavljaju pod podmeni **External Learning Tools**. Bilo koja od ovih putanja dodaje FastComments kao samostalnu temu.
2. U dijalogu **Insert Stuff** unutar HTML uređivača, pod **LTI Advantage**. Ovo ugrađuje FastComments inline u HTML temu putem LTI deep linking toka.

Ako se FastComments ne pojavljuje ni u jednom picker-u, deployment nije omogućen za org unit koji sadrži kurs. Zamolite vašeg Brightspace administratora da otvori **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments tool > **View Deployments**, otvori deployment i doda org unit kursa (ili roditeljski org unit) pod **Org Units**.

#### Dodajte FastComments kao temu u modulu

Classic Content:

1. Otvorite kurs i kliknite **Content** u navigacionom baru.
2. Izaberite modul koji treba da sadrži diskusiju (ili ga kreirajte preko **Add a module**).
3. Kliknite **Add Existing** (stariji Brightspace: **Add Existing Activities** > **External Learning Tools**).
4. U picker-u kliknite **FastComments**. Brightspace kreira temu u modulu i vraća vas u prikaz sadržaja.
5. Kliknite novu temu. Preimenujte je u nešto opisno kao `FastComments Discussion` koristeći inline editor naslova.

New Content Experience (Lessons):

1. Otvorite kurs i kliknite **Content**.
2. Otvorite jedinicu i lekciju koja treba da sadrži diskusiju.
3. Kliknite **Add** > **Existing Activity** i izaberite **FastComments** (stariji Brightspace: ugnježdeno pod **External Learning Tools**).
4. Aktivnost je dodata u lekciju.
5. Kliknite naslov aktivnosti da biste je preimenovali.

Prvi put kada bilo koji korisnik (predavač ili student) otvori temu, FastComments inicijalizuje thread za taj resource link. Thread je vezan za resource link ID, tako da preimenovanje ili premeštanje teme ne menja koji se thread učitava.

#### Ugradite FastComments inline u HTML temu

Koristite ovaj tok kada želite da komentari budu ispod štiva, videa ili drugog sadržaja unutar iste stranice teme, umesto kao zasebna tema.

1. Otvorite ili kreirajte HTML temu u modulu/lekciji.
2. Kliknite **Edit HTML** da otvorite Brightspace HTML uređivač.
3. Postavite kursor na mesto gde treba da se pojavi thread komentara.
4. Kliknite dugme **Insert Stuff** (ikonica slagalice u toolbar-u uređivača).
5. U dijalogu Insert Stuff, skrolujte do **LTI Advantage** i kliknite **FastComments**.
6. FastComments otvara deep linking picker. Potvrdite pozicioniranje (podrazumevane opcije rade za diskusije o sadržaju); kliknite **Insert** ili **Continue**.
7. Brightspace se vrati u HTML uređivač sa placeholder blokom koji predstavlja LTI launch. Kliknite **Save and Close** na temi.

Kada se tema učita, Brightspace zamenjuje placeholder iframe-om koji automatski pokreće FastComments putem LTI. Studenti vide thread diskusije inline.

Jedna HTML tema može da sadrži više deep-linked FastComments ugradnji. Svaka ugradnja dobija sopstveni thread jer svaki deep link proizvodi različit resource link ID.

#### Tema modula vs inline Quicklink

Izaberite pristup **tema modula** kada:

- Diskusija je primarna aktivnost za taj korak u modulu.
- Želite da tema bude prikazana u sadržaju Brightspace-a, u praćenju završetka i Class Progress.

Izaberite pristup **inline embed** kada:

- Komentari treba da stoje ispod drugog sadržaja na istoj stranici.
- Ne želite zaseban predmet koji se prati za završetak u sadržaju.

#### Vidljivost, Draft i uslovi objavljivanja

Nova FastComments tema je po defaultu vidljiva studentima. Da je sakrijete dok je postavljate:

1. U uređivaču sadržaja kliknite naslov teme (Classic) ili meni sa tri tačke na aktivnosti (New Content Experience).
2. Postavite status na **Draft** (Classic) ili isključite **Visibility** (New Content Experience).

Draft teme su nevidljive studentima. Instruktori i asistenti i dalje ih vide sa oznakom "Draft".

Da ograničite temu na određenu grupu ili sekciju:

1. Otvorite temu.
2. Kliknite meni naslova teme > **Edit Properties In-place** (Classic) ili **Edit** > **Restrictions** (New Content Experience).
3. Pod **Release Conditions**, kliknite **Create**.
4. Izaberite **Group enrollment** ili **Section enrollment**, izaberite grupu/sekciju i sačuvajte.

Uslovi objavljivanja se nadovezuju na FastComments-ovo mapiranje uloga. Studenti koji ne mogu da vide temu neće dobiti LTI launch.

#### Šta studenti vide pri prvom pokretanju

Kada student klikne temu (ili učita HTML temu sa ugradnjom):

1. Brightspace izvršava LTI 1.3 launch u pozadini.
2. FastComments prima studentovo ime, email, URL avatara i LMS ulogu, i automatski ga prijavljuje. Ne pojavljuje se FastComments prompt za prijavu.
3. Thread komentara za taj resource link se prikazuje unutar Brightspace iframe-a.

Mapiranje uloga pri launch-u:

- Brightspace `Administrator` postaje FastComments **admin** za thread (puno moderisanje, brisanje, banovanje i pristup konfiguraciji).
- Brightspace `Instructor` postaje FastComments **moderator** (pin, hide, delete, ban).
- Sve ostale uloge (`Learner`, `TeachingAssistant`, itd.) postaju standardni komentatori.

Komentari se dodeljuju studentovom Brightspace nalogu. Ako student promeni svoje ime ili avatar u Brightspace-u, sledeći LTI launch sinhronizuje promenu.

#### Visina iframe-a i promena veličine

FastComments emituje `org.imsglobal.lti.frameResize` postMessage pri svakom renderovanju threada i pri promenama sadržaja (novi komentar, proširi odgovore). Brightspace sluša ovu poruku i prilagođava visinu iframe-a tako da thread nije isečen i da se ne prikazuje unutrašnji scrollbar.

Ako iframe ostane fiksno kratak:

- Potvrdite da se kurs učitava preko HTTPS-a. Brightspace-ov postMessage listener odbacuje mixed-content frame-ove.
- Potvrdite da nijedna browser ekstenzija ne blokira postMessage kanal.
- Za inline ugradnje u HTML temi, okruživajući HTML ne sme umotavati iframe u kontejner fiksne visine. Uklonite bilo koji inline `style="height: ..."` sa parent elementa.

#### Specifične zamke Brightspace-a

**Alat se ne prikazuje u Add Existing picker-u.** Deployment nije omogućen za org unit ovog kursa. Administrator treba da doda org unit (ili roditelja) u listu Org Units za deployment. Sama registracija alata nije dovoljna; deployment određuje koji kursevi vide alat.

**`deployment_id` mismatch na launch-u.** FastComments TOFU-pin-uje prvi `deployment_id` koji vidi za registraciju. Ako administrator obriše originalni deployment i kreira novi, launch-ovi iz novog deployment-a će biti odbijeni sa greškom mismatch-a deployment-a. Rešenje je ponovo registrovati FastComments (generisati novi registration URL i ponovo pokrenuti Dynamic Registration); stara konfiguraciona evidencija se zamenjuje.

**Alat se pokreće ali prikazuje "Invalid LTI launch".** Kurs je u drugačijoj tenant/org strukturi nego što deployment pokriva, ili je deployment onemogućen nakon registracije. Ponovo proverite **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments > **Enabled** prekidač i listu org unit-a za deployment.

**Imena i uloge nedostaju unutar FastComments-a.** Brightspace šalje LTI launch-e sa Names and Role Provisioning Services (NRPS) claim-ovima. Ako je kurs nadograđen sa starijeg LTI 1.1 linka, launch može da nema `name` i `email` claim-ove. Ponovo dodajte FastComments temu preko **Add Existing** (ne migrirajte stari link) tako da launch koristi LTI 1.3.

**Ugradnja prikazuje ekran za prijavu umesto automatskog SSO-a.** HTML tema je ubačena kao običan `<iframe>` koji pokazuje na FastComments umesto putem **Insert Stuff** > **LTI Advantage**. Obični iframe preskače LTI launch i korisnike vodi na javnu FastComments stranicu. Obrišite iframe i ponovo ga umetnite putem Insert Stuff toka.