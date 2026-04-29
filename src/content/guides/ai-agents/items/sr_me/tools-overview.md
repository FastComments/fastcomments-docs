Alatke agenta su akcije koje može preduzeti. Formular za izmenu agenta ima odeljak **Allowed tool calls** gde označite alate koje je ovom agentu dozvoljeno koristiti, i odeljak **Approvals** gde označite akcije koje zahtevaju ljudsko odobrenje pre nego što stupe na snagu.

Postoje tri nivoa za bilo koji alat:

- **Disallowed** - agent ga ne može vidjeti niti koristiti.
- **Allowed, no approval** - agent ga koristi direktno. Zabilježeno u istoriji izvršavanja.
- **Allowed, with approval** - poziv agenta se stavlja u red za ljudsku reviziju i izvršava se tek kada ga čovjek odobri.

Zabranjeni alati su nevidljivi: agent ih ne može tražiti i platforma ih odmah odbija. Alati koji su uslovljeni odobrenjem uvijek prolaze kroz [approvals inbox](#approval-workflow).

### Revizorski trag za svaku radnju

Svaka radnja koju agent preduzme se beleži sa kratkim opravdanjem (1–2 rečenice koje objašnjavaju zašto) i skorom poverenja (0.0–1.0). Oba se prikazuju u [Run Detail View](#run-detail-view) i na svakom [approval](#approval-workflow). Pretraga memorije je jedini izuzetak koji je samo za čitanje: ona se ne beleži kao akcija i uvijek je dostupna bez obzira na allowlist.

### Referenca alata

#### Posting comments

Omogućava agentu da postavi komentar u svoje ime. Komentar se javno prikazuje pod imenom agenta. Koristi se za agenta koji dočekuje korisnike i za agente koji sažimaju. Povratno - bilo koji moderator može ukloniti loš komentar. Obično dozvoljeno bez odobrenja; stavite ga iza odobrenja ako vaša zajednica zahtijeva da svaka javna poruka bude pregledana od strane čovjeka.

#### Editing a comment

Omogućava agentu da prepiše tekst komentara koji je u opsegu. Originalni tekst se čuva u revizorskom zapisu komentara. Ostavite samo za uske slučajeve - brisanje PII koje je korisnik otkrio, ili ispravka sopstvenog prethodnog odgovora agenta. Ne treba ga koristiti za preformulisanje mišljenja ili ublažavanje tona. **Snažno razmotrite stavljanje zahtjeva za odobrenje.** Pogledajte [Edit comment](#tool-edit-comment) za cijelu stranicu.

#### Voting on comments

Omogućava agentu da glasa za ili protiv komentara. Glas se uračunava u ukupan broj glasova komentara kao i svaki drugi glas. Većina zajednica radije nema botove koji glasaju; nije uključen ni u jednom starter šablonu. Ako to dozvolite, glasanje je povratno.

#### Pin / unpin a comment

Omogućava agentu da zakači komentar na vrh stranice ili da otkači već zakačeni. Platforma ne nameće pravilo jednog pina po temi, pa agent koji zakači treba biti instruisan da prvo otkači prethodno zakačeni komentar. Koristi se u Top Comment Pinner template. Povratno; obično dozvoljeno bez odobrenja.

#### Lock / unlock a comment

Omogućava agentu da spriječi dalje odgovore ispod komentara, ili da vrati mogućnost odgovora. Zaključani komentar ostaje vidljiv. Korisno za hlađenje žustrijih niti, upareno sa odloženim otključavanjem. Povratno ali vidljivo vašoj zajednici; razmotrite stavljanje iza odobrenja u zajednicama visokog rizika.

#### Mark / unmark spam

Omogućava agentu da označi komentar kao spam (sakriva ga od čitaoca i prosleđuje spam klasifikatoru) ili da očisti tu oznaku. Osnovni alat za bilo kojeg moderacijskog agenta. Povratno. Snažno razmotrite stavljanje iza odobrenja tokom prvih nedjelja dok ne izgradite povjerenje u agenta.

#### Approve / un-approve a comment

Omogućava agentu da prikaže zadržani komentar čitaocima, ili da sakrije već vidljiv. Najkorisnije za tenant-e koji zadržavaju nove komentare za moderatorski pregled. Visok rizik kada se vidljiv komentar poništi — razmotrite stavljanje zahtjeva za odobrenje.

#### Mark a comment reviewed

Alat za stanje reda: označava komentar kao „moderator (ili agent) ga je pregledao“. Ne mijenja vidljivost. Niski rizik; rijetko se stavlja iza odobrenja.

#### Award a badge

Omogućava agentu da dodijeli korisniku značku iz konfiguracije znački vašeg tenanta. Povratno od strane moderatora. Rijetko se stavlja iza odobrenja. Agent mora znati ID značke, pa uključite relevantne ID-e u vaše [community guidelines](#community-guidelines) ili [initial prompt](#personality-prompt).

#### Send email

Omogućava agentu da pošalje plain-text email sa `noreply@fastcomments.com` na adresu koju izabere. Koristite štedljivo — email je alat sa najvećim trenjem i loše poslane poruke je teško poništiti. Snažno razmotrite stavljanje iza odobrenja, i usmjerite odobrenja za email onome ko posjeduje inbox koji će agent na kraju koristiti.

#### Save / search agent memory

Dva povezana alata koji čitaju i pišu u zajednički pool bilješki o korisniku za kojeg je okidač aktiviran. Memorija se dijeli između svih agenata u vašem tenant-u, tako da bilješke trijažnog agenta informišu odluke moderatorskog agenta. Pretraga je samo za čitanje i uvijek dostupna; čuvanje se rijetko stavlja iza odobrenja. Pogledajte [Agent Memory System](#agent-memory-system) za punu konstrukciju.

#### Warn a user

Šalje privatnu DM opomenu korisniku u vezi sa konkretnim komentarom, i atomски upisuje opomenu u memoriju agenta. Politika eskalacije platforme je izgrađena oko ovog alata - prvo opomeni, zabrani samo ako korisnik ponovi prekršaj. Rjeđe se stavlja iza odobrenja nego `ban_user`, ali razmotrite stavljanje iza odobrenja tokom prvih nedjelja rada agenta. Pogledajte [Warn user](#tool-warn-user) za cijelu stranicu.

#### Ban a user

Najkonsekventniji alat koji agent može pozvati. Ban-uje korisnika na fiksno trajanje, opcionalno kao shadow ban, opcionalno takođe ban-uje IP, opcionalno briše sve korisnikove komentare. Dvije destruktivne opcije (IP, delete-all) su uslovljene dodatnim opt-in-ovima na formularu za izmenu. U regionu EU, svi banovi zahtijevaju ljudsko odobrenje (pogledajte [EU DSA Article 17 Compliance](#eu-dsa-compliance)). Snažno razmotrite stavljanje iza odobrenja svuda. Pogledajte [Ban user](#tool-ban-user) za cijelu stranicu.

### Ban-tool sub-options

Ban alat izlaže dvije destruktivne opcije - delete-all-comments i ban-by-IP - koje su potpuno skrivene modelu dok ih ne omogućite kroz sekciju **Ban options** na formularu za izmenu. Čak i ako model halucinira parametar, platforma odbija vrijednosti koje niste uključili. Pogledajte [Ban user](#tool-ban-user).