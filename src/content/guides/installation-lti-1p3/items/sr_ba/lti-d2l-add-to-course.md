Ova stranica opisuje dodavanje FastComments u Brightspace kurs nakon što je administrator registrovao alat i kreirao deployment. Ako alat još nije registrovan, prvo pogledajte D2L vodič za registraciju.

<div class="screenshot white-bg">
    <div class="title">FastComments ugrađen kao tema jedinice u Brightspace</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-d2l-comments-in-unit.png" alt="FastComments running inside a Brightspace unit, showing threaded comments and an @-mention picker" />
</div>

Brightspace isporučuje dva iskustva za kreiranje sadržaja: **Classic Content** i **New Content Experience** (također nazvan **Lessons**). Oba omogućavaju FastComments, ali putanje kroz meni se razlikuju. Svaki odjeljak ispod pokriva obje opcije gdje se razlikuju.

#### Locate the FastComments Tool

FastComments alat se pojavljuje na dva mjesta unutar editora sadržaja kursa:

1. Activity picker, dostupan preko dugmeta **Add Existing** u modulu/jedinici (u starijim verzijama Brightspace označeno **Add Existing Activities**). U trenutnim izdanjima Brightspace, FastComments je odmah vidljiv u pickeru; starije verzije ga stavljaju pod podmeni **External Learning Tools**. Bilo koja od ovih putanja dodaje FastComments kao zasebnu temu.
2. **Insert Stuff** dijalog unutar HTML editora, pod **LTI Advantage**. Ovo ugrađuje FastComments inline u HTML temu putem LTI deep linking toka.

Ako se FastComments ne pojavljuje ni u jednom pickeru, deployment nije omogućen za org unit koji drži kurs. Zamolite vašeg Brightspace administratora da otvori **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments tool > **View Deployments**, otvori deployment i doda org unit kursa (ili roditeljski org unit) pod **Org Units**.

#### Add FastComments as a Topic in a Module

Classic Content:

1. Otvorite kurs i kliknite **Content** u navigacijskoj traci.
2. Odaberite modul koji treba sadržavati diskusiju (ili kreirajte jedan putem **Add a module**).
3. Kliknite **Add Existing** (stariji Brightspace: **Add Existing Activities** > **External Learning Tools**).
4. U pickeru kliknite **FastComments**. Brightspace kreira temu u modulu i vraća vas na prikaz sadržaja.
5. Kliknite novu temu. Preimenujte je u nešto opisno kao `FastComments Discussion` koristeći inline uređivač naslova.

New Content Experience (Lessons):

1. Otvorite kurs i kliknite **Content**.
2. Otvorite jedinicu i lesson koja treba sadržavati diskusiju.
3. Kliknite **Add** > **Existing Activity** i odaberite **FastComments** (stariji Brightspace: ugniježđen pod **External Learning Tools**).
4. Aktivnost se dodaje u lesson.
5. Kliknite naslov aktivnosti da biste je preimenovali.

Prvi put kada bilo koji korisnik (instruktor ili student) otvori temu, FastComments inicijalizuje thread za taj resource link. Thread je vezan za resource link ID, pa preimenovanje ili premještanje teme ne mijenja koji thread se učitava.

#### Embed FastComments Inline in an HTML Topic

Koristite ovaj tok kada želite da komentari budu prikazani ispod teksta, videa ili drugog sadržaja unutar iste stranice teme umjesto kao zasebna tema.

1. Otvorite ili kreirajte HTML temu u modulu/lessonu.
2. Kliknite **Edit HTML** da otvorite Brightspace HTML editor.
3. Postavite kursor na mjesto gdje treba da se pojavi thread komentara.
4. Kliknite dugme **Insert Stuff** (ikona puzzle komada u toolbaru editora).
5. U Insert Stuff dijalogu, skrolajte do **LTI Advantage** i kliknite **FastComments**.
6. FastComments otvara deep linking picker. Potvrdite poziciju (zadane opcije rade za content discussions); kliknite **Insert** ili **Continue**.
7. Brightspace se vraća u HTML editor sa placeholder blokom koji predstavlja LTI launch. Kliknite **Save and Close** na temi.

Kada se tema učita, Brightspace zamijeni placeholder iframe-om koji automatski pokreće FastComments putem LTI. Studenti vide thread diskusije inline.

Jedna HTML tema može sadržavati više deep-linked FastComments embedova. Svaki embed dobija svoj thread zato što svaki deep link proizvodi jedinstveni resource link ID.

#### Module Topic vs Inline Quicklink

Odaberite pristup **module topic** kada:

- Diskusija je primarna aktivnost za taj korak u modulu.
- Želite da tema bude prikazana u Brightspace table of contents, praćenju završetka i Class Progress.

Odaberite pristup **inline embed** kada:

- Komentari treba da stoje ispod drugog sadržaja na istoj stranici.
- Ne želite zaseban item koji je praćen u tabeli sadržaja za završetak.

#### Visibility, Draft, and Release Conditions

Nova FastComments tema je po defaultu vidljiva studentima. Da je sakrijete dok je pripremate:

1. U editoru sadržaja, kliknite naslov teme (Classic) ili tri-tačkasti meni na aktivnosti (New Content Experience).
2. Postavite status na **Draft** (Classic) ili isključite **Visibility** (New Content Experience).

Draft teme su nevidljive studentima. Instruktori i asistenti ih i dalje vide sa oznakom "Draft".

Da ograničite temu na specifičnu grupu ili sekciju:

1. Otvorite temu.
2. Kliknite meni naslova teme > **Edit Properties In-place** (Classic) ili **Edit** > **Restrictions** (New Content Experience).
3. Pod **Release Conditions**, kliknite **Create**.
4. Odaberite **Group enrollment** ili **Section enrollment**, izaberite grupu/sekciju i sačuvajte.

Release conditions se nadovezuju na FastComments-ovo vlastito mapiranje uloga. Studenti koji ne mogu vidjeti temu ne dobijaju LTI launch.

#### What Students See on First Launch

Kada student klikne temu (ili učita HTML temu sa embedom):

1. Brightspace izvodi LTI 1.3 launch u pozadini.
2. FastComments prima studentsko ime, email, URL avatara i LMS ulogu, i automatski ih prijavljuje. Nema FastComments zahtjeva za login.
3. Thread komentara za taj resource link se renderuje unutar Brightspace iframe-a.

Role mapping pri launchu:

- Brightspace `Administrator` postaje FastComments **admin** za thread (puni moderacijski pristup, brisanje, ban i pristup konfiguraciji).
- Brightspace `Instructor` postaje FastComments **moderator** (pin, hide, delete, ban).
- Sve ostale uloge (`Learner`, `TeachingAssistant`, itd.) postaju standardni komentatori.

Komentari su atribuirani studentskom Brightspace nalogu. Ako student izmijeni svoje ime ili avatar u Brightspace, sljedeći LTI launch sinhronizuje promjenu.

#### Lock Down Public Access (Recommended)

Po defaultu, FastComments podaci komentara su javno čitljivi. Bilo ko ko može pogoditi URL thread-a ili API endpoint može vidjeti komentare, čak i izvan Brightspace. Za diskusije u kursu gotovo sigurno želite ograničiti pregled samo na upisane polaznike.

Otvorite vašu <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">widget customization page</a> i kreirajte pravilo sa omogućenom opcijom **Require SSO To View Comments**, zatim postavite security level na **Secure SSO** tako da se threadovi mogu učitavati samo kroz potpisani LTI launch.

Pogledajte [Protecting Comment Threads With Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) za kompletan vodič, uključujući kako ograničiti pravilo na jednu domenu ili stranicu.

#### Iframe Height and Resize

FastComments emituje `org.imsglobal.lti.frameResize` postMessage pri svakom renderovanju threada i pri promjenama sadržaja (novi komentar, proširi odgovore). Brightspace sluša za ovu poruku i prilagođava visinu iframe-a tako da thread nije odsječen i da se ne pojavljuje unutrašnji scrollbar.

Ako iframe ostane na fiksnoj maloj visini:

- Potvrdite da je kurs učitan preko HTTPS-a. Brightspace-ov postMessage listener odbacuje mixed-content frame-ove.
- Potvrdite da nijedna browser ekstenzija ne blokira postMessage kanal.
- Za inline embedove u HTML temi, okolni HTML ne smije obavijati iframe u kontejner fiksne visine. Uklonite bilo koji inline `style="height: ..."` sa roditeljskog elementa.

#### Brightspace-Specific Gotchas

**Tool not showing in the Add Existing picker.** Deployment nije omogućen za org unit ovog kursa. Administrator treba dodati org unit (ili roditelja) na deploymentov **Org Units** spisak. Sama registracija alata nije dovoljna; deployment određuje koji kursevi vide alat.

**`deployment_id` mismatch on launch.** FastComments TOFU-pinsira prvi `deployment_id` koji vidi za registraciju. Ako administrator obriše originalni deployment i kreira novi, launch-ovi iz novog deploymenta se odbacuju sa greškom mismatch deployment-a. Rješenje je ponovo registrovati FastComments (generišite novi registration URL (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">get it here</a>) i pokrenite Dynamic Registration ponovo); stari konfiguracioni zapis se zamijeni.

**Tool launches but shows "Invalid LTI launch".** Kurs je u drugačijoj tenant/org strukturi nego što deployment pokriva, ili je deployment bio onemogućen nakon registracije. Ponovo provjerite **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments > **Enabled** toggle i spisak org unit-a na deploymentu.

**Names and roles missing inside FastComments.** Brightspace šalje LTI launch-e sa Names and Role Provisioning Services (NRPS) claim-ovima. Ako je kurs nadograđen sa starijeg LTI 1.1 linka, launch može nedostajati `name` i `email` claim-ove. Ponovo dodajte FastComments temu putem **Add Existing** (nemojte migrirati stari link) tako da launch koristi LTI 1.3.

**Embed shows a login screen instead of auto-SSO.** HTML tema je umetnuta kao običan `<iframe>` koji pokazuje na FastComments umjesto putem **Insert Stuff** > **LTI Advantage**. Obični iframe-ovi zaobilaze LTI launch i korisnici završavaju na javnoj FastComments stranici. Obrišite iframe i ponovo umetnite putem Insert Stuff toka.