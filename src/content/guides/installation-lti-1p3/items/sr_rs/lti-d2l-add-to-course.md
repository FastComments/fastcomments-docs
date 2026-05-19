Ova stranica opisuje dodavanje FastComments u Brightspace kurs nakon što je administrator registrovao alat i kreirao deployment. Ako alat još nije registrovan, prvo pogledajte D2L vodič za registraciju.

<div class="screenshot white-bg">
    <div class="title">FastComments ugrađen kao tema jedinice u Brightspace</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-d2l-comments-in-unit.png" alt="FastComments running inside a Brightspace unit, showing threaded comments and an @-mention picker" />
</div>

Brightspace nudi dva iskustva za kreiranje sadržaja: **Classic Content** i **New Content Experience** (takođe nazvan **Lessons**). Oba omogućavaju FastComments, ali su putevi u meniju različiti. Svaki odeljak ниже pokriva oba gde se razlikuju.

#### Pronađite FastComments alat

FastComments alat se pojavljuje na dva mesta u editoru sadržaja kursa:

1. U izboru aktivnosti, dostupan preko dugmeta **Add Existing** modula/jedinice (u starijim verzijama Brightspace označeno **Add Existing Activities**). FastComments se u trenutnim buildovima pojavljuje direktno u pickeru; u starijim verzijama je ugnježden pod podmenijem **External Learning Tools**. Bilo koji put dodaje FastComments kao samostalnu temu.
2. U dijalogu **Insert Stuff** unutar HTML editora, pod **LTI Advantage**. Ovo ugrađuje FastComments inline u HTML temu putem LTI deep linking toka.

Ako FastComments nije vidljiv ni u jednom pickeru, deployment nije omogućen za org unit koji drži kurs. Zamolite Brightspace administratora da otvori **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments tool > **View Deployments**, otvori deployment i doda org unit kursa (ili roditeljski org unit) pod **Org Units**.

#### Dodajte FastComments kao temu u modul

Classic Content:

1. Otvorite kurs i kliknite **Content** u navigacionom baru.
2. Izaberite modul koji treba da sadrži diskusiju (ili kreirajte jedan preko **Add a module**).
3. Kliknite **Add Existing** (stariji Brightspace: **Add Existing Activities** > **External Learning Tools**).
4. U pickeru kliknite **FastComments**. Brightspace kreira temu u modulu i vraća vas na prikaz sadržaja.
5. Kliknite novu temu. Preimenujte je u nešto deskriptivno kao `FastComments Discussion` koristeći inline uređivač naslova.

New Content Experience (Lessons):

1. Otvorite kurs i kliknite **Content**.
2. Otvorite jedinicu i lekciju koja treba da sadrži diskusiju.
3. Kliknite **Add** > **Existing Activity** i izaberite **FastComments** (stariji Brightspace: ugnježdeno pod **External Learning Tools**).
4. Aktivnost je dodata u lekciju.
5. Kliknite naslov aktivnosti da biste je preimenovali.

Prvi put kada bilo koji korisnik (instruktor ili student) otvori temu, FastComments inicijalizuje nit za tu resource link. Nit je vezana za resource link ID, tako da promena imena ili premještanje teme ne menja koja nit se učitava.

#### Ugradite FastComments inline u HTML temu

Koristite ovaj tok kada želite da se komentari pojave ispod teksta, videa ili drugog sadržaja unutar iste stranice teme umesto kao posebna tema.

1. Otvorite ili kreirajte HTML temu u modulu/lekciji.
2. Kliknite **Edit HTML** da otvorite Brightspace HTML editor.
3. Postavite kursor tamo gde treba da se pojavi nit komentara.
4. Kliknite dugme **Insert Stuff** (ikonica slagalice u traci editora).
5. U Insert Stuff dijalogu, skrolujte do **LTI Advantage** i kliknite **FastComments**.
6. FastComments otvara deep linking picker. Potvrdite pozicioniranje (podrazumevane opcije rade za diskusije sadržaja); kliknite **Insert** ili **Continue**.
7. Brightspace se vraća u HTML editor sa placeholder blokom koji predstavlja LTI launch. Kliknite **Save and Close** na temi.

Kada se tema učita, Brightspace zamenjuje placeholder iframe-om koji automatski pokreće FastComments putem LTI. Studenti vide nit diskusije inline.

Jedna HTML tema može sadržavati više deep-linked FastComments embedova. Svaki embed dobija sopstvenu nit jer svaki deep link proizvodi različit resource link ID.

#### Tema modula naspram inline quicklinka

Izaberite pristup **tema modula** kada:

- Diskusija je primarna aktivnost za taj korak u modulu.
- Želite da tema bude vidljiva u Brightspace sadržaju, praćenju dovršenosti i Class Progress.

Izaberite pristup **inline embed** kada:

- Komentari treba da stoje ispod drugog sadržaja na istoj stranici.
- Ne želite zasebnu stavku praćenu za dovršenost u tabeli sadržaja.

#### Vidljivost, Draft i uslovi objavljivanja

Nova FastComments tema je po podrazumevanju vidljiva studentima. Da biste je sakrili dok je podešavate:

1. U editoru sadržaja, kliknite naslov teme (Classic) ili meni sa tri tačke na aktivnosti (New Content Experience).
2. Postavite status na **Draft** (Classic) ili isključite **Visibility** (New Content Experience).

Draft teme su nevidljive studentima. Instruktori i TA i dalje ih vide sa oznakom "Draft".

Da biste ograničili temu na određenu grupu ili sekciju:

1. Otvorite temu.
2. Kliknite meni naslova teme > **Edit Properties In-place** (Classic) ili **Edit** > **Restrictions** (New Content Experience).
3. Pod **Release Conditions**, kliknite **Create**.
4. Izaberite **Group enrollment** ili **Section enrollment**, odaberite grupu/sekciju i sačuvajte.

Uslovi objavljivanja se sabiraju sa FastComments sopstvenim mapiranjem uloga. Studenti koji ne mogu da vide temu ne dobijaju LTI launch.

#### Šta studenti vide pri prvom pokretanju

Kada student klikne temu (ili učita HTML temu sa embedom):

1. Brightspace izvodi LTI 1.3 launch u pozadini.
2. FastComments prima studentovo ime, email, URL avatara i LMS ulogu, i automatski ga prijavljuje. Nema FastComments prozora za prijavu.
3. Nit komentara za taj resource link se renderuje unutar Brightspace iframe-a.

Mapiranje uloga pri launchu:

- Brightspace `Administrator` postaje FastComments administrator za nit (puna moderacija, brisanje, banovanje i pristup konfiguraciji).
- Brightspace `Instructor` postaje FastComments moderator (pin, sakrij, obriši, banovanje).
- Sve ostale uloge (`Learner`, `TeachingAssistant`, itd.) postaju standardni komentatori.

Komentari se pripisuju studentovom Brightspace nalogu. Ako student izmeni svoje ime ili avatar u Brightspace-u, sledeći LTI launch sinhronizuje promenu.

#### Zaključajte javni pristup (preporučeno)

Po podrazumevanju, FastComments podaci komentara su javno čitljivi. Bilo ko ko može da pogodi URL niti ili API endpoint može videti komentare, čak i van Brightspace-a. Za diskusije u kursu gotovo sigurno želite da ograničite pregled samo na upisane polaznike.

Otvorite vašu <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">stranicu za prilagođavanje widgeta</a> i kreirajte pravilo sa omogućenom opcijom **Require SSO To View Comments**, zatim postavite nivo bezbednosti na **Secure SSO** tako da se niti mogu učitavati samo putem potpisanog LTI launch-a.

Pogledajte [Protecting Comment Threads With Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) za kompletnu proceduru, uključujući kako da ograničite pravilo na jedan domen ili stranicu.

#### Visina iframe-a i promene veličine

FastComments emituje `org.imsglobal.lti.frameResize` postMessage pri svakom renderovanju niti i pri promenama sadržaja (novi komentar, proširi odgovore). Brightspace sluša ovu poruku i prilagođava visinu iframe-a tako da nit nije odsečena i da se ne prikazuje unutrašnji scrollbar.

Ako iframe ostane na fiksnoj maloj visini:

- Potvrdite da je kurs učitan preko HTTPS-a. Brightspace-ov postMessage listener odbacuje mixed-content frame-ove.
- Potvrdite da nijedno proširenje pregledača ne blokira postMessage kanal.
- Za inline embedove u HTML temi, okruživajući HTML ne sme da obavija iframe u kontejner fiksne visine. Uklonite bilo koji inline `style="height: ..."` sa roditeljskog elementa.

#### Specifični problemi u Brightspace-u

**Alat se ne prikazuje u Add Existing pickeru.** Deployment nije omogućen za org unit ovog kursa. Administrator treba da doda org unit (ili roditelja) na listu **Org Units** deployment-a. Sama registracija alata nije dovoljna; deployment određuje koji kursevi vide alat.

**`deployment_id` mismatch pri launchu.** FastComments TOFU-pin-uje prvi `deployment_id` koji vidi za registraciju. Ako administrator obriše originalni deployment i kreira novi, pokretanja iz novog deployment-a bivaju odbijena sa greškom o neusaglašenosti deployment-a. Rešenje je da se ponovo registruje FastComments (generišite novi registration URL (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">nabavite ga ovde</a>) i ponovo pokrenite Dynamic Registration); stara konfiguraciona evidencija se zamenjuje.

**Alat se pokreće ali prikazuje "Invalid LTI launch".** Kurs je u drugačijoj tenant/org strukturi nego što deployment pokriva, ili je deployment onemogućen nakon registracije. Ponovo proverite **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments > **Enabled** prekidač i listu org unit-a za deployment.

**Imena i uloge nedostaju u FastComments-u.** Brightspace šalje LTI launch-e sa Names and Role Provisioning Services (NRPS) tvrdnjama. Ako je kurs nadograđen sa starijeg LTI 1.1 linka, launch može da ne sadrži `name` i `email` tvrdnje. Ponovo dodajte FastComments temu preko **Add Existing** (ne migrirajte stari link) tako da launch koristi LTI 1.3.

**Embed prikazuje ekran za prijavu umesto automatskog SSO.** HTML tema je ubačena kao običan `<iframe>` koji pokazuje na FastComments umesto putem **Insert Stuff** > **LTI Advantage**. Obični iframe-ovi preskaču LTI launch i korisnici slijeću na javnu FastComments stranicu. Obrišite iframe i ponovo umetnite putem Insert Stuff toka.