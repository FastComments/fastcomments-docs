Ova stranica pokriva dodavanje FastComments-a u Brightspace kurs nakon što je administrator registrovao alat i kreirao deployment. Ako alat još nije registrovan, prvo pogledajte D2L vodič za registraciju.

<div class="screenshot white-bg">
    <div class="title">FastComments ugrađen kao tema jedinice u Brightspace-u</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-d2l-comments-in-unit.png" alt="FastComments radi unutar Brightspace jedinice, prikazujući razgranate komentare i @-mention birač" />
</div>

Brightspace nudi dva iskustva za kreiranje sadržaja: **Classic Content** i **New Content Experience** (takođe poznat kao **Lessons**). Oba izlažu FastComments, ali se putanje menija razlikuju. Svaka sekcija ispod pokriva obe varijante tamo gde se razlikuju.

#### Pronalaženje FastComments alata

FastComments alat se pojavljuje na dva mesta unutar editora sadržaja kursa:

1. Birač aktivnosti, do kojeg se dolazi preko dugmeta **Add Existing** modula/jedinice (označeno kao **Add Existing Activities** u starijim verzijama Brightspace-a). FastComments se direktno pojavljuje u biraču u trenutnim verzijama Brightspace-a; starije verzije ga ugnježđuju pod podmeni **External Learning Tools**. Bilo koja putanja dodaje FastComments kao samostalnu temu.
2. Dijalog **Insert Stuff** unutar HTML editora, pod **LTI Advantage**. Ovo ugrađuje FastComments inline u HTML temu putem LTI deep linking toka.

Ako se FastComments ne pojavljuje ni u jednom biraču, deployment nije omogućen za org jedinicu koja sadrži kurs. Zatražite od svog Brightspace administratora da otvori **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments alat > **View Deployments**, otvori deployment i doda org jedinicu kursa (ili nadređenu org jedinicu) pod **Org Units**.

#### Dodavanje FastComments kao teme u modulu

Classic Content:

1. Otvorite kurs i kliknite **Content** u navbar-u.
2. Izaberite modul koji treba da sadrži diskusiju (ili ga kreirajte preko **Add a module**).
3. Kliknite **Add Existing** (stariji Brightspace: **Add Existing Activities** > **External Learning Tools**).
4. U biraču kliknite **FastComments**. Brightspace kreira temu u modulu i vraća vas na prikaz sadržaja.
5. Kliknite na novu temu. Preimenujte je u nešto opisno poput `FastComments Discussion` koristeći inline editor naslova.

New Content Experience (Lessons):

1. Otvorite kurs i kliknite **Content**.
2. Otvorite jedinicu i lekciju koja treba da sadrži diskusiju.
3. Kliknite **Add** > **Existing Activity** i izaberite **FastComments** (stariji Brightspace: ugnježđeno pod **External Learning Tools**).
4. Aktivnost se dodaje u lekciju.
5. Kliknite na naslov aktivnosti da je preimenujete.

Kada bilo koji korisnik (instruktor ili student) prvi put otvori temu, FastComments inicijalizuje thread za taj resource link. Thread je vezan za ID resource linka, tako da preimenovanje ili premeštanje teme ne menja koji thread se učitava.

#### Inline ugrađivanje FastComments-a u HTML temu

Koristite ovaj tok kada želite da se komentari pojave ispod teksta, videa ili drugog sadržaja unutar iste stranice teme, umesto kao zasebna tema.

1. Otvorite ili kreirajte HTML temu u modulu/lekciji.
2. Kliknite **Edit HTML** da otvorite Brightspace HTML editor.
3. Postavite kursor tamo gde treba da se pojavi thread komentara.
4. Kliknite dugme **Insert Stuff** (ikona puzzle komada u traci alata editora).
5. U Insert Stuff dijalogu skrolujte do **LTI Advantage** i kliknite **FastComments**.
6. FastComments otvara deep linking birač. Potvrdite postavljanje (podrazumevane opcije rade za diskusije sadržaja); kliknite **Insert** ili **Continue**.
7. Brightspace se vraća u HTML editor sa placeholder blokom koji predstavlja LTI pokretanje. Kliknite **Save and Close** na temi.

Kada se tema učita, Brightspace zamenjuje placeholder iframe-om koji automatski pokreće FastComments putem LTI. Studenti vide thread diskusije inline.

Jedna HTML tema može sadržati više deep-linked FastComments ugrađivanja. Svako ugrađivanje dobija svoj thread jer svaki deep link proizvodi poseban ID resource linka.

#### Tema modula vs inline quicklink

Izaberite pristup **teme modula** kada:

- Diskusija je primarna aktivnost za taj korak u modulu.
- Želite da se tema pojavi u sadržaju kursa (table of contents), praćenju završenosti i Class Progress-u.

Izaberite pristup **inline ugrađivanja** kada:

- Komentari treba da stoje ispod drugog sadržaja na istoj stranici.
- Ne želite zasebnu stavku koja se prati za završenost u sadržaju kursa.

#### Vidljivost, Draft i uslovi objavljivanja

Nova FastComments tema je podrazumevano vidljiva studentima. Da je sakrijete dok je podešavate:

1. U editoru sadržaja kliknite na naslov teme (Classic) ili meni sa tri tačke na aktivnosti (New Content Experience).
2. Postavite status na **Draft** (Classic) ili isključite **Visibility** (New Content Experience).

Teme u Draft statusu su nevidljive studentima. Instruktori i asistenti i dalje vide sa oznakom "Draft".

Da ograničite temu na određenu grupu ili sekciju:

1. Otvorite temu.
2. Kliknite meni naslova teme > **Edit Properties In-place** (Classic) ili **Edit** > **Restrictions** (New Content Experience).
3. Pod **Release Conditions** kliknite **Create**.
4. Izaberite **Group enrollment** ili **Section enrollment**, odaberite grupu/sekciju i sačuvajte.

Uslovi objavljivanja se kombinuju sa sopstvenim mapiranjem uloga FastComments-a. Studenti koji ne mogu da vide temu ne dobijaju LTI pokretanje.

#### Šta studenti vide pri prvom pokretanju

Kada student klikne na temu (ili učita HTML temu sa ugrađivanjem):

1. Brightspace izvršava LTI 1.3 pokretanje u pozadini.
2. FastComments prima ime, email, URL avatara i LMS ulogu studenta i automatski ga prijavljuje. Nema FastComments ekrana za prijavu.
3. Thread komentara za taj resource link se renderuje unutar Brightspace iframe-a.

Mapiranje uloga pri pokretanju:

- Brightspace `Administrator` postaje FastComments **admin** za thread (puna kontrola moderacije, brisanje, ban i pristup konfiguraciji).
- Brightspace `Instructor` postaje FastComments **moderator** (pin, sakrivanje, brisanje, ban).
- Sve ostale uloge (`Learner`, `TeachingAssistant`, itd.) postaju standardni komentatori.

Komentari se pripisuju Brightspace nalogu studenta. Ako student izmeni svoje ime ili avatar u Brightspace-u, sledeće LTI pokretanje sinhronizuje promenu.

#### Ograničavanje javnog pristupa (preporučeno)

Podrazumevano, podaci o komentarima FastComments-a su javno čitljivi. Svako ko može da pogodi URL threada ili API endpoint može da vidi njegove komentare, čak i van Brightspace-a. Za diskusije na kursu gotovo sigurno želite da ograničite pregled samo na upisane studente.

Otvorite svoju <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">stranicu za prilagođavanje widget-a</a> i kreirajte pravilo sa omogućenom opcijom **Require SSO To View Comments**, zatim postavite bezbednosni nivo na **Secure SSO** tako da se threadovi mogu učitati samo kroz potpisano LTI pokretanje.

Pogledajte [Zaštita threadova komentara pomoću Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) za kompletan vodič, uključujući kako da ograničite pravilo na jedan domen ili stranicu.

#### Visina iframe-a i promena veličine

FastComments emituje `org.imsglobal.lti.frameResize` postMessage pri svakom renderovanju threada i pri promenama sadržaja (novi komentar, proširivanje odgovora). Brightspace sluša ovu poruku i prilagođava visinu iframe-a tako da thread nije odsečen i ne prikazuje unutrašnji scrollbar.

Ako iframe ostaje na fiksnoj maloj visini:

- Proverite da li se kurs učitava preko HTTPS-a. Brightspace postMessage listener odbija mixed-content frame-ove.
- Proverite da neki ekstenzija pregledača ne blokira postMessage kanal.
- Za inline ugrađivanja u HTML temi, okolni HTML ne sme ugnježđivati iframe u kontejner fiksne visine. Uklonite svaki inline `style="height: ..."` sa roditeljskog elementa.

#### Specifične zamke Brightspace-a

**Alat se ne prikazuje u Add Existing biraču.** Deployment nije omogućen za org jedinicu ovog kursa. Administrator treba da doda org jedinicu (ili nadređenu) na **Org Units** listu deployment-a. Sama registracija alata nije dovoljna; deployment određuje koji kursevi vide alat.

**`deployment_id` neusklađenost pri pokretanju.** FastComments TOFU-pinuje prvi `deployment_id` koji vidi za registraciju. Ako administrator obriše originalni deployment i kreira novi, pokretanja iz novog deployment-a se odbijaju sa greškom neusklađenosti deployment-a. Rešenje je ponovna registracija FastComments-a (generišite novi URL registracije (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">preuzmite ga ovde</a>) i ponovo pokrenite Dynamic Registration); stari zapis konfiguracije se zamenjuje.

**Alat se pokreće ali prikazuje "Invalid LTI launch".** Kurs je u drugoj tenant/org strukturi od one koju deployment pokriva, ili je deployment onemogućen nakon registracije. Ponovo proverite **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments > **Enabled** prekidač i org jedinice deployment-a.

**Imena i uloge nedostaju unutar FastComments-a.** Brightspace šalje LTI pokretanja sa Names and Role Provisioning Services (NRPS) claim-ovima. Ako je kurs nadograđen sa starijeg LTI 1.1 linka, pokretanje ne sadrži `name` i `email` claim-ove. Ponovo dodajte FastComments temu preko **Add Existing** (ne migrirajte stari link) tako da pokretanje koristi LTI 1.3.

**Ugrađivanje prikazuje ekran za prijavu umesto auto-SSO.** HTML tema je ubačena kao običan `<iframe>` koji pokazuje na FastComments umesto preko **Insert Stuff** > **LTI Advantage**. Obični iframe-ovi preskaču LTI pokretanje i vode korisnike na javnu FastComments stranicu. Obrišite iframe i ponovo ga ubacite putem Insert Stuff toka.