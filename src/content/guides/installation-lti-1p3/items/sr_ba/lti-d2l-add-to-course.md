Ova stranica objašnjava dodavanje FastComments u Brightspace kurs nakon što je administrator registrovao alat i kreirao deployment. Ako alat još nije registrovan, prvo pogledajte vodič za registraciju D2L.

<div class="screenshot white-bg">
    <div class="title">FastComments ugrađen kao topic jedinice u Brightspace</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-d2l-comments-in-unit.png" alt="FastComments radi unutar Brightspace jedinice, prikazujući threaded komentare i birač za @-pominjanje" />
</div>

Brightspace nudi dva iskustva za izradu sadržaja: **Klasični sadržaj** i **Novo iskustvo sadržaja** (također nazvano **Lekcije**). Oba omogućavaju FastComments, ali putanje u menijima se razlikuju. Svaki dio ispod pokriva oba gdje se razlikuju.

#### Pronađite FastComments alat

FastComments alat se pojavljuje na dva mjesta unutar editor-a sadržaja kursa:

1. Picker aktivnosti, dostupan iz modula/jedinice preko dugmeta **Dodaj postojeće** (u starijim verzijama Brightspace označeno **Add Existing Activities**). FastComments se pojavljuje direktno u picker-u u aktuelnim verzijama Brightspace; starije verzije ga postavljaju pod podmenijem **External Learning Tools**. Bilo koja od ovih putanja dodaje FastComments kao samostalni topic.
2. Dijalog **Insert Stuff** unutar HTML editora, pod **LTI Advantage**. Ovo ugrađuje FastComments inline u HTML topic putem LTI deep linking toka.

Ako se FastComments ne pojavljuje ni u jednom picker-u, deployment nije omogućen za organizacionu jedinicu koja drži kurs. Zamolite vašeg Brightspace administratora da otvori **Alati administratora** > **Upravljanje proširenjima** > **LTI Advantage** > FastComments alat > **Prikaži implementacije**, otvori deployment i doda org jedinicu kursa (ili roditeljsku org jedinicu) pod **Organizacione jedinice**.

#### Dodavanje FastComments kao topica u modul

Klasični sadržaj:

1. Otvorite kurs i kliknite **Sadržaj** u navigacionom baru.
2. Izaberite modul koji treba da sadrži diskusiju (ili kreirajte jedan preko **Dodaj modul**).
3. Kliknite **Dodaj postojeće** (stariji Brightspace: **Add Existing Activities** > **External Learning Tools**).
4. U picker-u kliknite **FastComments**. Brightspace kreira topic u modulu i vraća vas na prikaz sadržaja.
5. Kliknite novi topic. Preimenujte ga u nešto opisno poput `FastComments Discussion` koristeći inline editor za naslov.

Novo iskustvo sadržaja (Lekcije):

1. Otvorite kurs i kliknite **Sadržaj**.
2. Otvorite jedinicu i lekciju koja treba da sadrži diskusiju.
3. Kliknite **Dodaj** > **Postojeća aktivnost** i izaberite **FastComments** (stariji Brightspace: ugnježdeno pod **External Learning Tools**).
4. Aktivnost se dodaje u lekciju.
5. Kliknite na naslov aktivnosti da bi ste je preimenovali.

Prvi put kada bilo koji korisnik (instruktor ili student) otvori topic, FastComments inicijalizuje thread za taj resource link. Thread je vezan za resource link ID, tako da preimenovanje ili premještanje topica ne mijenja koji thread se učitava.

#### Ugradite FastComments inline u HTML topic

Koristite ovaj tok kada želite da komentari budu prikazani ispod teksta za čitanje, videa ili drugog sadržaja unutar iste stranice topica, umjesto kao zaseban topic.

1. Otvorite ili kreirajte HTML topic u modulu/lekciji.
2. Kliknite **Uredi HTML** da otvorite Brightspace HTML editor.
3. Postavite kursor na mjesto gdje treba da se pojavi thread komentara.
4. Kliknite dugme **Insert Stuff** (ikona puzzle komada u toolbar-u editora).
5. U Insert Stuff dijalogu, skrolajte do **LTI Advantage** i kliknite **FastComments**.
6. FastComments otvara deep linking picker. Potvrdite pozicioniranje (default opcije rade za diskusije o sadržaju); kliknite **Ubaci** ili **Nastavi**.
7. Brightspace se vraća u HTML editor sa placeholder blokom koji predstavlja LTI launch. Kliknite **Sačuvaj i zatvori** na topicu.

Kada se topic učita, Brightspace zamjenjuje placeholder iframe-om koji automatski pokreće FastComments putem LTI. Studenti vide thread diskusije inline.

Jedan HTML topic može sadržavati više deep-linked FastComments ugradnji. Svaka ugradnja dobija svoj thread jer svaki deep link proizvodi jedinstven resource link ID.

#### Topic u modulu nasuprot inline quicklink-u

Odaberite pristup **topic u modulu** kada:

- Diskusija je primarna aktivnost za taj korak u modulu.
- Želite da topic bude vidljiv u tabeli sadržaja Brightspace-a, praćenju završetka i Class Progress.

Odaberite pristup **inline ugradnje** kada:

- Komentari trebaju biti ispod drugog sadržaja na istoj stranici.
- Ne želite poseban item koji se prati u tabeli sadržaja.

#### Vidljivost, Draft i uslovi objave

Novi FastComments topic je po defaultu vidljiv studentima. Da biste ga sakrili dok ga podešavate:

1. U editoru sadržaja, kliknite naslov topica (Klasični) ili meni sa tri tačke na aktivnosti (Novo iskustvo sadržaja).
2. Postavite status na **Nacrt** (Klasični) ili isključite **Vidljivost** (Novo iskustvo sadržaja).

Nacrti su nevidljivi studentima. Instruktori i TA-i ih i dalje vide sa oznakom "Nacrt".

Da ograničite topic na specifičnu grupu ili sekciju:

1. Otvorite topic.
2. Kliknite meni naslova topica > **Uredi svojstva na licu mjesta** (Klasični) ili **Uredi** > **Ograničenja** (Novo iskustvo sadržaja).
3. Pod **Uslovi objave**, kliknite **Kreiraj**.
4. Izaberite **Upis u grupu** ili **Upis u sekciju**, izaberite grupu/sekciju i sačuvajte.

Uslovi objave se slažu sa FastComments-ovim sopstvenim mapiranjem uloga. Studenti koji ne mogu vidjeti topic neće dobiti LTI launch.

#### Šta studenti vide pri prvom pokretanju

Kada student klikne topic (ili učita HTML topic sa ugradnjom):

1. Brightspace izvodi LTI 1.3 launch u pozadini.
2. FastComments prima studentovo ime, email, URL avatara i LMS ulogu, i automatski ih prijavljuje. Nema FastComments prompta za prijavu.
3. Thread komentara za taj resource link se prikazuje unutar Brightspace iframe-a.

Mapiranje uloga pri launch-u:

- Brightspace `Administrator` postaje FastComments **admin** za thread (puni pristup moderaciji, brisanje, ban i konfiguracija).
- Brightspace `Instructor` postaje FastComments **moderator** (pin, sakrij, obriši, ban).
- Sve druge uloge (`Learner`, `TeachingAssistant`, itd.) postaju standardni komentatori.

Komentari su pripisani studentovom Brightspace nalogu. Ako student izmijeni svoje ime ili avatar u Brightspace-u, sljedeći LTI launch sinhronizuje promjenu.

#### Visina iframe-a i promjena veličine

FastComments emituje `org.imsglobal.lti.frameResize` postMessage pri svakom renderu threada i pri promjeni sadržaja (novi komentar, proširi odgovore). Brightspace sluša ovu poruku i prilagođava visinu iframe-a tako da thread ne bude odrezan i da se ne pojavi unutrašnji scrollbar.

Ako iframe ostaje na fiksnoj maloj visini:

- Potvrdite da se kurs učitava preko HTTPS. Brightspace-ov postMessage listener odbija frame-ove sa miješanim sadržajem.
- Potvrdite da nijedno ekstenzijsko proširenje browser-a ne blokira postMessage kanal.
- Za inline ugradnje u HTML topic, okolni HTML ne smije obmotavati iframe u kontejner fiksne visine. Uklonite bilo koji inline `style="height: ..."` sa roditeljskog elementa.

#### Brightspace-specifične zamke

**Alat se ne prikazuje u picker-u Dodaj postojeće.** Deployment nije omogućen za org jedinicu ovog kursa. Administrator treba dodati org jedinicu (ili roditelja) na listu **Organizacionih jedinica** deployment-a. Sama registracija alata nije dovoljna; deployment određuje koji kursevi vide alat.

**`deployment_id` ne odgovara pri launch-u.** FastComments TOFU-pin-uje prvi `deployment_id` koji vidi za registraciju. Ako administrator obriše originalni deployment i kreira novi, launch-ovi sa novog deployment-a će biti odbijeni sa greškom o neusaglašenosti deployment-a. Rješenje je ponovo registrovati FastComments (generišite novi registration URL (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">preuzmite ga ovdje</a>) i ponovo pokrenite Dynamic Registration); stara konfiguracija se zamjenjuje.

**Alat se pokreće ali prikazuje "Invalid LTI launch".** Kurs se nalazi u drugačijem tenant/org strukturi nego što deployment pokriva, ili je deployment onemogućen nakon registracije. Ponovo provjerite **Alati administratora** > **Upravljanje proširenjima** > **LTI Advantage** > FastComments > prekidač **Omogućen** i listu org jedinica u deployment-u.

**Imena i uloge nedostaju unutar FastComments.** Brightspace šalje LTI launch-e sa Names and Role Provisioning Services (NRPS) claims. Ako je kurs unaprijeđen sa starijeg LTI 1.1 linka, launch može nedostajati `name` i `email` claims. Ponovo dodajte FastComments topic preko **Dodaj postojeće** (nemojte migrirati stari link) tako da launch koristi LTI 1.3.

**Ugradnja prikazuje ekran za prijavu umjesto automatskog SSO.** HTML topic je umetnut kao običan `<iframe>` koji pokazuje direktno na FastComments umjesto putem **Insert Stuff** > **LTI Advantage**. Obični iframe-ovi preskaču LTI launch i vode korisnike na javnu FastComments stranicu. Obrišite iframe i ponovo umetnite putem Insert Stuff toka.