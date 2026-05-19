Nakon što je FastComments registrovan na platformi, instruktori ga dodaju u sadržaj kursa koristeći standardne tokove za eksterne alate platforme. Ova stranica pokriva Sakai 23.x i Schoology Enterprise.

#### Zaključajte javni pristup (preporučeno)

Po defaultu, FastComments podaci o komentarima su javno čitljivi na obe platforme. Bilo ko ko pogodi URL niti ili API endpoint može videti komentare, čak i van Sakai ili Schoology. Za diskusije na kursevima gotovo sigurno želite ograničiti pregled samo na upisane studente.

Otvorite svoju <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">stranicu za prilagođavanje vidžeta</a> i kreirajte pravilo sa omogućenim **Require SSO To View Comments**, zatim postavite nivo bezbednosti na **Secure SSO** tako da se niti mogu učitavati samo kroz potpisano LTI pokretanje.

Pogledajte [Zaštita niti komentara pomoću Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) za kompletan vodič, uključujući kako da ograničite pravilo na jedini domen ili stranicu.

#### Sakai

**1. Dodajte FastComments na sajt**

Održavalac sajta omogućava alat na nivou pojedinačnog sajta:

1. Otvorite sajt i kliknite **Site Info** u levom meniju.
2. Kliknite **Manage Tools**.
3. Skrolujte do liste **External Tools** i uključite **FastComments**.
4. Kliknite **Continue**, pregledajte listu alata, zatim kliknite **Finish**.

FastComments se sada pojavljuje kao stavka u levom meniju na sajtu.

**2. Promena redosleda stavke u levom meniju**

Idite na **Site Info** > **Tool Order**. Prevucite **FastComments** na željenu poziciju i kliknite **Save**. Takođe možete promeniti naziv stavke u meniju i sakriti je od studenata sa ovog ekrana.

**3. Umetanje unutar stranice Lessons**

Da biste postavili FastComments direktno unutar stranice Lessons umesto kao samostalan alat u levom meniju:

1. Otvorite alat **Lessons** na sajtu.
2. Kliknite **Add Content** > **Add External Tool**.
3. Selektujte **FastComments** sa liste.
4. Ako je FastComments tokom registracije oglasio Deep Linking, Sakai otvara alat za izbor sadržaja tako da možete izabrati ili označiti nit. Ako Deep Linking nije oglašen, Sakai umeće podrazumevani link za pokretanje.
5. Sačuvajte stavku u Lessons.

Svaka ugrađena instanca dobija sopstvenu nit, ograničenu na taj resursni link.

**4. Podešavanja dozvola za pristup studenata**

Sakai kontroliše pokretanja eksternih alata preko Realms. Da potvrdite da studenti mogu pokrenuti FastComments:

1. Prijavite se kao Sakai admin i otvorite **Administration Workspace** > **Realms**.
2. Otvorite odgovarajući realm (na primer, `!site.template.course` ili specifični realm sajta).
3. Potvrdite da `access` uloga ima omogućeno `lti.launch` i da su role permissions u grupi **external.tools** dodeljene.
4. Sačuvajte realm.

Za prepiske na nivou sajta, održavalac može podesiti vidljivost alata po ulozi iz **Site Info** > **Tool Order** tako što će sakriti ili prikazati FastComments po ulozi.

**5. Šta studenti vide**

Studenti kliknu na FastComments stavku u levom meniju (ili se skroluju do ugrađenog bloka u Lessons) i dolaze direktno u prikaz niti komentara. SSO je automatski: Sakai šalje identitet korisnika u LTI pokretanju i FastComments ih prijavljuje pod njihovim Sakai nalogom.

Role mapping:

- Sakai `Instructor` -> FastComments moderator
- Sakai `Admin` (admin u Administratorskom radnom prostoru) -> FastComments administrator
- Sakai `Student` / `access` -> FastComments komentator

**6. Mogući problemi u Sakai-u**

- **Alat nije vidljiv u Manage Tools.** Ako se FastComments ne pojavljuje u listi External Tools, Sakai admin treba da otvori registar alata (**Administration Workspace** > **External Tools** > **FastComments**) i postavi **Stealthed** na `false`. Stealthed alati su skriveni iz izbora za upravljanje alatima po sajtu.
- **Prekidanja pokretanja u pregledačima sa deljenom sesijom.** Sakai-jev portal CSRF token je vezan za sesiju pregledača. Ako je student prijavljen na dva Sakai sajta u različitim tabovima ili ima zastarelu sesiju, pokretanje vraća 403. Rešenje: zatvorite druge Sakai tabove, odjavite se, ponovo se prijavite i ponovo pokrenite. Admini takođe mogu povećati `sakai.csrf.token.cache.ttl` ako se ovo dešava u klasteru.
- **Ugradnja u okviru (frame).** Potvrdite da je `lti.frameheight` u `sakai.properties` dovoljno velik (600 ili više) kako nit komentara ne bi bila isečena unutar stranice Lessons.

#### Schoology

Schoology Enterprise ima dva scenarija instalacije. Potvrdite koji se primenjuje pre nego što dodate alat u kurs.

**1. Dva scenarija instalacije**

- **(a) Instalacija na nivou preduzeća.** Schoology System Administrator je instalirao FastComments na nivou organizacije i dodelio ga svim kursevima ili specifičnim šablonima kurseva. Instruktori preskaču instalaciju i idu direktno na "Add Materials".
- **(b) Samostalna instalacija od strane instruktora.** Instruktor instalira alat u pojedinačni kurs iz **Course Options** > **External Tools** > **Install LTI Apps**. Samostalna instalacija zahteva da je System Administrator prethodno odobrio FastComments aplikaciju na nivou organizacije.

**2. Dodajte FastComments kao materijal za kurs**

Unutar kursa:

1. Otvorite kurs i idite na **Materials**.
2. Kliknite **Add Materials** > **Add File/Link/External Tool**.
3. Izaberite **External Tool**.
4. Selektujte **FastComments** sa liste registrovanih alata.
5. Postavite **Name** (ovo je ono što studenti vide u listi materijala) i opcioni **Description**.
6. Ostavite **Enable Grading** (grade passback) **OFF**. FastComments ne šalje ocene nazad u Schoology, pa uključivanje slanja ocena pravi prazan stupac u knjizi ocena.
7. Kliknite **Submit**.

Materijal se sada pojavljuje u listi materijala kursa i otvara FastComments nit kada se klikne.

**3. Inline ugrađivanje putem Rich Text editora**

Ako je System Administrator omogućio Deep Linking placement za FastComments tokom registracije, instruktori mogu ugraditi nit komentara unutar bilo kog Rich Text polja (instrukcije zadatka, tela stranica, razgovorne teme):

1. Otvorite Rich Text editor na ciljnoj stranici.
2. Kliknite ikonu **External Tool** (ikona slagalice) u traci sa alatkama.
3. Izaberite **FastComments**.
4. Konfigurišite ugradnju u dijalogu za deep-linking i kliknite **Insert**.
5. Sačuvajte stranicu.

Ako dugme External Tool nije vidljivo u Rich Text editoru, Deep Linking je onemogućen za ovaj alat na ovom tenant-u. Pogledajte niže navedene probleme.

**4. Vidljivost i dodela po sekcijama**

Schoology ograničava dostupnost alata po sekcijama kroz Course Options:

1. Iz kursa, kliknite **Course Options** > **External Tools**.
2. Za svaku instaliranu LTI aplikaciju, kontrolišete da li je dostupna svim sekcijama u kursu ili samo određenim sekcijama.
3. Da ograničite FastComments na određene sekcije, odčekirajte sekcije koje ne bi trebalo da vide alat.
4. Pristup po sekcijama takođe kontroliše koje sekcije vide unos **Add Materials** > **External Tool** za FastComments.

**5. Šta studenti vide**

Studenti kliknu na FastComments materijal (ili se skroluju do inline ugradnje) i dolaze u nit diskusije. SSO je automatski putem Schoology LTI pokretanja pod njihovim Schoology nalogom.

Role mapping:

- Schoology `Administrator` -> FastComments administrator
- Schoology `Instructor` -> FastComments moderator
- Schoology `Student` -> FastComments komentator

**6. Mogući problemi u Schoology-u**

- **Samo za Enterprise.** Lični i besplatni Schoology nalozi ne mogu instalirati LTI 1.3 alate. Ako je vaš tenant na besplatnom nivou, opcija **External Tools** je odsutna iz Course Options. Nadogradite na Schoology Enterprise da biste koristili FastComments.
- **Deep Linking onemogućen po defaultu za tenant.** Neki Schoology tenant-i ograničavaju Deep Linking placement na nivou organizacije. Kada je to slučaj, instruktori vide samo tok **Add Materials** > **External Tool**, a ne dugme External Tool u Rich Text editoru. Da biste omogućili inline ugradnju, System Administrator ide na **System Settings** > **Integration** > **LTI 1.3** > **FastComments** i uključuje **Content Item / Deep Linking** placement, zatim sačuva.
- **Preklapanje dodele po sekcijama.** Ako je FastComments dodeljen na nivou preduzeća, ali instruktor ne može da ga vidi u **Add Materials**, sekcija kursa je isključena u dodeli na nivou organizacije. Zamolite System Administrator-a da doda sekciju u dodelu FastComments aplikacije.
- **Naziv materijala naspram identiteta niti.** Preimenovanje materijala u Schoology ne pomera nit komentara. Niti su vezane za LTI resource link ID, tako da preimenovanje zadržava istu nit; brisanje i ponovno kreiranje materijala stvara novu, praznu nit.

---