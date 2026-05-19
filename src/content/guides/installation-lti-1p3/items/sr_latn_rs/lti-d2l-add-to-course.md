Ova stranica objašnjava kako dodati FastComments u Brightspace kurs nakon što je administrator registrovao alat i kreirao deployment. Ako alat još nije registrovan, prvo pogledajte D2L vodič za registraciju.

<div class="screenshot white-bg">
    <div class="title">FastComments ugrađen kao temu jedinice u Brightspace-u</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-d2l-comments-in-unit.png" alt="FastComments pokrenut unutar Brightspace jedinice, prikazujući threaded komentare i picker za @-pominjanje" />
</div>

Brightspace isporučuje dva iskustva za autorstvo sadržaja: **Classic Content** i **New Content Experience** (takođe nazvano **Lessons**). Oba omogućavaju FastComments, ali se putanje kroz meni razlikuju. Svaki odeljak ispod pokriva oba gde se razlikuju.

#### Locate the FastComments Tool

Alat FastComments se pojavljuje na dve lokacije unutar uređivača sadržaja kursa:

1. U activity picker-u, do kojeg se dolazi preko **Add Existing** dugmeta u modulu/jedinici (označeno kao **Add Existing Activities** u starijim verzijama Brightspace-a). FastComments se pojavljuje direktno u picker-u u aktuelnim izdanjima Brightspace-a; starije verzije ga ugrađuju pod **External Learning Tools** podmeni. Bilo koji put dodaje FastComments kao zasebnu temu.
2. U **Insert Stuff** dijalogu unutar HTML uređivača, pod **LTI Advantage**. Ovo ugrađuje FastComments inline u HTML temu putem LTI deep linking toka.

Ako se FastComments ne pojavljuje ni u jednom picker-u, deployment nije omogućen za org unit koji drži kurs. Zamolite vašeg Brightspace administratora da otvori **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments alat > **View Deployments**, otvori deployment i doda org unit kursa (ili roditeljski org unit) pod **Org Units**.

#### Add FastComments as a Topic in a Module

Classic Content:

1. Otvorite kurs i kliknite **Content** u navigacionom baru.
2. Izaberite modul koji bi trebalo da sadrži diskusiju (ili kreirajte jedan preko **Add a module**).
3. Kliknite **Add Existing** (stariji Brightspace: **Add Existing Activities** > **External Learning Tools**).
4. U picker-u, kliknite **FastComments**. Brightspace kreira temu u modulu i vraća vas na pregled sadržaja.
5. Kliknite novu temu. Preimenujte je u nešto opisno kao `FastComments Discussion` koristeći inline editor naslova.

New Content Experience (Lessons):

1. Otvorite kurs i kliknite **Content**.
2. Otvorite jedinicu i lesson koji bi trebalo da sadrže diskusiju.
3. Kliknite **Add** > **Existing Activity** i izaberite **FastComments** (stariji Brightspace: ugnježdeno pod **External Learning Tools**).
4. Aktivnost je dodata u lesson.
5. Kliknite na naslov aktivnosti da biste je preimenovali.

Prvi put kad bilo koji korisnik (instruktor ili student) otvori temu, FastComments inicijalizuje thread za taj resource link. Thread je vezan za resource link ID, tako da preimenovanje ili premestanje teme ne menja koji thread se učitava.

#### Embed FastComments Inline in an HTML Topic

Koristite ovaj tok kada želite da se komentari pojave ispod čitanja, videa ili drugog sadržaja unutar iste stranice teme, umesto kao zasebna tema.

1. Otvorite ili kreirajte HTML temu u modulu/lesson-u.
2. Kliknite **Edit HTML** da otvorite Brightspace HTML uređivač.
3. Postavite kursor tamo gde thread komentara treba da se pojavi.
4. Kliknite dugme **Insert Stuff** (ikonica slagalice u toolbar-u uređivača).
5. U Insert Stuff dijalogu, skrolujte do **LTI Advantage** i kliknite **FastComments**.
6. FastComments otvara deep linking picker. Potvrdite pozicioniranje (podrazumevana podešavanja rade za diskusije o sadržaju); kliknite **Insert** ili **Continue**.
7. Brightspace se vraća u HTML uređivač sa placeholder blokom koji predstavlja LTI launch. Kliknite **Save and Close** na temi.

Kada se tema učita, Brightspace zamenjuje placeholder iframe-om koji automatski pokreće FastComments preko LTI-a. Studenti vide thread diskusije inline.

Jedna HTML tema može držati više deep-linked FastComments ugradnji. Svaka ugradnja dobija svoj thread jer svaki deep link proizvodi različit resource link ID.

#### Module Topic vs Inline Quicklink

Izaberite pristup **module topic** kada:

- Diskusija je primarna aktivnost za taj korak u modulu.
- Želite da tema bude vidljiva u Brightspace-ovom sadržaju, praćenju kompletiranja i Class Progress.

Izaberite pristup **inline embed** kada:

- Komentari treba da stoje ispod drugog sadržaja na istoj stranici.
- Ne želite zaseban item koji se prati u tabeli sadržaja.

#### Visibility, Draft, and Release Conditions

Nova FastComments tema je po defaultu vidljiva studentima. Da je sakrijete dok je podešavate:

1. U uređivaču sadržaja, kliknite naslov teme (Classic) ili meni sa tri tačke na aktivnosti (New Content Experience).
2. Podesite status na **Draft** (Classic) ili isključite **Visibility** (New Content Experience).

Draft teme su nevidljive studentima. Instruktori i asistenti i dalje ih vide sa oznakom "Draft".

Da ograničite temu na određenu grupu ili sekciju:

1. Otvorite temu.
2. Kliknite meni naslova teme > **Edit Properties In-place** (Classic) ili **Edit** > **Restrictions** (New Content Experience).
3. Pod **Release Conditions**, kliknite **Create**.
4. Izaberite **Group enrollment** ili **Section enrollment**, izaberite grupu/sekciju i sačuvajte.

Release conditions se slažu sa FastComments-ovim sopstvenim mapiranjem uloga. Studenti koji ne mogu da vide temu nemaju LTI launch.

#### What Students See on First Launch

Kada student klikne temu (ili učita HTML temu sa ugradnjom):

1. Brightspace obavlja LTI 1.3 launch u pozadini.
2. FastComments prima studentovo ime, email, avatar URL i LMS ulogu, i automatski ga prijavljuje. Nema FastComments prijavnog prompta.
3. Thread komentara za taj resource link se prikazuje unutar Brightspace iframe-a.

Mapiranje uloga pri launch-u:

- Brightspace `Administrator` postaje FastComments **admin** za thread (puni moderacioni pristup: brisanje, banovanje i pristup konfiguraciji).
- Brightspace `Instructor` postaje FastComments **moderator** (pinovanje, skrivanje, brisanje, banovanje).
- Sve druge uloge (`Learner`, `TeachingAssistant`, itd.) postaju standardni komentatori.

Komentari su pripisani studentovom Brightspace nalogu. Ako student izmeni ime ili avatar u Brightspace-u, sledeći LTI launch sinhronizuje promenu.

#### Iframe Height and Resize

FastComments emituje `org.imsglobal.lti.frameResize` postMessage pri svakom renderu threada i pri promenama sadržaja (novi komentar, proširenje odgovora). Brightspace sluša ovu poruku i podešava visinu iframe-a tako da thread nije obrezan i da se ne prikazuje unutrašnji scrollbar.

Ako iframe ostane na fiksnoj maloj visini:

- Potvrdite da je kurs učitan preko HTTPS-a. Brightspace-ov postMessage listener odbija mixed-content frame-ove.
- Potvrdite da nijedno browser proširenje ne blokira postMessage kanal.
- Za inline ugradnje u HTML temi, okolni HTML ne sme umotavati iframe u kontejner fiksne visine. Uklonite bilo koji inline `style="height: ..."` iz roditeljskog elementa.

#### Brightspace-Specific Gotchas

**Tool not showing in the Add Existing picker.** Deployment nije omogućen za org unit ovog kursa. Administrator treba da doda org unit (ili roditelja) na listu Org Units u deployment-u. Sama registracija alata nije dovoljna; deployment određuje koji kursevi vide alat.

**`deployment_id` mismatch on launch.** FastComments TOFU-pin-uje prvi `deployment_id` koji vidi za registraciju. Ako administrator obriše originalni deployment i kreira novi, launch-ovi iz novog deployment-a se odbijaju sa greškom o neusaglašenosti deployment-a. Rešenje je ponovo registrovati FastComments (generišite novi registration URL (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">get it here</a>) i pokrenite Dynamic Registration ponovo); stari zapis konfiguracije se zamenjuje.

**Tool launches but shows "Invalid LTI launch".** Kurs se nalazi u drugom tenant/org strukturi nego što deployment pokriva, ili je deployment onemogućen nakon registracije. Ponovo proverite **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments > **Enabled** prekidač i listu org unit-a deployment-a.

**Names and roles missing inside FastComments.** Brightspace šalje LTI launcheve sa Names and Role Provisioning Services (NRPS) claim-ovima. Ako je kurs nadograđen sa starijeg LTI 1.1 linka, launch može da nema `name` i `email` claim-ove. Ponovo dodajte FastComments temu preko **Add Existing** (ne migrirajte stari link) tako da launch koristi LTI 1.3.

**Embed shows a login screen instead of auto-SSO.** HTML tema je umetnuta kao običan `<iframe>` koji pokazuje na FastComments umesto preko **Insert Stuff** > **LTI Advantage**. Obični iframe-ovi preskaču LTI launch i vode korisnike na javnu FastComments stranicu. Obrišite iframe i ponovo umetnite kroz Insert Stuff tok.