Kad je FastComments registriran na platformi, instruktori ga dodaju u sadržaj kolegija koristeći standardne tijekove za vanjske alate platforme. Ova stranica obuhvaća Sakai 23.x i Schoology Enterprise.

#### Sakai

**1. Dodavanje FastComments na stranicu**

Održavatelj stranice omogućuje alat za svaku stranicu posebno:

1. Otvorite stranicu i u lijevoj navigaciji kliknite **Informacije o stranici**.
2. Kliknite **Upravljanje alatima**.
3. Pomaknite se do popisa **Vanjski alati** i uključite **FastComments**.
4. Kliknite **Nastavi**, pregledajte popis alata, zatim kliknite **Završi**.

FastComments se sada pojavljuje kao stavka u lijevoj navigaciji stranice.

**2. Promjena redoslijeda stavke u lijevoj navigaciji**

Idite na **Informacije o stranici** > **Redoslijed alata**. Povucite **FastComments** na željeno mjesto i kliknite **Spremi**. S ovog zaslona možete također preimenovati oznaku u navigaciji i sakriti je od studenata.

**3. Ugradnja unutar stranice Lessons**

Da biste postavili FastComments izravno unutar stranice Lessons, umjesto kao samostalan alat u lijevoj navigaciji:

1. Otvorite alat **Lessons** na stranici.
2. Kliknite **Dodaj sadržaj** > **Dodaj vanjski alat**.
3. Odaberite **FastComments** s popisa.
4. Ako je FastComments tijekom registracije oglasio Deep Linking, Sakai otvara selektor sadržaja alata tako da možete odabrati ili označiti thread. Ako Deep Linking nije oglašen, Sakai umeće zadanu poveznicu za pokretanje.
5. Spremite stavku Lessons.

Svaki ugrađeni primjer dobiva vlastiti thread, ograničen na tu resursnu poveznicu.

**4. Podešavanje dopuštenja za pristup studenata**

Sakai kontrolira pokretanja vanjskih alata preko Realmova. Da potvrdite da studenti mogu pokretati FastComments:

1. Prijavite se kao Sakai administrator i otvorite **Radni prostor administracije** > **Realms**.
2. Otvorite odgovarajući realm (na primjer, `!site.template.course` ili specifični realm stranice).
3. Potvrdite da je ulozi `access` omogućeno `lti.launch` i da su dozvole u grupi **external.tools** dodijeljene.
4. Spremite realm.

Za preklapanja na razini stranice, održavatelj može prilagoditi vidljivost alata po ulozi u **Informacije o stranici** > **Redoslijed alata** skrivajući ili pokazujući FastComments po ulozi.

**5. Što studenti vide**

Studenti kliknu stavku FastComments u lijevoj navigaciji (ili se pomaknu do ugrađenog bloka u Lessons) i dolaze izravno u prikaz threaded komentara. SSO je automatski: Sakai šalje identitet korisnika u LTI pokretanju i FastComments ih prijavljuje pod njihovim Sakai računom.

Mapiranje uloga:

- Sakai `Instructor` -> FastComments moderator
- Sakai `Admin` (admin u Radnom prostoru administracije) -> FastComments admin
- Sakai `Student` / `access` -> FastComments commenter

**6. Sakai napomene (gotchas)**

- **Alat nije vidljiv u Upravljanju alatima.** Ako se FastComments ne pojavljuje na popisu Vanjskih alata, Sakai administrator treba otvoriti registar alata (**Radni prostor administracije** > **Vanjski alati** > **FastComments**) i postaviti **Stealthed** na `false`. Stealthed alati su skriveni iz selektora Upravljanja alatima po stranici.
- **Pokretanja se zbog zajedničkih sesija u pregledniku vraćaju s greškom.** Portalni CSRF token Sakaija vezan je za sesiju preglednika. Ako je student prijavljen na dvije Sakai stranice u različitim karticama ili ima zastarjelu sesiju, pokretanje vraća 403. Rješenje: zatvorite druge Sakai kartice, odjavite se, ponovno se prijavite i pokrenite ponovno. Administratori također mogu povećati `sakai.csrf.token.cache.ttl` ako se ovo događa u cijelom klasteru.
- **Ugradnja u frame.** Potvrdite da je `lti.frameheight` u `sakai.properties` dovoljno velik (600 ili više) kako thread komentara ne bi bio odsječen unutar stranice Lessons.

#### Schoology

Schoology Enterprise ima dva scenarija instalacije. Potvrdite koji se primjenjuje prije dodavanja alata u kolegij.

**1. Dva scenarija instalacije**

- **(a) Instalacija na razini Enterprisea.** System Administrator Schoologyja instalirao je FastComments na razini organizacije i dodijelio ga svim kolegijima ili određenim predlošcima kolegija. Instruktori preskaču instalaciju i idu izravno na "Dodavanje materijala".
- **(b) Samostalna instalacija od strane instruktora.** Instruktor instalira alat u pojedinačni kolegij iz **Opcije kolegija** > **Vanjski alati** > **Instaliraj LTI aplikacije**. Samostalna instalacija zahtijeva da je System Administrator prethodno odobrio FastComments aplikaciju na razini organizacije.

**2. Dodavanje FastComments kao materijal kolegija**

Unutar kolegija:

1. Otvorite kolegij i idite na **Materijali**.
2. Kliknite **Dodaj materijale** > **Dodaj datoteku/poveznicu/vanjski alat**.
3. Odaberite **Vanjski alat**.
4. Odaberite **FastComments** s popisa registriranih alata.
5. Postavite **Naziv** (to je ono što studenti vide na popisu materijala) i opcionalno **Opis**.
6. Ostavite **Omogući ocjenjivanje** (grade passback) ISKLJUČENIM. FastComments ne prijavljuje ocjene natrag u Schoology, pa uključivanje prijenosa ocjena stvara praznu kolonu u dnevniku ocjena.
7. Kliknite **Pošalji**.

Materijal se sada pojavljuje na popisu materijala kolegija i otvara FastComments thread kada se klikne.

**3. Ugradnja unutar Rich Text urednika**

Ako je System Administrator tijekom registracije omogućio Deep Linking postavljanje za FastComments, instruktori mogu ugraditi thread komentara unutar bilo kojeg Rich Text polja (upute za zadatke, tijela stranica, poticaji za raspravu):

1. Otvorite Rich Text urednik na ciljanoj stranici.
2. Kliknite ikonu **Vanjski alat** (ikona slagalice) u alatnoj traci.
3. Odaberite **FastComments**.
4. Konfigurirajte ugradnju u dijaloškom prozoru za deep-linking i kliknite **Umetni**.
5. Spremite stranicu.

Ako gumb Vanjski alat ne postoji u Rich Text uredniku, Deep Linking je onemogućen za ovaj alat na ovom tenant-u. Pogledajte napomene u nastavku.

**4. Vidljivost i dodjela po sekcijama**

Schoology određuje dostupnost alata po sekciji kroz Opcije kolegija:

1. Iz kolegija kliknite **Opcije kolegija** > **Vanjski alati**.
2. Za svaku instaliranu LTI aplikaciju kontrolirate je li dostupna svim sekcijama u kolegiju ili samo određenim sekcijama.
3. Da biste ograničili FastComments na određene sekcije, poništite sekcije koje ne bi trebale vidjeti alat.
4. Pristup po sekcijama također upravlja koje sekcije vide stavku **Dodaj materijale** > **Vanjski alat** za FastComments.

**5. Što studenti vide**

Studenti kliknu FastComments materijal (ili se pomaknu do ugrađenog sadržaja) i dolaze u threaded raspravu. SSO je automatski putem Schoology LTI pokretanja pod njihovim Schoology računom.

Mapiranje uloga:

- Schoology `Administrator` -> FastComments admin
- Schoology `Instructor` -> FastComments moderator
- Schoology `Student` -> FastComments commenter

**6. Schoology napomene (gotchas)**

- **Samo za Enterprise.** Osobni i besplatni Schoology računi ne mogu instalirati LTI 1.3 alate. Ako je vaš tenant na besplatnom nivou, opcija **Vanjski alati** nije dostupna u Opcijama kolegija. Nadogradite na Schoology Enterprise da biste koristili FastComments.
- **Deep Linking onemogućen prema zadanim postavkama tenant-a.** Neki Schoology tenant-i ograničavaju Deep Linking postavljanje na razini organizacije. Kad je to slučaj, instruktori vide samo tijek **Dodaj materijale** > **Vanjski alat**, a ne gumb Vanjski alat u Rich Text uredniku. Da bi omogućili ugradnju unutar sadržaja, System Administrator ide na **Postavke sustava** > **Integracija** > **LTI 1.3** > **FastComments** i omogućuje postavljanje **Content Item / Deep Linking**, zatim sprema.
- **Preklapanje dodjele po sekcijama.** Ako je FastComments dodijeljen na razini enterprisea, ali instruktor ga ne vidi u **Dodaj materijale**, sekcija kolegija je isključena u dodjeli na razini organizacije. Zamolite System Administrator-a da doda sekciju u dodjelu FastComments aplikacije.
- **Naziv materijala naspram identiteta threada.** Preimenovanje materijala u Schoologyju ne premješta thread komentara. Threadovi su ključani na ID-u resursne poveznice LTI, pa preimenovanje zadržava isti thread; brisanje i ponovno stvaranje materijala stvara novi, prazan thread.