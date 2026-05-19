Ova stranica objašnjava kako dodati FastComments u Brightspace kurs nakon što je administrator registrovao alat i kreirao deployment. Ako alat još nije registrovan, prvo pogledajte D2L registration guide.

<div class="screenshot white-bg">
    <div class="title">FastComments ugrađen kao tema jedinice u Brightspace</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-d2l-comments-in-unit.png" alt="FastComments radi unutar Brightspace jedinice, prikazujući thread-ovane komentare i selektor za @-pominjanje" />
</div>

Brightspace nudi dva iskustva za kreiranje sadržaja: **Classic Content** i **New Content Experience** (takođe nazvan **Lessons**). Oba omogućavaju FastComments, ali putanje kroz menije se razlikuju. Svaki odeljak ispod pokriva oba slučaja gde se razlikuju.

#### Locate the FastComments Tool

FastComments alat se pojavljuje na dva mesta unutar editora sadržaja kursa:

1. Activity picker, dostupan iz modula/jedinice preko dugmeta **Add Existing** (u starijim verzijama Brightspace označeno kao **Add Existing Activities**). U aktuelnim verzijama Brightspace, FastComments se pojavljuje direktno u pickeru; u starijim verzijama je ugnježden pod podmenijem **External Learning Tools**. Bilo koji od ova dva puta dodaje FastComments kao zasebnu temu.
2. **Insert Stuff** dijalog unutar HTML editora, pod **LTI Advantage**. Ovo ugrađuje FastComments inline u HTML temu preko LTI deep linking procesa.

Ako se FastComments ne pojavljuje ni u jednom pickeru, deployment nije omogućen za org unit koji sadrži kurs. Zamolite Brightspace administratora da otvori **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments tool > **View Deployments**, otvori deployment i doda org unit kursa (ili roditeljski org unit) pod **Org Units**.

#### Add FastComments as a Topic in a Module

Classic Content:

1. Otvorite kurs i kliknite **Content** u navigacionom baru.
2. Izaberite modul koji treba da sadrži diskusiju (ili kreirajte jedan preko **Add a module**).
3. Kliknite **Add Existing** (stariji Brightspace: **Add Existing Activities** > **External Learning Tools**).
4. U pickeru kliknite **FastComments**. Brightspace kreira temu u modulu i vraća vas u prikaz sadržaja.
5. Kliknite novu temu. Preimenujte je u nešto opisno kao `FastComments Discussion` koristeći inline title editor.

New Content Experience (Lessons):

1. Otvorite kurs i kliknite **Content**.
2. Otvorite jedinicu i lesson koja treba da sadrži diskusiju.
3. Kliknite **Add** > **Existing Activity** i izaberite **FastComments** (stariji Brightspace: ugnježdeno pod **External Learning Tools**).
4. Aktivnost je dodata u lesson.
5. Kliknite na naslov aktivnosti da biste je preimenovali.

Prvi put kada bilo koji korisnik (instruktor ili student) otvori temu, FastComments inicijalizuje thread za taj resource link. Thread je vezan za resource link ID, tako da preimenovanje ili premještanje teme ne menja koji se thread učitava.

#### Embed FastComments Inline in an HTML Topic

Koristite ovaj tok kada želite da komentari budu prikazani ispod teksta za čitanje, videa ili drugog sadržaja unutar iste stranice teme, umesto kao zasebna tema.

1. Otvorite ili kreirajte HTML temu u modulu/lesson-u.
2. Kliknite **Edit HTML** da otvorite Brightspace HTML editor.
3. Postavite kursor na mesto gde treba da se pojavi thread komentara.
4. Kliknite dugme **Insert Stuff** (ikona slagalice u toolbar-u editora).
5. U Insert Stuff dijalogu, skrolujte do **LTI Advantage** i kliknite **FastComments**.
6. FastComments otvara deep linking picker. Potvrdite pozicioniranje (podrazumevana podešavanja rade za sadržajne diskusije); kliknite **Insert** ili **Continue**.
7. Brightspace se vraća u HTML editor sa placeholder blokom koji predstavlja LTI launch. Kliknite **Save and Close** na temi.

Kada se tema učita, Brightspace zamenjuje placeholder iframe-om koji automatski pokreće FastComments putem LTI. Studenti vide thread diskusije inline.

Jedna HTML tema može sadržavati više deep-linked FastComments embed-ova. Svaki embed dobija sopstveni thread jer svaki deep link proizvodi jedinstven resource link ID.

#### Module Topic vs Inline Quicklink

Izaberite pristup **module topic** kada:

- Diskusija predstavlja glavnu aktivnost za taj korak u modulu.
- Želite da tema bude vidljiva u Brightspace tabeli sadržaja, praćenju završetka i Class Progress.

Izaberite pristup **inline embed** kada:

- Komentari treba da se nalaze ispod drugog sadržaja na istoj stranici.
- Ne želite zaseban element koji se prati u tabeli sadržaja za praćenje završetka.

#### Visibility, Draft, and Release Conditions

Nova FastComments tema je podrazumevano vidljiva studentima. Da je sakrijete dok je podešavate:

1. U editoru sadržaja kliknite naslov teme (Classic) ili meni sa tri tačke na aktivnosti (New Content Experience).
2. Podesite status na **Draft** (Classic) ili isključite **Visibility** (New Content Experience).

Draft teme su nevidljive studentima. Instruktori i TA i dalje ih vide sa "Draft" značkom.

Da ograničite temu na određenu grupu ili sekciju:

1. Otvorite temu.
2. Kliknite meni naslova teme > **Edit Properties In-place** (Classic) ili **Edit** > **Restrictions** (New Content Experience).
3. Pod **Release Conditions**, kliknite **Create**.
4. Izaberite **Group enrollment** ili **Section enrollment**, odaberite grupu/sekciju i sačuvajte.

Release conditions se nadovezuju na FastComments sopstveni role mapping. Studenti koji ne mogu da vide temu neće dobiti LTI launch.

#### What Students See on First Launch

Kada student klikne temu (ili učita HTML temu sa embed-om):

1. Brightspace izvršava LTI 1.3 launch u pozadini.
2. FastComments prima studentovo ime, e‑mail, URL avatara i LMS ulogu, i automatski ga prijavljuje. Nema FastComments zahteva za prijavu.
3. Thread komentara za taj resource link se prikazuje unutar Brightspace iframe-a.

Role mapping pri launch-u:

- Brightspace `Administrator` postaje FastComments **admin** za thread (puna moderacija, brisanje, ban i pristup konfiguraciji).
- Brightspace `Instructor` postaje FastComments **moderator** (pin, hide, delete, ban).
- Sve ostale uloge (`Learner`, `TeachingAssistant`, itd.) postaju standardni komentatori.

Komentari su pripisani studentovom Brightspace nalogu. Ako student izmeni svoje ime ili avatar u Brightspace-u, sledeći LTI launch sinhronizuje promenu.

#### Lock Down Public Access (Recommended)

Po podrazumevanju, FastComments podaci komentara su javno čitljivi. Bilo ko ko pogodi URL thread-a ili API endpoint može videti komentare, čak i izvan Brightspace-a. Za diskusije u kursevima gotovo sigurno želite ograničiti pregled samo na upisane polaznike.

Otvorite svoju <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">stranicu za prilagođavanje widget-a</a> i kreirajte pravilo sa omogućenom opcijom **Require SSO To View Comments**, zatim postavite nivo bezbednosti na **Secure SSO** tako da se thread-ovi mogu učitavati samo preko potpisanog LTI launcha.

Pogledajte [Protecting Comment Threads With Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) za kompletan vodič, uključujući kako da ograničite pravilo na jedan domen ili stranicu.

#### Iframe Height and Resize

FastComments emituje `org.imsglobal.lti.frameResize` postMessage pri svakom renderovanju threada i pri promenama sadržaja (novi komentar, proširi odgovore). Brightspace osluškuje ovu poruku i prilagođava visinu iframe-a tako da thread nije isečen i da se ne prikazuje unutrašnji scrollbar.

Ako iframe ostaje fiksno niske visine:

- Potvrdite da je kurs učitan preko HTTPS-a. Brightspace-ov postMessage listener odbacuje mixed-content frame-ove.
- Potvrdite da nijedno proširenje pregledača ne blokira postMessage kanal.
- Za inline embed-ove u HTML temi, okolni HTML ne sme da omota iframe unutar kontejnera fiksne visine. Uklonite bilo koji inline `style="height: ..."` sa roditeljskog elementa.

#### Brightspace-Specific Gotchas

**Tool not showing in the Add Existing picker.** Deployment nije omogućen za org unit ovog kursa. Administrator treba da doda org unit (ili roditelja) u deployment-ovu listu **Org Units**. Sama registracija alata nije dovoljna; deployment određuje koji kursevi vide alat.

**`deployment_id` mismatch on launch.** FastComments TOFU-zakucava prvi `deployment_id` koji vidi za registraciju. Ako administrator obriše originalni deployment i kreira novi, launch-ovi sa novog deployment-a će biti odbijeni sa greškom neslaganja deployment-a. Rešenje je ponovo registrovati FastComments (generišite novu registration URL (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">get it here</a>) i ponovo pokrenite Dynamic Registration); stari zapis konfiguracije će biti zamenjen.

**Tool launches but shows "Invalid LTI launch".** Kurs se nalazi u drugoj tenant/organizacijskoj strukturi nego što deployment pokriva, ili je deployment onemogućen nakon registracije. Ponovo proverite **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments > **Enabled** prekidač i listu org unit-a za deployment.

**Names and roles missing inside FastComments.** Brightspace šalje LTI launcheve sa Names and Role Provisioning Services (NRPS) claim-ovima. Ako je kurs nadograđen sa starijeg LTI 1.1 linka, launch može da nema `name` i `email` claim-ove. Ponovo dodajte FastComments temu preko **Add Existing** (nemojte migrirati stari link) tako da launch koristi LTI 1.3.

**Embed shows a login screen instead of auto-SSO.** HTML tema je ubačena kao plain `<iframe>` koji pokazuje direktno na FastComments umesto putem **Insert Stuff** > **LTI Advantage**. Plain iframe-ovi preskaču LTI launch i korisnici dospeju na javnu FastComments stranicu. Obrišite taj iframe i ponovo ubacite putem Insert Stuff toka.