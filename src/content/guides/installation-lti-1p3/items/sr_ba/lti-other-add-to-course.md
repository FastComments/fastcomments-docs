Once FastComments je registrovan na platformi, instruktori ga dodaju u sadržaj kursa koristeći standardne tokove za eksterni alat platforme. Ova stranica pokriva Sakai 23.x i Schoology Enterprise.

#### Zaključajte javni pristup (Preporučeno)

Po defaultu, podaci komentara iz FastComments-a su javno čitljivi na obje platforme. Bilo ko ko može pogoditi URL niti ili API endpoint može vidjeti komentare, čak i izvan Sakai-a ili Schoology-a. Za kursne diskusije gotovo sigurno želite ograničiti pregled samo na upisane studente.

Otvorite svoju <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">stranicu za prilagođavanje widgeta</a> i kreirajte pravilo sa uključenom opcijom **Require SSO To View Comments**, zatim postavite nivo sigurnosti na **Secure SSO** tako da se niti mogu učitavati samo kroz potpisano LTI pokretanje.

Pogledajte [Protecting Comment Threads With Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) za kompletnu proceduru, uključujući kako ograničiti pravilo na jedan domen ili stranicu.

#### Sakai

**1. Dodajte FastComments na sajt**

Održavalac sajta omogućava alat na nivou pojedinačnog sajta:

1. Otvorite sajt i kliknite **Site Info** u lijevoj navigaciji.
2. Kliknite **Manage Tools**.
3. Skrolajte do liste **External Tools** i uključite **FastComments**.
4. Kliknite **Continue**, pregledajte listu alata, zatim kliknite **Finish**.

FastComments se sada pojavljuje kao stavka u lijevoj navigaciji sajta.

**2. Promijenite redoslijed stavke u lijevoj navigaciji**

Idite na **Site Info** > **Tool Order**. Prevucite **FastComments** na željenu poziciju i kliknite **Save**. Također možete promijeniti oznaku u navigaciji i sakriti je od studenata na ovom ekranu.

**3. Umetnite direktno u stranicu Lekcija**

Da biste postavili FastComments direktno unutar stranice Lekcija umjesto kao zaseban alat u lijevoj navigaciji:

1. Otvorite alat **Lessons** na sajtu.
2. Kliknite **Add Content** > **Add External Tool**.
3. Odaberite **FastComments** sa liste.
4. Ako je FastComments oglasio Deep Linking tokom registracije, Sakai otvara selektor sadržaja alata tako da možete izabrati ili označiti nit. Ako Deep Linking nije bio oglašen, Sakai umeće podrazumijevani link za pokretanje.
5. Sačuvajte stavku Lekcija.

Svaka ugrađena instanca dobija svoju nit, ograničenu na taj resursni link.

**4. Podešavanja dozvola za pristup studenata**

Sakai kontroliše pokretanja eksternih alata kroz Realms. Da potvrdite da studenti mogu pokretati FastComments:

1. Prijavite se kao Sakai admin i otvorite **Administration Workspace** > **Realms**.
2. Otvorite relevantan realm (na primjer, `!site.template.course` ili specifični realm sajta).
3. Potvrdite da uloga `access` ima omogućeno `lti.launch` i da su dozvole za uloge u grupi **external.tools** odobrene.
4. Sačuvajte realm.

Za nadjačavanja na nivou sajta, održavalac može prilagoditi vidljivost alata po ulozi iz **Site Info** > **Tool Order** sakrivanjem ili prikazivanjem FastComments-a po ulozi.

**5. Šta studenti vide**

Studenti kliknu stavku FastComments u lijevoj navigaciji (ili skrolaju do ugrađenog bloka u Lekcijama) i direktno dolaze u prikaz niti komentara. SSO je automatsko: Sakai šalje identitet korisnika u LTI pokretanju i FastComments ih prijavljuje pod njihovim Sakai nalogom.

Mapiranje uloga:

- Sakai `Instructor` -> FastComments moderator
- Sakai `Admin` (admin u Administration Workspace) -> FastComments admin
- Sakai `Student` / `access` -> FastComments commenter

**6. Uobičajeni problemi sa Sakai-jem**

- **Alat nije vidljiv u Manage Tools.** Ako se FastComments ne pojavljuje u listi External Tools, Sakai admin treba otvoriti registar alata (**Administration Workspace** > **External Tools** > **FastComments**) i postaviti **Stealthed** na `false`. Stealthed alati su skriveni iz izbora Manage Tools po sajtu.
- **Pokretanja se prekidaju u browserima sa dijeljenom sesijom.** Sakai-jev portal CSRF token je vezan za browser sesiju. Ako je student prijavljen na dva Sakai sajta u različitim tabovima ili ima zastarjelu sesiju, pokretanje vraća 403. Rješenje: zatvorite druge Sakai tabove, odjavite se, ponovo se prijavite i pokrenite ponovno. Admini također mogu povećati `sakai.csrf.token.cache.ttl` ako se ovo dešava na nivou klastera.
- **Ugradnja u frame.** Potvrdite da je `lti.frameheight` u `sakai.properties` dovoljno velik (600 ili više) kako se nit komentara ne bi obrezala unutar stranice Lekcija.

#### Schoology

Schoology Enterprise ima dva scenarija instalacije. Potvrdite koji se primjenjuje prije dodavanja alata u kurs.

**1. Dva scenarija instalacije**

- **(a) Instalacija na nivou preduzeća.** Schoology sistem administrator je instalirao FastComments na nivou organizacije i dodelio ga svim kursevima ili određenim šablonima kurseva. Instruktori preskaču instalaciju i idu direktno na "Add Materials".
- **(b) Samoinstalacija instruktora.** Instruktor instalira alat u pojedinačni kurs iz **Course Options** > **External Tools** > **Install LTI Apps**. Samoinstalacija zahtijeva da je sistem administrator prethodno odobrio FastComments aplikaciju na nivou organizacije.

**2. Dodajte FastComments kao materijal kursa**

Unutar kursa:

1. Otvorite kurs i idite na **Materials**.
2. Kliknite **Add Materials** > **Add File/Link/External Tool**.
3. Izaberite **External Tool**.
4. Odaberite **FastComments** iz liste registrovanih alata.
5. Postavite **Name** (ovo je ono što studenti vide na listi materijala) i opcioni **Description**.
6. Ostavite **Enable Grading** (grade passback) ISKLJUČENO. FastComments ne vraća ocjene nazad u Schoology, pa uključivanje povratka ocjena kreira praznu kolonu u ocjenjivaču.
7. Kliknite **Submit**.

Materijal se sada pojavljuje na listi materijala kursa i otvara FastComments nit kada se klikne.

**3. Inline ugradnja putem Rich Text editora**

Ako je sistem administrator omogućio Deep Linking postavljanje za FastComments tokom registracije, instruktori mogu ugraditi nit komentara u bilo koje polje Rich Text editora (uputstva za zadatak, tijelo stranice, prompti za diskusiju):

1. Otvorite Rich Text editor na ciljnoj stranici.
2. Kliknite ikonu **External Tool** (komad puzzle) u alatnoj traci.
3. Izaberite **FastComments**.
4. Konfigurišite ugradnju u deep-linking dijalogu i kliknite **Insert**.
5. Sačuvajte stranicu.

Ako dugme External Tool ne pojavljuje u Rich Text editoru, Deep Linking je onemogućen za ovaj alat na ovom tenant-u. Pogledajte napomene ispod.

**4. Vidljivost i dodjela sekcija**

Schoology ograničava dostupnost alata po sekcijama kroz Course Options:

1. Iz kursa kliknite **Course Options** > **External Tools**.
2. Za svaku instaliranu LTI aplikaciju kontrolišete da li je dostupna svim sekcijama u kursu ili samo određenim sekcijama.
3. Da ograničite FastComments na određene sekcije, poništite izbor sekcija koje ne bi trebale vidjeti alat.
4. Pristup na nivou sekcije takođe kontroliše koje sekcije vide unos **Add Materials** > **External Tool** za FastComments.

**5. Šta studenti vide**

Studenti kliknu materijal FastComments (ili skrolaju do inline ugradnje) i dolaze u nit diskusije. SSO je automatsko putem Schoology LTI pokretanja pod njihovim Schoology nalogom.

Mapiranje uloga:

- Schoology `Administrator` -> FastComments admin
- Schoology `Instructor` -> FastComments moderator
- Schoology `Student` -> FastComments commenter

**6. Schoology napomene**

- **Samo za Enterprise.** Personalni i besplatni Schoology nalozi ne mogu instalirati LTI 1.3 alate. Ako je vaš tenant na besplatnom nivou, opcija **External Tools** nedostaje u Course Options. Nadogradite na Schoology Enterprise da biste koristili FastComments.
- **Deep Linking onemogućen po defaultu na tenant-u.** Neki Schoology tenant-i ograničavaju Deep Linking postavljanje na nivou organizacije. Kada je to slučaj, instruktori vide samo tok **Add Materials** > **External Tool**, a ne dugme External Tool u Rich Text editoru. Da biste omogućili inline ugradnju, sistem administrator treba otići na **System Settings** > **Integration** > **LTI 1.3** > **FastComments** i omogućiti postavljanje **Content Item / Deep Linking**, zatim sačuvati.
- **Nadjačavanje dodele po sekcijama.** Ako je FastComments dodeljen na nivou preduzeća ali instruktor ne može da ga vidi u **Add Materials**, sekcija kursa je izuzeta u dodeli na nivou organizacije. Zamolite sistem administratora da doda sekciju u dodelu FastComments aplikacije.
- **Ime materijala naspram identiteta niti.** Preimenovanje materijala u Schoology-u ne premješta nit komentara. Niti su ključane na LTI resource link ID-u, tako da preimenovanje zadržava istu nit; brisanje i ponovno kreiranje materijala stvara novu, praznu nit.