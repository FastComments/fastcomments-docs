Nakon što administrator registruje FastComments kao LTI 1.3 Advantage alat i odobri politike institucije, instruktori ga dodaju u kurseve kroz standardne Blackboard tačke postavljanja. Tačni koraci se razlikuju između Ultra Course View i Original Course View, pa su oba prikazana ispod.

#### Ultra Course View

Ultra Course View je podrazumijevan u Blackboard Learn SaaS od 2026.

1. Otvorite kurs i idite na stranicu **Course Content**.
2. Zadržite pokazivač ili dodirnite mjesto gdje želite da niti komentara budu u rasporedu i kliknite ljubičasto **+** (dugme Dodaj sadržaj).
3. Izaberite **Content Market**. Panel Content Market prikazuje svaki odobren LTI alat i Building Block postavljanje za vašu instituciju.
4. Pronađite **FastComments** pločicu i kliknite je. Blackboard kreira sadržaj na poziciji gdje ste otvorili **+** meni.
5. Stavka se po defaultu pojavljuje u rasporedu kao unos "Visible to students" za instruktore koji imaju **Hide from students** isključeno kao lični podrazumijevani izbor. Ako je vaš podrazumijevani stav **Hidden**, stavka se kreira skrivena i uključite selektor vidljivosti na redu stavke kada budete spremni.
6. Da preimenujete stavku, kliknite naslov u rasporedu i otkucajte novu oznaku. Naslov koji studenti vide u rasporedu je nezavisan od FastComments identifikatora niti, tako da je preimenovanje sigurno u bilo kom trenutku.

Ako ne vidite **Content Market** kao opciju, vaša institucija je sakrila to postavljanje. Isti izbor možete također otvoriti kroz **More tools** u istom **+** meniju pod grupom **LTI Tools**.

#### Original Course View

Original Course View je i dalje podržan u Learn SaaS i ostaje primarno iskustvo za samohostovane Learn 9.1 sajtove na Q4 2024 CU izdanju.

1. Otvorite kurs i uđite u **Content Area** (na primjer, podrazumijevano **Information** ili **Content** područje u meniju kursa).
2. Uključite **Edit Mode** pomoću prekidača u gornjem desnom uglu stranice.
3. Kliknite **Build Content** u traci akcija.
4. Pod podmenijem **Learning Tools**, kliknite **FastComments**. Podmeniji Learning Tools se popunjavaju iz LTI 1.3 postavljanja alata nakon što administrator registruje alat. Ako ga ne vidite, pogledajte odjeljak o mogućim problemima ispod.
5. Na formi **Create FastComments**, podesite:
   - **Name**: oznaka koju studenti vide u području sadržaja.
   - **Description**: opcioni tekst koji se prikazuje iznad ugrađene niti.
   - **Permit Users to View this Content**: uključivanje/isključivanje dostupnosti (Da/Ne).
   - **Track Number of Views**: omogućite ako želite Blackboard statistiku po-stavci. FastComments vodi vlastitu analitiku nezavisno.
   - **Date and Time Restrictions**: opcioni prozori **Display After** / **Display Until**.
6. Pošaljite. Alat se pojavljuje kao klikabilna stavka u području sadržaja.

#### Ugrađivanje unutar stavke ili dokumenta

U oba prikaza kursa, instruktori ugrađuju FastComments linijski unutar tijela stavke, dokumenta ili bilo kojeg polja bogatog teksta preko dugmeta LTI Advantage u Content Editoru.

Ultra Course View:

1. Kreirajte ili uredite **Document**.
2. Kliknite **Add content** unutar tijela dokumenta gdje želite da se nit prikaže.
3. U alatnoj traci editora, otvorite meni **Insert content** i kliknite **Content Market** (ulazna tačka LTI Advantage / Deep Linking).
4. Izaberite **FastComments**. FastComments vraća deep-link payload i Blackboard ubacuje ugrađeni blok u tijelo dokumenta na poziciju kursora.
5. Sačuvajte dokument. Studenti vide nit prikazanu linijski dok skrolaju pored nje.

Original Course View:

1. Uredite bilo koju stavku sa tijelom bogatog teksta.
2. U alatnoj traci Content Editora, kliknite plus ikonu **Add Content** i izaberite **Content Market** (označeno kao **Add Content from External Tool** u starijim Q4 2024 CU izdanjima).
3. Izaberite **FastComments**. Editor ubacuje blok rezervisanog mjesta koji referencira deep-linked resurs.
4. Pošaljite stavku.

Svako deep-link ugradjenje proizvodi vlastitu FastComments nit, tako da stavka sa dva ugrađena FastComments bloka ima dva nezavisna toka komentara.

#### Vidljivost, uslovi objavljivanja i ograničenja grupa

FastComments sadržajne stavke se ponašaju kao bilo koja druga Blackboard sadržajna stavka u pogledu pravila pristupa koje se primjenjuju na njih.

- Ultra: kliknite selektor vidljivosti na redu (**Visible to students**, **Hidden from students**, **Conditional availability**). Conditional availability podržava vremenske intervale, pravila zasnovana na učinku u odnosu na stavke iz dnevnika ocjena, i pravila članstva u odnosu na grupe kursa.
- Original: otvorite kontekst meni stavke i izaberite **Adaptive Release** ili **Adaptive Release: Advanced** da ograničite pristup alatu po datumu, članstvu, ocjeni ili statusu pregleda. Koristite **Set Group Availability** na stavci da ograničite pristup specifičnim grupama kursa.

FastComments poštuje odluku Blackboard-ove kontrole pristupa. Ako Blackboard sakrije stavku od studenta, LTI pokretanje za tog studenta se nikada ne dešava i oni se ne pojavljuju u moderator prikazu.

#### Ponašanje u dnevniku ocjena

FastComments ne šalje ocjene nazad preko LTI Advantage Assignment and Grade Services. Ne kreira se automatski kolona ocjena za FastComments sadržajne stavke.

Ako je vaš Blackboard tenant konfigurisan da automatski kreira kolonu u dnevniku ocjena za svaku novu sadržajnu stavku bez obzira na metapodatke o ocjenjivanju, prazna kolona će se pojaviti. Da je sakrijete:

- Ultra: otvorite **Gradebook**, kliknite zaglavlje kolone, izaberite **Edit**, i isključite **Show to students** plus **Include in calculations**. Ili koristite **Delete** ako vaša institucija dopušta brisanje kolona za neocijenjene stavke.
- Original: otvorite **Grade Center**, kliknite strelicu (chevron) kolone, izaberite **Hide from Users (on/off)**, i opcionalno **Hide from Instructor View** pod **Column Organization**.

#### Šta studenti vide

Kada student otvori FastComments stavku ili skrola do ugrađenog bloka:

1. Blackboard pokreće LTI 1.3 poruku ka FastComments. Student se prijavljuje preko SSO koristeći svoj Blackboard identitet (ime, email, avatar, uloga) bez prikaza obrasca za prijavu.
2. Nit komentara se renderuje u iframe-u. Threading, odgovori, pomeni i reakcije su svi dostupni u zavisnosti od postavki widgeta komentara konfigurisanih u FastComments.
3. Njihovi komentari su pripisani njihovom Blackboard nalogu. Ako student naknadno izmijeni ime ili fotografiju u Blackboard-u, sljedeće pokretanje ažurira FastComments profil.

Mapiranje uloga iz Blackboard-a u FastComments:

- **System Administrator** i **Course Builder** mapiraju se na FastComments **admin**.
- **Instructor** i **Teaching Assistant** mapiraju se na FastComments **moderator**.
- **Student**, **Guest**, i **Observer** mapiraju se na FastComments **commenter**.

Moderatori vide kontrole moderacije (pinovanje, skrivanje, zabrana, brisanje) direktno pored svakog komentara u niti.

#### Opseg niti

FastComments određuje opseg svake niti prema **(Blackboard host, course ID, resource link ID)**. Dva FastComments unosa u istom kursu proizvode dvije niti. Ista stavka kopirana preko dvaju kursnih školjki (na primjer, kroz kopiju kursa) proizvodi dvije niti, jer Blackboard izdaje novi resource link ID tokom kopiranja. Da biste zadržali zajedničku nit kroz kopije kursa, koristite Deep Linking sa eksplicitnim thread URN-om koji je konfigurisan u FastComments prije pokretanja kopije.

#### Blackboard-specifični problemi

**Nedostaje FastComments pločica u Build Content meniju (Original) ili Content Marketu (Ultra).** Administrator je odobrio alat ali ostavio politiku institucije koja blokira odgovarajuće postavljanje. Idite na **Administrator Panel** > **Integrations** > **LTI Tool Providers**, uredite FastComments unos i potvrdite da su omogućena oba postavljanja **Course Content Tool** (Original) i **Course Content Tool - allow students** / **Deep Linking content tool** (Ultra). Sačuvajte i osvježite stranicu kursa.

**Greška "Tool not configured for this context" ili "Tool is not deployed" prilikom pokretanja.** Opseg deploymenta registrisan tokom dinamičke registracije se ne poklapa sa kontekstom institucije kojem kurs pripada. U Blackboard-ovom unosu provajdera alata, provjerite da li se **Deployment ID** poklapa sa onim što FastComments prikazuje na svojoj LTI 1.3 Configuration stranici za ovog tenant-a. Ako se razlikuju, obrišite postavljanje i ponovo pokrenite dinamičku registraciju sa svježim URL-om za registraciju.

**Visina iframe-a izgleda fiksno ili se sadržaj odseca.** Neki Blackboard tenanti dolaze sa strogom Content Security Policy koja blokira podrazumijevani LTI iframe-resize postMessage. FastComments emituje i Canvas-stil `lti.frameResize` poruku i IMS spec-formu `org.imsglobal.lti.frameResize` poruku radi maksimalne kompatibilnosti, ali tenant-nivo CSP override može blokirati parent listener. Zamolite vašeg administratora da potvrdi da je `*.fastcomments.com` na allowlisti LTI alata i da nijedan prilagođeni CSP header ne uklanja postMessage evente. Nakon toga resizing radi bez dodatne konfiguracije.

**Kopija kursa duplira niti.** Blackboard kopija kursa izdaje nove resource link ID-jeve za LTI postavke, pa kopirani kursevi počinju sa praznim nitima. Ovo je očekivano. Ako želite da kopirani kurs naslijedi originalnu nit, postavite Deep Linking sa eksplicitnim thread URN prije kopiranja, ili kontaktirajte FastComments podršku da masovno remapiraju ID-jeve niti.

**Student vidi generičku Blackboard grešku pri pokretanju.** Uzrok je nedostajući ili zastarjeli `email` claim. Potvrdite da institucijska politika za FastComments ima omogućeno **Role**, **Name**, i **Email Address** pod **User Fields to Send**. Sačuvajte, zatim ponovo pokrenite u novoj sesiji pretraživača.