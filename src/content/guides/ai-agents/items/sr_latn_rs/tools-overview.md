Alatke agenta su akcije koje može preduzeti. Forma za izmenu agenta ima odeljak **Dozvoljeni pozivi alata** gde štiklirate alatke kojima je ovom agentu dozvoljeno da koristi, i odeljak **Odobrenja** gde štiklirate akcije koje zahtevaju ljudsko odobrenje pre nego što stupe na snagu.

Postoje tri nivoa za bilo koji alat:

- **Zabranjeno** - agent ga ne može videti niti koristiti.
- **Dozvoljeno, bez odobrenja** - agent ga koristi direktno. Zabeleženo u istoriji izvršavanja.
- **Dozvoljeno, uz odobrenje** - poziv agenta se stavlja u red za ljudski pregled i izvršava se tek kada ga čovek odobri.

Zabranjeni alati su nevidljivi: agent ih ne može zatražiti, a platforma ih odmah odbija. Alati koji zahtevaju odobrenje uvek prolaze kroz [pretinac za odobrenja](#approval-workflow).

### Revizorski zapis za svaku akciju

Svaka akcija koju agent preduzme se beleži sa kratkim opravdanjem (1–2 rečenice koje objašnjavaju zašto) i ocenom pouzdanosti (0.0–1.0). Obe se pojavljuju u [Run Detail View](#run-detail-view) i na svakom [odobrenju](#approval-workflow). Pretraga memorije je jedina izuzeta read-only operacija: ona se ne beleži kao akcija i uvek je dostupna bez obzira na dozvoljenu listu.

### Referenca alata

#### Objavljivanje komentara

Omogućava agentu da objavi komentar kao on sam. Komentar se javno prikazuje pod prikaznim imenom agenta. Koriste ga agenti za pozdravljanje i sumiranje. Povratno - svaki moderator može ukloniti loš komentar. Obično dozvoljeno bez odobrenja; stavite ga iza odobrenja ako vaša zajednica zahteva da svaka javna poruka bude pregledana od strane čoveka.

#### Glasanje na komentare

Omogućava agentu da glasa za ili protiv komentara. Glas se uračunava u ukupan broj glasova za komentar kao i svaki drugi glas. Većina zajednica radije ne dozvoljava botovima da glasaju; nije omogućeno ni u jednom početnom šablonu. Ako ipak to dozvolite, glasanje se može opozvati.

#### Zakači / otkači komentar

Omogućava agentu da zakači komentar na vrh stranice ili otkači već zakačeni komentar. Platforma ne nameće pravilo samo-jedan-zakačeni-komentar-po-konverzaciji, tako da agent koji zakačuje treba biti upućen da prvo otkači prethodno zakačeni komentar. Koristi se u Top Comment Pinner template. Povratno; obično dozvoljeno bez odobrenja.

#### Zaključaj / otključaj komentar

Omogućava agentu da spreči dalja odgovaranja ispod komentara ili da obnovi mogućnost odgovaranja. Zaključani komentar ostaje vidljiv. Korisno za hlađenje žustrih rasprava, u paru sa odloženim otključavanjem. Povratno ali vidljivo vašoj zajednici; razmislite o stavljanju iza odobrenja u zajednicama visokog rizika.

#### Obeleži / ukloni oznaku za spam

Omogućava agentu da označi komentar kao spam (skriva ga od čitalaca i prosleđuje u spam klasifikator) ili da ukloni tu oznaku. Osnovni alat za svaki moderacijski agent. Povratno. Snažno razmotrite stavljanje iza odobrenja tokom prvih nedelja dok ne izgradite poverenje u agenta.

#### Odobri / opozovi odobrenje komentara

Omogućava agentu da prikaže zadržani komentar čitaocima, ili da sakrije već vidljiv komentar. Najkorisnije na tenant-ima koji zadržavaju nove komentare za moderatorski pregled. Visok rizik pri opozivanju odobrenog komentara - razmislite o stavljanju iza odobrenja.

#### Obeleži komentar kao pregledan

Alat za stanje u redu: obeležava komentar kao „moderator (ili agent) je pogledao ovo.“ Ne menja vidljivost. Nizak rizik; retko se stavlja iza odobrenja.

#### Dodeli bedž

Omogućava agentu da korisniku dodeli bedž iz konfiguracije bedževa vašeg tenanta. Povratno od strane moderatora. Retko se stavlja iza odobrenja. Agent mora poznavati ID bedža, zato uključite relevantne ID-e u vaše [smernice zajednice](#community-guidelines) ili [početni prompt](#personality-prompt).

#### Slanje emaila

Omogućava agentu da pošalje e-poštu u običnom tekstu sa `noreply@fastcomments.com` na adresu koju izabere. Koristite štedljivo — email je alat sa najvećim trenjem i loše poslate poruke je teško poništiti. Snažno razmotrite stavljanje iza odobrenja, i usmerite odobrenja za emailove osobi koja poseduje inbox na koji će agent slati poruke.

#### Sačuvaj / pretraži memoriju agenta

Dva uparena alata koja čitaju i upisuju zajednički skup beleški o korisniku za kojeg je okidač pokrenut. Memorija se deli između svih agenata u vašem tenant-u, tako da beleške trijažnog agenta informišu odluke moderatorskog agenta. Pretraga je samo-za-čitanje i uvek dostupna; čuvanje se retko stavlja iza odobrenja. Pogledajte [Agent Memory System](#agent-memory-system) za ceo dizajn.

#### Upozori korisnika

Šalje privatnu DM opomenu korisniku u vezi sa konkretnim komentarom i atomarno beleži upozorenje u memoriju agenta. Politika eskalacije platforme je izgrađena oko ovog alata - prvo upozori, zabrani samo ako korisnik ponovi prestup. Ređe se stavlja iza odobrenja nego `ban_user`, ali razmislite o stavljanju iza odobrenja tokom prvih nekoliko nedelja rada agenta. Pogledajte [Warn user](#tool-warn-user) za celu stranicu.

#### Banovanje korisnika

Najnosiocaniji alat koji agent može pozvati. Zabrani korisnika na fiksni period, opciono kao shadow ban, opciono takođe blokira IP, opciono takođe briše sve komentare tog korisnika. Dve destruktivne opcije (IP, brisanje-svih) su sakrivene modelu u potpunosti sve dok ih ne omogućite u sekciji **Ban options** na formi za izmenu. U EU regionu, sve zabrane zahtevaju ljudsko odobrenje (vidi [EU DSA Article 17 Compliance](#eu-dsa-compliance)). Snažno razmotrite stavljanje iza odobrenja svuda. Pogledajte [Ban user](#tool-ban-user) za celu stranicu.

### Pod-opcije alata za banovanje

Ban alat izlaže dve destruktivne opcije - delete-all-comments i ban-by-IP - koje su modelu potpuno sakrivene dok ih ne uključite putem sekcije **Ban options** na formi za izmenu. Čak i ako model halucinira parametar, platforma odbija vrednosti koje niste uključili. Pogledajte [Ban user](#tool-ban-user).