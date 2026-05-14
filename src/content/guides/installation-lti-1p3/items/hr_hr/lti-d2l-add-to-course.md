Ova stranica objašnjava kako dodati FastComments u Brightspace tečaj nakon što je administrator registrirao alat i stvorio implementaciju. Ako alat još nije registriran, prvo pogledajte vodič za registraciju D2L-a.

<div class="screenshot white-bg">
    <div class="title">FastComments ugrađen kao tema jedinice u Brightspace</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-d2l-comments-in-unit.png" alt="FastComments pokrenut unutar Brightspace jedinice, prikazujući threaded komentare i birač @-spominjanja" />
</div>

Brightspace isporučuje dva načina za izradu sadržaja: **Klasični sadržaj** i **Novo iskustvo sadržaja** (također nazvano **Lessons**). Oba izlažu FastComments, ali se putanje kroz izbornik razlikuju. Svaki odjeljak u nastavku pokriva oba gdje se razlikuju.

#### Pronađite FastComments alat

FastComments alat pojavljuje se na dva mjesta unutar uređivača sadržaja tečaja:

1. Izbor aktivnosti (activity picker), do kojeg se pristupa putem gumba **Add Existing** u modulu/jedinici (u starijim verzijama Brightspace označeno **Add Existing Activities**). FastComments se u trenutnim verzijama Brightspace prikazuje izravno u pickeru; starije verzije ga ugnijezde pod podizbornik **External Learning Tools**. Bilo koja od tih putanja dodaje FastComments kao zasebnu temu.
2. Dijalog **Insert Stuff** unutar HTML uređivača, pod **LTI Advantage**. Ovo ugrađuje FastComments inline u HTML temu putem LTI deep linking toka.

Ako FastComments ne pojavljuje ni u jednom pickeru, implementacija nije omogućena za organizacijsku jedinicu (org unit) koja drži tečaj. Zamolite svog Brightspace administratora da otvori **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments tool > **View Deployments**, otvori implementaciju i doda org jedinicu tečaja (ili roditeljsku org jedinicu) pod **Org Units**.

#### Dodavanje FastComments kao teme u modu

Klasični sadržaj:

1. Otvorite tečaj i kliknite **Content** u navigacijskoj traci.
2. Odaberite modul koji bi trebao sadržavati raspravu (ili ga stvorite putem **Add a module**).
3. Kliknite **Add Existing** (stariji Brightspace: **Add Existing Activities** > **External Learning Tools**).
4. U pickeru kliknite **FastComments**. Brightspace stvara temu u modulu i vraća vas u prikaz sadržaja.
5. Kliknite novu temu. Preimenujte je u nešto opisno kao `FastComments Discussion` koristeći inline editor naslova.

Novo iskustvo sadržaja (Lessons):

1. Otvorite tečaj i kliknite **Content**.
2. Otvorite jedinicu i lekciju koja bi trebala sadržavati raspravu.
3. Kliknite **Add** > **Existing Activity** i odaberite **FastComments** (stariji Brightspace: ugniježđeno pod **External Learning Tools**).
4. Aktivnost se dodaje u lekciju.
5. Kliknite naslov aktivnosti da biste ga preimenovali.

Prvi put kada bilo koji korisnik (instruktor ili student) otvori temu, FastComments inicijalizira thread za tu resource link vezu. Thread je vezan za resource link ID, tako da preimenovanje ili premještanje teme ne mijenja koji se thread učitava.

#### Ugradite FastComments inline u HTML temu

Koristite ovaj tok kada želite da komentari budu prikazani ispod teksta, videa ili drugog sadržaja unutar iste stranice teme, umjesto kao zasebna tema.

1. Otvorite ili stvorite HTML temu u modulu/lekciji.
2. Kliknite **Edit HTML** da otvorite Brightspace HTML uređivač.
3. Postavite kursor na mjesto gdje bi se thread komentara trebao pojaviti.
4. Kliknite gumb **Insert Stuff** (ikona puzzle komadića u alatnoj traci uređivača).
5. U dijalogu Insert Stuff skrolajte do **LTI Advantage** i kliknite **FastComments**.
6. FastComments otvara deep linking picker. Potvrdite mjesto (zadane opcije rade za rasprave o sadržaju); kliknite **Insert** ili **Continue**.
7. Brightspace se vraća u HTML uređivač s rezervnim blokom koji predstavlja LTI launch. Kliknite **Save and Close** na temi.

Kad se tema učita, Brightspace zamjenjuje rezervni blok iframeom koji automatski pokreće FastComments putem LTI-a. Studenti vide thread rasprave inline.

Jedna HTML tema može sadržavati više deep-linked FastComments ugradnji. Svaka ugradnja dobiva svoj vlastiti thread jer svaki deep link proizvodi jedinstveni resource link ID.

#### Téma modula nasuprot inline brzog povezivanja

Odaberite pristup **teme modula** kada:

- Rasprava je primarna aktivnost za taj korak u modulu.
- Želite da tema bude vidljiva u Brightspace tablici sadržaja (table of contents), praćenju dovršenosti i Class Progress.

Odaberite pristup **inline ugradnje** kada:

- Komentari trebaju stajati ispod drugog sadržaja na istoj stranici.
- Ne želite zaseban predmet koji se prati dovršenost u tablici sadržaja.

#### Vidljivost, Draft i Release uvjeti

Nova FastComments tema je zadano vidljiva studentima. Da je sakrijete dok je postavljate:

1. U uređivaču sadržaja kliknite naslov teme (Klasični) ili izbornik s tri točke na aktivnosti (Novo iskustvo sadržaja).
2. Postavite status na **Draft** (Klasični) ili isključite **Visibility** (Novo iskustvo sadržaja).

Draft teme su nevidljive studentima. Instruktori i asistenti i dalje ih vide s oznakom "Draft".

Da ograničite temu na određenu grupu ili sekciju:

1. Otvorite temu.
2. Kliknite izbornik naslova teme > **Edit Properties In-place** (Klasični) ili **Edit** > **Restrictions** (Novo iskustvo sadržaja).
3. Pod **Release Conditions**, kliknite **Create**.
4. Odaberite **Group enrollment** ili **Section enrollment**, odaberite grupu/sekciju i spremite.

Release uvjeti se zbrajaju s FastComments vlastitim mapiranjem uloga. Studenti koji ne mogu vidjeti temu neće dobiti LTI launch.

#### Što studenti vide pri prvom pokretanju

Kada student klikne temu (ili učita HTML temu s ugradnjom):

1. Brightspace izvodi LTI 1.3 launch u pozadini.
2. FastComments prima studentsko ime, email, URL avatara i LMS ulogu, te ih automatski prijavljuje. Nema FastComments prozora za prijavu.
3. Thread komentara za taj resource link se prikazuje unutar Brightspace iframe-a.

Mapiranje uloga pri launchu:

- Brightspace `Administrator` postaje FastComments **admin** za thread (puno moderiranje, brisanje, zabrane i pristup konfiguraciji).
- Brightspace `Instructor` postaje FastComments **moderator** (pinanje, skrivanje, brisanje, zabrana).
- Sve ostale uloge (`Learner`, `TeachingAssistant`, itd.) postaju standardni komentatori.

Komentari se pripisuju studentskom Brightspace računu. Ako student izmijeni ime ili avatar u Brightspaceu, sljedeći LTI launch sinkronizira promjenu.

#### Visina iframe-a i promjena veličine

FastComments emitira `org.imsglobal.lti.frameResize` postMessage pri svakom prikazu threada i pri promjenama sadržaja (novi komentar, proširi odgovore). Brightspace sluša tu poruku i prilagođava visinu iframe-a tako da thread nije odsječen i ne prikazuje unutarnji scrollbar.

Ako iframe ostane na fiksnoj niskoj visini:

- Potvrdite da se tečaj učitava preko HTTPS-a. Brightspace-ov postMessage listener odbacuje frameove s miješanim sadržajem.
- Potvrdite da nijedno proširenje preglednika ne blokira postMessage kanal.
- Za inline ugradnje u HTML temi, okolni HTML ne smije omotavati iframe u kontejner fiksne visine. Uklonite bilo koji inline `style="height: ..."` s roditeljskog elementa.

#### Brightspace-specifične zamke

**Alat se ne pojavljuje u Add Existing pickeru.** Implementacija nije omogućena za org jedinicu ovog tečaja. Administrator treba dodati org jedinicu (ili roditelja) u popis Org Units implementacije. Sama registracija alata nije dovoljna; implementacija određuje koji tečajevi vide alat.

**`deployment_id` mismatch na launchu.** FastComments TOFU-pinningom bilježi prvi `deployment_id` koji vidi za registraciju. Ako administrator izbriše izvornu implementaciju i stvori novu, launchovi s nove implementacije se odbijaju s greškom mismatcha deploymenta. Rješenje je ponovno registrirati FastComments (generirati novu registration URL i ponovno pokrenuti Dynamic Registration); stari zapis konfiguracije se zamjenjuje.

**Alat se pokreće, ali prikazuje "Invalid LTI launch".** Tečaj je u drugom tenantu/organizacijskoj strukturi nego što implementacija pokriva, ili je implementacija onemogućena nakon registracije. Ponovno provjerite **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments > prekidač **Enabled** i popis org jedinica implementacije.

**Imena i uloge nedostaju unutar FastComments.** Brightspace šalje LTI launcheve s Names and Role Provisioning Services (NRPS) tvrdnjama. Ako je tečaj nadograđen sa starijeg LTI 1.1 linka, launch nedostaje `name` i `email` tvrdnje. Ponovo dodajte FastComments temu putem **Add Existing** (ne migrirajte stari link) kako bi launch koristio LTI 1.3.

**Ugradnja prikazuje ekran za prijavu umjesto automatskog SSO.** HTML tema je umetnuta kao običan `<iframe>` koji pokazuje na FastComments umjesto putem **Insert Stuff** > **LTI Advantage**. Obični iframeovi preskaču LTI launch i dovode korisnike na javnu FastComments stranicu. Izbrišite iframe i ponovno ga umetnite putem toka Insert Stuff.