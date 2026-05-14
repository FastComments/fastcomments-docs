Ova stranica opisuje dodavanje FastComments u Brightspace kurs nakon što je administrator registrovao alat i kreirao deployment. Ako alat još nije registrovan, prvo pogledajte D2L vodič za registraciju.

Brightspace nudi dva okruženja za kreiranje sadržaja: **Classic Content** i **New Content Experience** (takođe nazvano **Lessons**). Oba omogućavaju FastComments, ali putanje kroz meni se razlikuju. Svaki odeljak ispod pokriva obe opcije tamo gde se razlikuju.

#### Pronađite FastComments alat

FastComments alat se pojavljuje na dva mesta unutar editora sadržaja kursa:

1. Izbor aktivnosti, do kojeg se dolazi kroz dugme modula/jedinice **Add Existing** (u starijim verzijama Brightspace označeno **Add Existing Activities**). FastComments se u aktuelnim verzijama Brightspace pojavljuje direktno u pickeru; u starijim verzijama je ugnježden pod podmenijem **External Learning Tools**. Bilo koja od ovih putanja dodaje FastComments kao zasebnu temu.
2. Dijalog **Insert Stuff** unutar HTML editora, pod **LTI Advantage**. Ovo ugrađuje FastComments inline u HTML temu putem LTI deep linking toka.

Ako se FastComments ne pojavljuje ni u jednom pickeru, deployment nije omogućen za org jedinicu koja drži kurs. Zamolite vašeg Brightspace administratora da otvori **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments tool > **View Deployments**, otvori deployment i doda org jedinicu kursa (ili neku nadređenu org jedinicu) pod **Org Units**.

#### Dodavanje FastComments kao teme u modulu

Classic Content:

1. Otvorite kurs i kliknite **Content** u navigacionom baru.
2. Izaberite modul koji treba da sadrži diskusiju (ili ga kreirajte putem **Add a module**).
3. Kliknite **Add Existing** (stariji Brightspace: **Add Existing Activities** > **External Learning Tools**).
4. U pickeru kliknite **FastComments**. Brightspace kreira temu u modulu i vraća vas na prikaz sadržaja.
5. Kliknite novu temu. Preimenujte je u nešto opisno poput `FastComments Discussion` koristeći inline editor naslova.

New Content Experience (Lessons):

1. Otvorite kurs i kliknite **Content**.
2. Otvorite jedinicu i lesson koji treba da sadrže diskusiju.
3. Kliknite **Add** > **Existing Activity** i izaberite **FastComments** (stariji Brightspace: ugnježdeno pod **External Learning Tools**).
4. Aktivnost se doda u lesson.
5. Kliknite na naslov aktivnosti da biste je preimenovali.

Prvi put kada bilo koji korisnik (instruktor ili student) otvori temu, FastComments inicijalizuje thread za taj resource link. Thread je vezan za resource link ID, tako da preimenovanje ili pomeranje teme ne menja koji thread se učitava.

#### Ugradnja FastComments inline u HTML temu

Koristite ovaj tok kada želite da se komentari pojavljuju ispod štiva, video zapisa ili drugog sadržaja unutar iste stranice teme umesto kao zasebna tema.

1. Otvorite ili kreirajte HTML temu u modulu/lessonu.
2. Kliknite **Edit HTML** da otvorite Brightspace HTML editor.
3. Postavite kursor na mesto gde treba da se pojavi thread komentara.
4. Kliknite dugme **Insert Stuff** (ikona slagalice u toolbaru editora).
5. U Insert Stuff dijalogu skrolujte do **LTI Advantage** i kliknite **FastComments**.
6. FastComments otvara deep linking picker. Potvrdite pozicioniranje (podrazumevane opcije rade za diskusije o sadržaju); kliknite **Insert** ili **Continue**.
7. Brightspace vraća HTML editor sa placeholder blokom koji predstavlja LTI launch. Kliknite **Save and Close** na temi.

Kada se tema učita, Brightspace zamenjuje placeholder iframe-om koji automatski pokreće FastComments putem LTI-ja. Studenti vide thread diskusije inline.

Jedna HTML tema može sadržati više deep-linked FastComments ugradnji. Svaka ugradnja dobija sopstveni thread jer svaki deep link proizvodi jedinstven resource link ID.

#### Tema modula vs Inline Quicklink

Izaberite pristup **tema modula** kada:

- Diskusija je primarna aktivnost za taj korak u modulu.
- Želite da tema bude prikazana u Brightspace-ovom sadržaju, praćenju završetka i Class Progress.

Izaberite pristup **inline ugradnje** kada:

- Komentari treba da stoje ispod drugog sadržaja na istoj stranici.
- Ne želite zaseban stavku koja se prati za završetak u tabeli sadržaja.

#### Vidljivost, Draft i uslovi objavljivanja

Nova FastComments tema je po defaultu vidljiva studentima. Da je sakrijete dok je podešavate:

1. U editoru sadržaja kliknite naslov teme (Classic) ili trostruki meni na aktivnosti (New Content Experience).
2. Podesite status na **Draft** (Classic) ili isključite **Visibility** (New Content Experience).

Draft teme su nevidljive studentima. Instruktori i TA i dalje ih vide sa oznakom "Draft".

Da ograničite temu na određenu grupu ili sekciju:

1. Otvorite temu.
2. Kliknite meni naslova teme > **Edit Properties In-place** (Classic) ili **Edit** > **Restrictions** (New Content Experience).
3. Pod **Release Conditions**, kliknite **Create**.
4. Izaberite **Group enrollment** ili **Section enrollment**, odaberite grupu/sekciju i sačuvajte.

Uslovi objavljivanja se slažu sa FastComments sopstvenim mapiranjem uloga. Studenti koji ne mogu da vide temu neće dobiti LTI launch.

#### Šta studenti vide pri prvom pokretanju

Kada student klikne temu (ili učita HTML temu sa ugradnjom):

1. Brightspace izvršava LTI 1.3 launch u pozadini.
2. FastComments prima studentovo ime, email, URL avatara i LMS ulogu, i automatski ga prijavljuje. Ne pojavljuje se FastComments prompt za prijavu.
3. Thread komentara za taj resource link se renderuje unutar Brightspace iframe-a.

Mapiranje uloga pri launch-u:

- Brightspace `Administrator` postaje FastComments **admin** za thread (puna moderacija, brisanje, zabrana i pristup konfiguraciji).
- Brightspace `Instructor` postaje FastComments **moderator** (pinovanje, skrivanje, brisanje, zabrana).
- Sve ostale uloge (`Learner`, `TeachingAssistant`, itd.) postaju standardni komentatori.

Komentari su pripisani studentovom Brightspace nalogu. Ako student izmeni svoje ime ili avatar u Brightspace-u, sledeći LTI launch sinhronizuje promenu.

#### Visina iframe-a i promene veličine

FastComments emituje `org.imsglobal.lti.frameResize` postMessage pri svakom renderovanju threada i pri promenama sadržaja (novi komentar, proširenje odgovora). Brightspace sluša ovu poruku i prilagođava visinu iframe-a tako da thread nije isečen i da ne prikazuje unutrašnju traku za skrolovanje.

Ako iframe ostaje na fiksnoj maloj visini:

- Potvrdite da se kurs učitava preko HTTPS-a. Brightspace-ov postMessage listener odbija frame-ove sa mešovitim sadržajem.
- Potvrdite da nijedna ekstenzija pretraživača ne blokira postMessage kanal.
- Za inline ugradnje u HTML temi, okolni HTML ne sme da obavija iframe u kontejner fiksne visine. Uklonite bilo koji inline `style="height: ..."` sa roditeljskog elementa.

#### Specifične zamerke za Brightspace

**Alat se ne prikazuje u Add Existing pickeru.** Deployment nije omogućen za org jedinicu ovog kursa. Administrator treba da doda org jedinicu (ili neku nadređenu) u listu **Org Units** deployment-a. Sama registracija alata nije dovoljna; deployment određuje koji kursevi vide alat.

**`deployment_id` mismatch na launch-u.** FastComments TOFU-pinuje prvi `deployment_id` koji vidi za registraciju. Ako administrator obriše originalni deployment i kreira novi, launch-ovi sa novog deployment-a se odbacuju sa greškom o neusaglašenosti deployment-a. Rešenje je ponovo registrovati FastComments (generisati novi registration URL i pokrenuti Dynamic Registration ponovo); stara konfiguraciona stavka se zamenjuje.

**Alat se pokreće ali prikazuje "Invalid LTI launch".** Kurs se nalazi u drugom tenant/org strukturi nego što deployment pokriva, ili je deployment onemogućen nakon registracije. Ponovo proverite **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments > prekidač **Enabled** i listu org jedinica za deployment.

**Imena i uloge nedostaju unutar FastComments.** Brightspace šalje LTI launch-e sa Names and Role Provisioning Services (NRPS) claim-ovima. Ako je kurs nadograđen sa starijeg LTI 1.1 linka, launch može da nema `name` i `email` claim-ove. Ponovo dodajte FastComments temu putem **Add Existing** (nemigrirajte stari link) tako da launch koristi LTI 1.3.

**Ugradnja prikazuje ekran za prijavu umesto automatskog SSO.** HTML tema je umetnuta kao običan `<iframe>` koji pokazuje direktno na FastComments umesto preko **Insert Stuff** > **LTI Advantage**. Obični iframe-ovi preskaču LTI launch i dovode korisnike na javnu FastComments stranicu. Obrišite taj iframe i ponovo ga umetnite putem Insert Stuff toka.