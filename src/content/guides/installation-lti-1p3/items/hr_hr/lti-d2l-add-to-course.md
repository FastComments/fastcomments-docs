Ova stranica objašnjava kako dodati FastComments u Brightspace tečaj nakon što je administrator registrirao alat i kreirao deployment. Ako alat još nije registriran, prvo pogledajte vodič za registraciju D2L.

<div class="screenshot white-bg">
    <div class="title">FastComments ugrađen kao tema jedinice u Brightspaceu</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-d2l-comments-in-unit.png" alt="FastComments pokrenut unutar Brightspace jedinice, prikazujući ugniježdene komentare i izbornik za @-spominjanje" />
</div>

Brightspace nudi dva iskustva za izradu sadržaja: **Classic Content** i **New Content Experience** (također nazvan **Lessons**). Oba izlažu FastComments, ali putevi kroz izbornike se razlikuju. Svaki odjeljak u nastavku pokriva oba gdje se razilaze.

#### Pronađite FastComments alat

FastComments alat pojavljuje se na dva mjesta unutar uređivača sadržaja tečaja:

1. U izboru aktivnosti, pristupačno iz modula/jednice preko gumba **Add Existing** (u starijim verzijama Brightspacea označeno **Add Existing Activities**). U novijim izdanjima Brightspace-a FastComments se prikazuje izravno u izboru; u starijim verzijama nalazi se podpodizbornik **External Learning Tools**. Bilo koji put dodaje FastComments kao zasebnu temu.
2. Dijalog **Insert Stuff** unutar HTML uređivača, pod **LTI Advantage**. Ovo ugrađuje FastComments inline u HTML temu putem LTI deep linking toka.

Ako FastComments nije dostupan ni u jednom od izbornika, deployment nije omogućen za org jedinicu koja drži tečaj. Zamolite svog Brightspace administratora da otvori **Admin Tools** > **Upravljanje proširivošću** > **LTI Advantage** > FastComments alat > **View Deployments**, otvori deployment i doda org jedinicu tečaja (ili roditeljsku org jedinicu) pod **Org Units**.

#### Dodajte FastComments kao temu u modulu

Classic Content:

1. Otvorite tečaj i kliknite **Content** u navigacijskoj traci.
2. Odaberite modul koji treba sadržavati raspravu (ili ga kreirajte putem **Add a module**).
3. Kliknite **Add Existing** (stariji Brightspace: **Add Existing Activities** > **External Learning Tools**).
4. U izboru kliknite **FastComments**. Brightspace stvara temu u modulu i vraća vas u prikaz sadržaja.
5. Kliknite novu temu. Preimenujte je u nešto opisno poput `FastComments Discussion` koristeći uređivač naslova u liniji.

New Content Experience (Lessons):

1. Otvorite tečaj i kliknite **Content**.
2. Otvorite jedinicu i lekciju koja treba sadržavati raspravu.
3. Kliknite **Add** > **Existing Activity** i odaberite **FastComments** (stariji Brightspace: ugniježdeno pod **External Learning Tools**).
4. Aktivnost se dodaje u lekciju.
5. Kliknite naslov aktivnosti da biste ga preimenovali.

Prvi put kad bilo koji korisnik (instruktor ili student) otvori temu, FastComments inicijalizira thread za tu resource link. Thread je vezan uz ID resource linka, tako da promjena naziva ili premještanje teme ne mijenja koji se thread učitava.

#### Ugradite FastComments inline u HTML temu

Koristite ovaj tok kad želite da se komentari pojave ispod čitanja, videa ili drugog sadržaja unutar iste stranice teme umjesto kao zasebna tema.

1. Otvorite ili kreirajte HTML temu u modulu/lekciji.
2. Kliknite **Edit HTML** da otvorite Brightspace HTML uređivač.
3. Postavite kursor na mjesto gdje bi se thread komentara trebao pojaviti.
4. Kliknite gumb **Insert Stuff** (ikona slagalice u alatnoj traci uređivača).
5. U dijalogu Insert Stuff skrolajte do **LTI Advantage** i kliknite **FastComments**.
6. FastComments otvara deep linking picker. Potvrdite mjesto (zadane opcije funkcioniraju za rasprave o sadržaju); kliknite **Insert** ili **Continue**.
7. Brightspace se vraća u HTML uređivač s privremenim blokom koji predstavlja LTI launch. Kliknite **Save and Close** na temi.

Kad se tema učita, Brightspace zamjenjuje privremeni blok iframeom koji automatski pokreće FastComments putem LTI-ja. Studenti vide thread rasprave inline.

Jedna HTML tema može sadržavati više deep-linked FastComments ugradnji. Svaka ugradnja dobiva vlastiti thread jer svaki deep link proizvodi jedinstveni resource link ID.

#### Tema modula naspram inline brzog linka

Odaberite pristup **tema modula** kada:

- Rasprava je primarna aktivnost za taj korak u modulu.
- Želite da tema bude vidljiva u Brightspaceovom sadržaju, praćenju završetka i Class Progress.

Odaberite pristup **inline ugradnje** kada:

- Komentari trebaju stajati ispod drugog sadržaja na istoj stranici.
- Ne želite zasebnu stavku koja se prati u sadržaju kao dovršetak.

#### Vidljivost, Draft i uvjeti objave

Nova FastComments tema je prema zadanim postavkama vidljiva studentima. Da je sakrijete dok je postavljate:

1. U uređivaču sadržaja kliknite naslov teme (Classic) ili izbornik s tri točke na aktivnosti (New Content Experience).
2. Postavite status na **Draft** (Classic) ili isključite **Visibility** (New Content Experience).

Draft teme su nevidljive studentima. Instruktori i asistenti i dalje ih vide s oznakom "Draft".

Da ograničite temu na određenu grupu ili sekciju:

1. Otvorite temu.
2. Kliknite izbornik naslova teme > **Edit Properties In-place** (Classic) ili **Edit** > **Restrictions** (New Content Experience).
3. Pod **Release Conditions**, kliknite **Create**.
4. Odaberite **Group enrollment** ili **Section enrollment**, odaberite grupu/sekciju i spremite.

Uvjeti objave se zbrajaju s FastCommentsovim vlastitim mapiranjem uloga. Studenti koji ne mogu vidjeti temu ne dobivaju LTI launch.

#### Što studenti vide pri prvom pokretanju

Kad student klikne temu (ili učita HTML temu s ugradnjom):

1. Brightspace izvršava LTI 1.3 launch u pozadini.
2. FastComments prima studentsko ime, e-mail, URL avatara i LMS ulogu te ih automatski prijavljuje. Nema FastComments prijavnog upita.
3. Thread komentara za taj resource link prikazuje se unutar Brightspace iframea.

Mapiranje uloga pri launchu:

- Brightspace `Administrator` postaje FastComments **admin** za thread (puna moderacija, brisanje, blokiranje i pristup konfiguraciji).
- Brightspace `Instructor` postaje FastComments **moderator** (pinanje, skrivanje, brisanje, blokiranje).
- Sve ostale uloge (`Learner`, `TeachingAssistant`, itd.) postaju obični komentatori.

Komentari su pripisani studentskom Brightspace računu. Ako student promijeni svoje ime ili avatar u Brightspaceu, sljedeći LTI launch sinkronizira promjenu.

#### Ograničite javni pristup (preporučeno)

Prema zadanim postavkama, podaci komentara FastComments-a su javno čitljivi. Bilo tko tko pogodi URL threada ili API endpoint može vidjeti njegove komentare, čak i izvan Brightspacea. Za rasprave u tečaju vjerojatno želite ograničiti pregled samo na upisane polaznike.

Otvorite svoju <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">stranicu za prilagodbu widgeta</a> i kreirajte pravilo s omogućenom opcijom **Zahtijevaj SSO za pregled komentara**, zatim postavite sigurnosnu razinu na **Sigurni SSO** tako da se threadovi mogu učitavati samo putem potpisanog LTI launch-a.

Pogledajte [Protecting Comment Threads With Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) za potpuni vodič, uključujući kako ograničiti pravilo na jedinstvenu domenu ili stranicu.

#### Visina iframea i promjena veličine

FastComments šalje `org.imsglobal.lti.frameResize` postMessage pri svakom renderiranju threada i kod promjena sadržaja (novi komentar, proširi odgovore). Brightspace sluša za tu poruku i prilagođava visinu iframea kako thread ne bi bio odsječen i kako se ne bi pojavio unutarnji scrollbar.

Ako iframe ostaje na fiksnoj maloj visini:

- Potvrdite da je tečaj učitan preko HTTPS-a. Brightspaceov postMessage listener odbija frameove s miješanim sadržajem.
- Potvrdite da nijedno proširenje preglednika ne blokira postMessage kanal.
- Za inline ugradnje u HTML temu, okolni HTML ne smije omotati iframe u kontejner fiksne visine. Uklonite bilo koji inline `style="height: ..."` iz roditeljskog elementa.

#### Specifične zamke za Brightspace

**Alat se ne prikazuje u Add Existing pickeru.** Deployment nije omogućen za org jedinicu ovog tečaja. Administrator treba dodati org jedinicu (ili roditeljsku) na popis Org Units deploymenta. Sama registracija alata nije dovoljna; deployment definira koji tečajevi vide alat.

**`deployment_id` se ne poklapa pri launchu.** FastComments TOFU-prišiva prvi `deployment_id` koji vidi za registraciju. Ako administrator obriše izvornik deployment i kreira novi, launchovi iz novog deploymenta se odbijaju s greškom neslaganja deploymenta. Rješenje je ponovo registrirati FastComments (generirajte novi registration URL (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">nabavite ga ovdje</a>) i pokrenite Dynamic Registration ponovno); stara konfiguracija se zamjenjuje.

**Alat se pokreće, ali pokazuje "Invalid LTI launch".** Tečaj je u drugoj strukturi tenant/organizacije nego što deployment pokriva, ili je deployment onemogućen nakon registracije. Ponovno provjerite **Admin Tools** > **Upravljanje proširivošću** > **LTI Advantage** > FastComments > prekidač **Omogućeno** i popis org jedinica deploymenta.

**Imena i uloge nedostaju unutar FastComments-a.** Brightspace šalje LTI launcheve s Names and Role Provisioning Services (NRPS) claimovima. Ako je tečaj nadograđen iz starije LTI 1.1 veze, launch možda nema `name` i `email` claimove. Ponovo dodajte FastComments temu putem **Add Existing** (nemojte migrirati staru vezu) kako bi launch koristio LTI 1.3.

**Ugradnja prikazuje zaslon za prijavu umjesto automatskog SSO-a.** HTML tema je umetnuta kao običan `<iframe>` koji pokazuje na FastComments umjesto putem **Insert Stuff** > **LTI Advantage**. Obični iframeovi preskaču LTI launch i korisnike dovode na javnu FastComments stranicu. Obrišite iframe i ponovno umetnite putem Insert Stuff toka.