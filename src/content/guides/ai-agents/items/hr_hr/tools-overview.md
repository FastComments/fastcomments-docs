Alati agenta su radnje koje može poduzeti. U obrascu za uređivanje agenta postoji odjeljak **Allowed tool calls** gdje označavate alate koje ovaj agent smije koristiti, i odjeljak **Approvals** gdje označavate radnje koje trebaju ljudsko odobrenje prije nego što stupe na snagu.

Postoje tri razine za bilo koji alat:

- **Disallowed** - agent ga ne može vidjeti niti koristiti.
- **Allowed, no approval** - agent ga koristi izravno. Zapisano u povijesti izvršavanja.
- **Allowed, with approval** - poziv agenta se stavlja u red za ljudski pregled i izvršava se tek kad ga čovjek odobri.

Disallowed alati su tihi: agent ih ne može tražiti, a platforma ih izričito odbija. Alati koji zahtijevaju odobrenje uvijek prolaze kroz [inbox za odobrenja](#approval-workflow).

### Revizijska traka za svaku radnju

Svaka radnja koju agent poduzme bilježi se s kratkim opravdanjem (1–2 rečenice koje objašnjavaju zašto) i ocjenom povjerenja (0.0–1.0). Oba se pojavljuju u [Prikazu detalja izvršavanja](#run-detail-view) i na svakom [odobrenju](#approval-workflow). Pretraživanje memorije je jedini izuzetak za čitanje: nije zabilježeno kao radnja i uvijek je dostupno bez obzira na dopuštenu listu.

### Referenca alata

#### Objavljivanje komentara

Omogućuje agentu da objavi komentar u svoje ime. Komentar se javno prikazuje pod prikazanim imenom agenta. Koristi se od strane agente pozdravljača i sažimatelja. Povratno - bilo koji moderator može ukloniti štetan komentar. Obično dozvoljeno bez odobrenja; postavite ograničenje ako vaša zajednica zahtijeva ljudski pregled svake javne poruke.

#### Glasanje o komentarima

Omogućuje agentu da komentaru doda glas gore ili dolje. Glas se računa u ukupni broj glasova kao i svaki drugi glas. Većina zajednica preferira da botovi ne glasaju; nije omogućeno ni u jednom početnom predlošku. Ako to dopustite, glasanje je povratno.

#### Pričvrstiti / odpričvrstiti komentar

Omogućuje agentu da pričvrsti komentar na vrh stranice ili da odpričvrsti već pričvršćeni komentar. Platforma ne nameće pravilo jednog pričvršćenog komentara po niti, pa agent za pričvršćivanje treba biti uputljen da prvo odpričvrsti prethodni pričvršćeni komentar. Koristi se u predlošku Top Comment Pinner. Povratno; obično dozvoljeno bez odobrenja.

#### Zaključati / otključati komentar

Omogućuje agentu da spriječi daljnje odgovore ispod komentara ili da ih ponovno omogući. Zaključani komentar ostaje vidljiv. Korisno za hlađenje zauzetih niti, upareno s odgođenim otključavanjem. Povratno, ali vidljivo vašoj zajednici; razmislite o zahtjevu za odobrenjem u visokorizičnim zajednicama.

#### Označiti / ukloniti oznaku neželjene pošte

Omogućuje agentu da označi komentar kao spam (sakriva ga od čitatelja i dovodi u klasifikator spama) ili da ukloni tu oznaku. Temeljni alat za bilo kojeg moderačijskog agenta. Povratno. Snažno razmotrite zahtjev za odobrenjem tijekom prvih tjedana dok ne steknete povjerenje u agenta.

#### Odobriti / opozvati odobrenje komentara

Omogućuje agentu da prikaže zadržani komentar čitateljima ili sakrije već vidljiv komentar. Najkorisnije na tenantima koji zadržavaju nove komentare na pregled moderatora. Visoki ulozi pri opozivu odobrenja vidljivog komentara - razmislite o zahtjevu za odobrenjem.

#### Označiti komentar kao pregledan

Alat za stanje reda: označava komentar kao "moderator (ili agent) je pogledao ovo." Ne mijenja vidljivost. Mali rizik; rijetko ograničeno.

#### Dodijeliti značku

Omogućuje agentu da korisniku dodeli značku iz konfiguracije znački vašeg tenanta. Povratno od strane moderatora. Rijetko ograničeno. Agent mora znati ID značke, stoga uključite relevantne ID-ove u svoje [smjernice zajednice](#community-guidelines) ili [početni prompt](#personality-prompt).

#### Slanje e-pošte

Omogućuje agentu da pošalje običan tekstualni e-mail s `noreply@fastcomments.com` na adresu koju odabere. Koristite štedljivo — e-pošta je alat sa najvećim otporom i loše poslane poruke teško je poništiti. Snažno razmotrite zahtjev za odobrenjem i usmjerite odobrenja e-pošte prema osobi koja upravlja sandučićem u koji će agent slati poruke.

#### Spremanje / pretraživanje memorije agenta

Dvije uparene funkcije koje čitaju i pišu zajednički skup bilješki o korisniku za kojeg je okidač aktiviran. Memorija je zajednička svim agentima u vašem tenant-u, tako da bilješke triage agenta informiraju odluke moderator agenta. Pretraživanje je samo za čitanje i uvijek dostupno; spremanje se rijetko ograničava. Vidi [Sustav memorije agenata](#agent-memory-system) za puni dizajn.

#### Upozoriti korisnika

Šalje privatnu izravnu poruku s upozorenjem korisniku o određenom komentaru i atomski bilježi upozorenje u memoriji agenta. Politika eskalacije platforme izgrađena je oko ovog alata — prvo upozorite, zabrana samo ako korisnik ponovno prekrši pravila. Rjeđe ograničeno nego `ban_user`, ali razmislite o ograničenju tijekom prvih tjedana rada agenta. Vidi [Upozori korisnika](#tool-warn-user) za cijelu stranicu.

#### Zabrana korisnika

Najkonzekventniji alat koji agent može pozvati. Zabrani korisnika na određeno vrijeme, opcionalno kao shadow ban, opcionalno također zabrani IP, opcionalno također obriše sve komentare korisnika. Dvije destruktivne opcije (IP, delete-all-comments) su zaštićene dodatnim pristancima na obrascu za uređivanje. U EU regiji sve zabrane zahtijevaju ljudsko odobrenje (vidi [Usklađenost s EU DSA člankom 17](#eu-dsa-compliance)). Snažno razmotrite zahtjev za odobrenjem svugdje. Vidi [Zabrani korisnika](#tool-ban-user) za cijelu stranicu.

### Podopcije alata za zabranu

Ban alat izlaže dvije destruktivne opcije - delete-all-comments i ban-by-IP - koje su modelu u potpunosti skrivene dok ih ne omogućite putem odjeljka **Ban options** na obrascu za uređivanje. Čak i ako model halucinira parametar, platforma odbija vrijednosti koje niste omogućili. Vidi [Zabrani korisnika](#tool-ban-user).