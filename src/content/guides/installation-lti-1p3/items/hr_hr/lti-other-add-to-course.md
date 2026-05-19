Nakon što je FastComments registriran na platformi, instruktori ga dodaju u sadržaj tečaja koristeći standardne tokove platforme za vanjske alate. Ova stranica pokriva Sakai 23.x i Schoology Enterprise.

#### Onemogućite javni pristup (Preporučeno)

Po zadanim postavkama, podaci komentara FastComments-a su javno čitljivi na obje platforme. Bilo tko tko može pogoditi URL niti ili API endpoint može vidjeti komentare, čak i izvan Sakai-a ili Schoology-a. Za rasprave u tečaju gotovo sigurno želite ograničiti pregled samo na upisane studente.

Otvorite svoju <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">stranicu za prilagodbu widgeta</a> i stvorite pravilo s omogućenom opcijom **Require SSO To View Comments**, zatim postavite razinu sigurnosti na **Secure SSO** tako da se niti mogu učitavati samo kroz potpisani LTI launch.

Pogledajte [Protecting Comment Threads With Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) za potpun vodič, uključujući kako opseg pravila ograničiti na jednu domenu ili stranicu.

#### Sakai

**1. Dodajte FastComments na lokaciju**

Održavatelj lokacije omogućuje alat na razini pojedine lokacije:

1. Otvorite lokaciju i kliknite **Site Info** u lijevom navigacijskom izborniku.
2. Kliknite **Manage Tools**.
3. Pomaknite se do popisa **External Tools** i uključite **FastComments**.
4. Kliknite **Continue**, pregledajte popis alata, zatim kliknite **Finish**.

FastComments se sada pojavljuje kao stavka u lijevom izborniku na lokaciji.

**2. Promijenite redoslijed stavke u lijevom izborniku**

Idite na **Site Info** > **Tool Order**. Povucite **FastComments** na željenu poziciju i kliknite **Save**. Također možete preimenovati naljepnicu izbornika i sakriti je od studenata s ovog zaslona.

**3. Ugradnja inline u Lessons stranicu**

Da biste postavili FastComments izravno unutar Lessons stranice umjesto kao samostalni alat u lijevom izborniku:

1. Otvorite alat **Lessons** na lokaciji.
2. Kliknite **Add Content** > **Add External Tool**.
3. Odaberite **FastComments** s popisa.
4. Ako je FastComments prilikom registracije oglašavao Deep Linking, Sakai otvara selektor sadržaja alata tako da možete odabrati ili označiti nit. Ako Deep Linking nije bio oglašen, Sakai umeće zadanu launch poveznicu.
5. Spremite stavku Lessons.

Svaka ugrađena instanca dobiva svoju nit, ograničenu na taj resursni link.

**4. Podešavanja dozvola za pristup studentima**

Sakai kontrolira pokretanja vanjskih alata kroz Realms. Da potvrdite da studenti mogu pokrenuti FastComments:

1. Prijavite se kao Sakai admin i otvorite **Administration Workspace** > **Realms**.
2. Otvorite odgovarajući realm (na primjer, `!site.template.course` ili specifični realm lokacije).
3. Potvrdite da `access` uloga ima omogućeno `lti.launch` i da su dopuštenja u grupi **external.tools** dodijeljena.
4. Spremite realm.

Za nadjačavanja na razini lokacije, održavatelj može prilagoditi vidljivost alata po ulozi iz **Site Info** > **Tool Order** skrivajući ili prikazujući FastComments po ulozi.

**5. Što studenti vide**

Studenti kliknu stavku FastComments u lijevom izborniku (ili se pomaknu do ugrađenog bloka u Lessons) i izravno dolaze u prikaz niti s komentarima. SSO je automatski: Sakai šalje identitet korisnika u LTI launch i FastComments ih prijavljuje pod njihovim Sakai računom.

Preslikavanje uloga:

- Sakai `Instructor` -> FastComments moderator
- Sakai `Admin` (admin u Administration Workspace) -> FastComments admin
- Sakai `Student` / `access` -> FastComments commenter

**6. Sakai zamke**

- **Alat nije vidljiv u Manage Tools.** Ako se FastComments ne pojavljuje u popisu External Tools, Sakai admin treba otvoriti registar alata (**Administration Workspace** > **External Tools** > **FastComments**) i postaviti **Stealthed** na `false`. Stealthed alati su skriveni iz birača Manage Tools po lokaciji.
- **Pokretanja se ruše u preglednicima sa zajedničkom sesijom.** Sakai-jev portal CSRF token vezan je uz sesiju preglednika. Ako je student prijavljen u dvije Sakai lokacije u različitim karticama ili ima zastarjelu sesiju, launch vraća 403. Rješenje: zatvorite druge Sakai kartice, odjavite se, ponovo se prijavite i ponovno pokrenite. Admini također mogu povećati `sakai.csrf.token.cache.ttl` ako se ovo događa na cijelom klasteru.
- **Ugradnja u iframe.** Potvrdite da je `lti.frameheight` u `sakai.properties` dovoljno velik (600 ili više) kako nit komentara ne bi bila odrezana unutar Lessons stranice.

#### Schoology

Schoology Enterprise ima dva scenarija instalacije. Potvrdite koji se primjenjuje prije dodavanja alata u tečaj.

**1. Dva scenarija instalacije**

- **(a) Instalacija na razini organizacije.** System Administrator Schoology-a instalirao je FastComments na razini organizacije i dodijelio ga svim tečajevima ili određenim predlošcima tečajeva. Instruktori preskaču instalaciju i idu izravno na "Add Materials".
- **(b) Instruktor samoinstalacija.** Instruktor instalira alat u pojedinačni tečaj iz **Course Options** > **External Tools** > **Install LTI Apps**. Samoinstalacija zahtijeva da je System Administrator prethodno odobrio FastComments aplikaciju na razini organizacije.

**2. Dodajte FastComments kao materijal tečaja**

Unutar tečaja:

1. Otvorite tečaj i idite na **Materials**.
2. Kliknite **Add Materials** > **Add File/Link/External Tool**.
3. Odaberite **External Tool**.
4. Odaberite **FastComments** s popisa registriranih alata.
5. Postavite **Name** (to je ono što studenti vide na popisu materijala) i opcionalni **Description**.
6. Ostavite **Enable Grading** (grade passback) **OFF**. FastComments ne vraća ocjene u Schoology, pa uključivanje povratka ocjena stvara prazan stupac ocjenika.
7. Kliknite **Submit**.

Materijal se sada pojavljuje na popisu materijala tečaja i otvara FastComments nit kada se klikne.

**3. Inline ugradnja putem Rich Text editora**

Ako je System Administrator omogućio Deep Linking placement za FastComments tijekom registracije, instruktori mogu ugraditi nit komentara unutar bilo kojeg polja Rich Text editora (upute za zadatak, tijela stranica, poticaji za raspravu):

1. Otvorite Rich Text editor na ciljnoj stranici.
2. Kliknite ikonu **External Tool** (komadić slagalice) u alatnoj traci.
3. Odaberite **FastComments**.
4. Konfigurirajte ugradnju u dijalogu za deep-linking i kliknite **Insert**.
5. Spremite stranicu.

Ako gumb External Tool ne pojavljuje u Rich Text editoru, Deep Linking je onemogućen za ovaj alat na ovom tenant-u. Pogledajte zamke u nastavku.

**4. Vidljivost i dodjela po sekcijama**

Schoology ograničava dostupnost alata po sekciji kroz Course Options:

1. Iz tečaja kliknite **Course Options** > **External Tools**.
2. Za svaku instaliranu LTI aplikaciju kontrolirate je li dostupna svim sekcijama u tečaju ili samo određenim sekcijama.
3. Da biste ograničili FastComments određenim sekcijama, poništite odabir sekcija koje ne bi trebale vidjeti alat.
4. Pristup po sekcijama također kontrolira koje sekcije vide unos **Add Materials** > **External Tool** za FastComments.

**5. Što studenti vide**

Studenti kliknu FastComments materijal (ili se pomaknu do inline ugradnje) i dolaze u nit rasprave. SSO je automatski putem Schoology LTI launcha pod njihovim Schoology računom.

Preslikavanje uloga:

- Schoology `Administrator` -> FastComments admin
- Schoology `Instructor` -> FastComments moderator
- Schoology `Student` -> FastComments commenter

**6. Schoology zamke**

- **Samo za Enterprise.** Osobni i besplatni Schoology računi ne mogu instalirati LTI 1.3 alate. Ako je vaš tenant na besplatnom planu, opcija **External Tools** nedostaje u Course Options. Nadogradite na Schoology Enterprise da biste koristili FastComments.
- **Deep Linking onemogućen prema zadanoj postavci tenant-a.** Neki Schoology tenant-i ograničavaju Deep Linking placement na razini organizacije. U tom slučaju instruktori vide samo tok **Add Materials** > **External Tool**, a ne gumb External Tool u Rich Text editoru. Da biste omogućili inline ugradnju, System Administrator ide na **System Settings** > **Integration** > **LTI 1.3** > **FastComments** i omogućava **Content Item / Deep Linking** placement, zatim sprema.
- **Nadjačavanje dodjele po sekcijama.** Ako je FastComments dodijeljen na razini organizacije, ali instruktor ga ne može vidjeti u **Add Materials**, sekcija tečaja isključena je u dodjeli na razini organizacije. Zamolite System Administratora da doda sekciju u dodjelu FastComments aplikacije.
- **Ime materijala naspram identiteta niti.** Preimenovanje materijala u Schoology-u ne premješta nit komentara. Niti su ključane prema LTI resource link ID-u, pa preimenovanje zadržava istu nit; brisanje i ponovna izrada materijala stvara novu, praznu nit.