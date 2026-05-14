Ova stranica objašnjava dodavanje FastComments-a u Brightspace tečaj nakon što je administrator registrirao alat i stvorio deployment. Ako alat još nije registriran, najprije pogledajte vodič za D2L registraciju.

Brightspace isporučuje dva iskustva za izradu sadržaja: **Classic Content** i **New Content Experience** (također nazvan **Lessons**). Oba izlažu FastComments, ali se putanje u izborniku razlikuju. Svaki je odjeljak u nastavku obuhvaća oba gdje se razlikuju.

#### Locate the FastComments Tool

FastComments alat pojavljuje se na dva mjesta unutar uređivača sadržaja tečaja:

1. U activity picker-u, do kojeg se dolazi iz modula/odjeljka klikom na **Add Existing** gumb (u starijim verzijama Brightspace označeno **Add Existing Activities**). FastComments se pojavljuje izravno u pickeru u trenutnim verzijama Brightspace-a; u starijim verzijama nalazi se unutar **External Learning Tools** podizbornika. Bilo koja od tih putanja dodaje FastComments kao samostalnu temu.
2. U **Insert Stuff** dijalogu unutar HTML uređivača, pod **LTI Advantage**. Ovo ugrađuje FastComments inline u HTML temu preko LTI deep linking toka.

Ako se FastComments ne pojavi u nijednom pickeru, deployment nije omogućen za org jedinicu koja drži tečaj. Zamolite vašeg Brightspace administratora da otvori **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments tool > **View Deployments**, otvori deployment i doda org jedinicu tečaja (ili roditeljsku org jedinicu) pod **Org Units**.

#### Add FastComments as a Topic in a Module

Classic Content:

1. Otvorite tečaj i kliknite **Content** u navigaciji.
2. Odaberite modul koji bi trebao sadržavati raspravu (ili ga stvorite putem **Add a module**).
3. Kliknite **Add Existing** (stariji Brightspace: **Add Existing Activities** > **External Learning Tools**).
4. U pickeru kliknite **FastComments**. Brightspace stvara temu u modulu i vraća vas na prikaz sadržaja.
5. Kliknite novu temu. Preimenujte je u nešto opisno poput `FastComments Discussion` koristeći inline editor naslova.

New Content Experience (Lessons):

1. Otvorite tečaj i kliknite **Content**.
2. Otvorite jedinicu i lesson koji bi trebao sadržavati raspravu.
3. Kliknite **Add** > **Existing Activity** i odaberite **FastComments** (u starijim verzijama Brightspace: ugniježđeno pod **External Learning Tools**).
4. Aktivnost je dodana u lesson.
5. Kliknite naslov aktivnosti da biste ga preimenovali.

Prvi put kada bilo koji korisnik (instruktor ili student) otvori temu, FastComments inicijalizira thread za taj resource link. Thread je vezan za resource link ID, pa preimenovanje ili premještanje teme ne mijenja koji će thread biti učitan.

#### Embed FastComments Inline in an HTML Topic

Koristite ovaj tok kada želite da se komentari pojavljuju ispod štiva, videa ili drugog sadržaja unutar iste stranice teme umjesto kao zasebna tema.

1. Otvorite ili stvorite HTML temu u modulu/lessonu.
2. Kliknite **Edit HTML** da otvorite Brightspace HTML editor.
3. Postavite kursor na mjesto gdje bi se thread komentara trebao pojaviti.
4. Kliknite gumb **Insert Stuff** (ikona puzzla u traci alata editor-a).
5. U Insert Stuff dijalogu skrolajte do **LTI Advantage** i kliknite **FastComments**.
6. FastComments otvara deep linking picker. Potvrdite pozicioniranje (zadane opcije rade za rasprave sadržaja); kliknite **Insert** ili **Continue**.
7. Brightspace se vraća u HTML editor s placeholder blokom koji predstavlja LTI launch. Kliknite **Save and Close** na temi.

Kad se tema učita, Brightspace zamjenjuje placeholder iframe-om koji automatski pokreće FastComments putem LTI-a. Studenti vide thread rasprave inline.

Jedna HTML tema može sadržavati više deep-linked FastComments embedova. Svaki embed dobiva vlastiti thread jer svaki deep link stvara jedinstveni resource link ID.

#### Module Topic vs Inline Quicklink

Odaberite pristup **module topic** kada:

- Rasprava je primarna aktivnost za taj korak u modulu.
- Želite da tema bude vidljiva u Brightspace-ovom table of contents, praćenju dovršetka i Class Progress.

Odaberite pristup **inline embed** kada:

- Komentari bi trebali sjediti ispod drugog sadržaja na istoj stranici.
- Ne želite zasebnu stavku koja se prati dovršetak u table of contents.

#### Visibility, Draft, and Release Conditions

Nova FastComments tema je zadano vidljiva studentima. Da biste je sakrili dok je postavljate:

1. U uređivaču sadržaja kliknite naslov teme (Classic) ili izbornik s tri točkice na aktivnosti (New Content Experience).
2. Postavite status na **Draft** (Classic) ili isključite **Visibility** (New Content Experience).

Draft teme su nevidljive studentima. Instruktori i asistenti i dalje ih vide s oznakom "Draft".

Da ograničite temu na određenu grupu ili sekciju:

1. Otvorite temu.
2. Kliknite izbornik naslova teme > **Edit Properties In-place** (Classic) ili **Edit** > **Restrictions** (New Content Experience).
3. Pod **Release Conditions**, kliknite **Create**.
4. Odaberite **Group enrollment** ili **Section enrollment**, odaberite grupu/sekciju i spremite.

Release conditions se slažu s FastComments-ovim vlastitim mapiranjem uloga. Studenti koji ne mogu vidjeti temu neće dobiti LTI launch.

#### What Students See on First Launch

Kad student klikne temu (ili učita HTML temu s embedom):

1. Brightspace izvršava LTI 1.3 launch u pozadini.
2. FastComments prima studentovo ime, email, URL avatara i LMS ulogu, te ih automatski prijavljuje. Nema FastComments prompta za prijavu.
3. Thread komentara za taj resource link se renderira unutar Brightspace iframe-a.

Mapiranje uloga pri launchu:

- Brightspace `Administrator` postaje FastComments **administrator** za thread (puna moderacija, brisanje, ban i pristup konfiguraciji).
- Brightspace `Instructor` postaje FastComments **moderator** (pinanje, skrivanje, brisanje, ban).
- Sve ostale uloge (`Learner`, `TeachingAssistant`, itd.) postaju standardni komentatori.

Komentari se pripisuju studentovom Brightspace računu. Ako student promijeni svoje ime ili avatar u Brightspace-u, sljedeći LTI launch sinkronizira promjenu.

#### Iframe Height and Resize

FastComments šalje `org.imsglobal.lti.frameResize` postMessage pri svakom renderu threada i pri promjenama sadržaja (novi komentar, proširi odgovore). Brightspace sluša ovu poruku i prilagođava visinu iframe-a tako da thread nije odsječen i ne prikazuje unutarnji scrollbar.

Ako iframe ostaje na fiksnoj maloj visini:

- Provjerite da je tečaj učitan preko HTTPS-a. Brightspace-ov postMessage listener odbacuje frame-ove s miješanim sadržajem.
- Provjerite da nijedno proširenje preglednika ne blokira postMessage kanal.
- Za inline embedove u HTML temi, okolni HTML ne smije omotavati iframe u kontejner fiksne visine. Uklonite bilo koji inline `style="height: ..."` iz roditeljskog elementa.

#### Brightspace-Specific Gotchas

**Tool not showing in the Add Existing picker.** Deployment nije omogućen za org jedinicu ovog tečaja. Administrator treba dodati org jedinicu (ili roditelja) na deployment-ov popis **Org Units**. Sama registracija alata nije dovoljna; deployment određuje koji tečajevi vide alat.

**`deployment_id` mismatch on launch.** FastComments TOFU-pinne prvi `deployment_id` koji vidi za registraciju. Ako administrator izbriše izvorni deployment i stvori novi, launchovi s novog deploymenta se odbijaju s pogreškom mismatcha deploymenta. Rješenje je ponovno registrirati FastComments (generirati novu registration URL i ponovno pokrenuti Dynamic Registration); stari zapis konfiguracije se zamjenjuje.

**Tool launches but shows "Invalid LTI launch".** Tečaj se nalazi u drugačijoj tenantskoj/org strukturi nego što deployment pokriva, ili je deployment onemogućen nakon registracije. Ponovno provjerite **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments > **Enabled** toggle i popis org jedinica u deploymentu.

**Names and roles missing inside FastComments.** Brightspace isporučuje LTI launchove s Names and Role Provisioning Services (NRPS) claimovima. Ako je tečaj nadograđen sa starijeg LTI 1.1 linka, launch nedostaje `name` i `email` claimove. Ponovno dodajte FastComments temu putem **Add Existing** (ne migrirajte stari link) tako da launch koristi LTI 1.3.

**Embed shows a login screen instead of auto-SSO.** HTML tema je umetnuta kao običan `<iframe>` koji pokazuje na FastComments umjesto putem **Insert Stuff** > **LTI Advantage**. Obični iframe-ovi preskaču LTI launch i vode korisnike na javnu FastComments stranicu. Izbrišite iframe i ponovno umetnite putem Insert Stuff toka.