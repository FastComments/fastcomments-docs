Alatke agenta su akcije koje može preduzeti. Formular za uređivanje agenta ima odeljak **Allowed tool calls** gde označavate alate koje je agentu dozvoljeno koristiti, i odeljak **Approvals** gde označavate radnje koje bi trebalo da zahtevaju ljudsko odobrenje pre nego što stupe na snagu.

Postoje tri nivoa za svaki alat:

- **Nedozvoljeno** - agent ga ne može videti niti koristiti.
- **Dozvoljeno, bez odobrenja** - agent ga koristi direktno. Zabeleženo u istoriji izvršavanja.
- **Dozvoljeno, sa odobrenjem** - poziv agenta se stavlja u red za ljudsku proveru i izvršava se tek kada ga čovek odobri.

Nedozvoljeni alati su tihi: agent ih ne može tražiti i platforma ih odmah odbija. Alati koji zahtevaju odobrenje uvek prolaze kroz [pretinac za odobrenja](#approval-workflow).

### Revizijski trag za svaku radnju

Svaka radnja koju agent preduzme se beleži sa kratkim obrazloženjem (1–2 rečenice koje objašnjavaju zašto) i skorom poverenja (0.0–1.0). Obe se prikazuju u [Pregledu detalja izvršavanja](#run-detail-view) i na svakom [odobrenju](#approval-workflow). Pretraga memorije je jedini izuzetak koji je samo za čitanje: ona se ne beleži kao radnja i uvek je dostupna bez obzira na listu dozvoljenih.

### Referenca alata

#### Objavljivanje komentara

Omogućava agentu da objavi komentar u svoje ime. Komentar se javno prikazuje pod prikazanim imenom agenta. Koriste ga agenti za pozdravljanje i za sumiranje. Povratno - bilo koji moderator može ukloniti loš komentar. Obično je dozvoljeno bez odobrenja; ograničite ga ako vaša zajednica zahteva da svaka javna poruka prođe ljudsku proveru.

#### Uređivanje komentara

Omogućava agentu da preformuliše tekst komentara koji je u obuhvatu. Originalni tekst se čuva u revizijskom zapisu komentara. Koristiti samo u uskim slučajevima - brisanje ličnih podataka (PII) koje je korisnik slučajno otkrio, ili ispravka prethodnog odgovora agenta. Nije za prepisivanje mišljenja ili ublažavanje tona. **Snažno razmotrite stavljanje ovog alata iza odobrenja.** Pogledajte [Uredi komentar](#tool-edit-comment) za potpunu stranicu.

#### Glasanje na komentarima

Omogućava agentu da glasom podrži ili odbaci komentar. Glas se računa u ukupan broj glasova komentara kao i svaki drugi glas. Većina zajednica radije nema botove koji glasaju; nije omogućeno ni u jednom početnom šablonu. Ako ipak dozvolite, glasanje je reverzibilno.

#### Zakači / otkači komentar

Omogućava agentu da zakači komentar na vrh stranice ili otkači već zakačeni komentar. Platforma ne nameće pravilo o jednom zakačenom komentaru po niti, pa agent koji zakačuje treba biti upućen da prvo otkači prethodno zakačeni komentar. Koristi se u šablonu Top Comment Pinner. Povratno; obično dozvoljeno bez odobrenja.

#### Zaključaj / otključaj komentar

Omogućava agentu da onemogući dalji odgovor pod komentarem, ili da ponovo omogući odgovore. Zaključani komentar ostaje vidljiv. Korisno za hlađenje žustrih niti, u kombinaciji sa odloženim otključavanjem. Reverzibilno, ali vidljivo vašoj zajednici; razmotrite stavljanje iza odobrenja u zajednicama sa visokim ulozima.

#### Obeleži / ukloni kao spam

Omogućava agentu da označi komentar kao spam (sakrivajući ga od čitaoca i prosleđujući ga spam klasifikatoru) ili da ukloni tu oznaku. Osnovni alat za svakog moderatorskog agenta. Reverzibilno. Snažno razmotrite stavljanje iza odobrenja tokom prvih nedelja dok ne izgradite poverenje u agenta.

#### Odobri / poništi odobrenje komentara

Omogućava agentu da prikaže zadržani komentar čitaocima, ili da sakrije već vidljiv komentar. Najkorisnije na tenantima koji zadržavaju nove komentare za moderatorski pregled. Visok rizik pri poništavanju odobrenja vidljivog komentara - razmislite o stavljanju iza odobrenja.

#### Označi komentar kao pregledan

Alat za stanje reda: označava komentar kao "moderator (ili agent) je ovo pogledao". Ne menja vidljivost. Nizak rizik; retko se stavlja iza odobrenja.

#### Dodeli značku

Omogućava agentu da dodeli korisniku značku iz konfiguracije znački vašeg tenanta. Reverzibilno od strane moderatora. Retko staviti iza odobrenja. Agent mora znati ID značke, zato uključite relevantne ID-e u vaše [smernice zajednice](#community-guidelines) ili [početni prompt](#personality-prompt).

#### Slanje e-pošte

Omogućava agentu da pošalje plain-text e-poruku sa `noreply@fastcomments.com` na adresu koju odabere. Koristite štedljivo - e-pošta je alat sa najvećim trenjem i loše e-poruke je teško ispraviti. Snažno razmotrite stavljanje iza odobrenja, i usmerite mejlove za odobrenje kome god poseduje pretinac na koji će agent slati poruke.

#### Sačuvaj / pretraži memoriju agenta

Dva povezana alata koja čitaju i pišu zajednički skup beleški o korisniku za kojeg je okidač aktiviran. Memorija se deli među svim agentima u vašem tenant-u, tako da beleške trijažnog agenta informišu odluke moderatorskog agenta. Pretraga je samo za čitanje i uvek dostupna; čuvanje se retko stavlja iza odobrenja. Pogledajte [Sistem memorije agenta](#agent-memory-system) za kompletan dizajn.

#### Upozori korisnika

Šalje privatnu DM opomenu korisniku u vezi određenog komentara, i istovremeno evidentira upozorenje u memoriji agenta. Politika eskalacije platforme je izgrađena oko ovog alata - prvo upozori, zabrani samo ako korisnik ponovi prekršaj. Ređe se stavlja iza odobrenja nego `ban_user`, ali razmotrite stavljanje iza odobrenja tokom prvih nedelja života agenta. Pogledajte [Upozori korisnika](#tool-warn-user) za potpunu stranicu.

#### Banovanje korisnika

Najkonzekventniji alat koji agent može pozvati. Banovanje korisnika na fiksni period, opciono kao shadow-ban, opciono i zabrana po IP-u, opciono i brisanje svih korisnikovih komentara. Dve destruktivne opcije (IP, delete-all) su stavljene iza dodatnih opt-ina na formularu za uređivanje. U EU regionu, sva banovanja zahtevaju ljudsko odobrenje (pogledajte [Usklađenost sa EU DSA članom 17](#eu-dsa-compliance)). Snažno razmotrite stavljanje iza odobrenja svuda. Pogledajte [Ban user](#tool-ban-user) za kompletnu stranicu.

### Pod-opcije alata za banovanje

Alat Ban izlaže dve destruktivne opcije - delete-all-comments i ban-by-IP - koje su modelu u potpunosti skrivene dok ih ne odobrite u sekciji **Ban options** na formularu za uređivanje. Čak i ako model halucinira parametar, platforma odbija vrednosti koje niste odobrili. Pogledajte [Ban user](#tool-ban-user).