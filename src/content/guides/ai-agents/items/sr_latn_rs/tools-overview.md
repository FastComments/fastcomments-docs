Alatke agenta su radnje koje može preduzeti. Formular za izmenu agenta ima sekciju **Dozvoljeni pozivi alata** u kojoj označavate alate koje je ovom agentu dozvoljeno da koristi, i sekciju **Odobrenja** u kojoj označavate akcije koje bi trebalo da zahtevaju ljudsko odobrenje pre nego što stupe na snagu.

Postoje tri nivoa za bilo koji alat:

- **Zabranjeno** - agent ga ne može videti niti koristiti.
- **Dozvoljeno, bez odobrenja** - agent ga koristi direktno. Zabeleženo u istoriji izvršavanja.
- **Dozvoljeno, uz odobrenje** - poziv agenta se stavi u red za ljudsku reviziju i izvršava se tek kada ga čovek odobri.

Zabranjeni alati su tihi: agent ih ne može zahtevati i platforma ih odmah odbija. Alati koji zahtevaju odobrenje uvek prolaze kroz [pretinac za odobrenja](#approval-workflow).

### Trag revizije za svaku akciju

Svaka akcija koju agent preduzme se beleži sa kratkim opravdanjem (1–2 rečenice koje objašnjavaju zašto) i skorom poverenja (0.0–1.0). Obe informacije se pojavljuju u [Pregled detalja izvršavanja](#run-detail-view) i na svakom [odobrenju](#approval-workflow). Pretraživanje memorije je jedini izuzetak koji je samo za čitanje: ono se ne beleži kao akcija i uvek je dostupno bez obzira na listu dozvoljenih alata.

### Referenca alata

#### Objavljivanje komentara

Omogućava agentu da objavi komentar u svoje ime. Komentar je javno prikazan pod prikaznim imenom agenta. Koristi se kod agenata za pozdravljanje i sažimanje. Reverzibilno - bilo koji moderator može ukloniti loš komentar. Obično je dozvoljeno bez odobrenja; stavite ga iza odobrenja ako vaša zajednica zahteva da svaka javna poruka bude pregledana od strane čoveka.

#### Izmena komentara

Omogućava agentu da preformuliše tekst komentara koji je u opsegu. Originalni tekst se čuva u revizijskom zapisu komentara. Rezervišite za uske slučajeve - brisanje PII koje je korisnik slučajno otkrio, ili izmena prethodnog odgovora agenta. Ne za prepravljanje stavova ili ublažavanje tona. **Snažno razmotrite postavljanje iza odobrenja.** Pogledajte [Uredi komentar](#tool-edit-comment) za celu stranicu.

#### Glasanje o komentarima

Omogućava agentu da označi komentar kao glas za/ protiv. Glas se računa u ukupnom broju glasova za komentar kao i svaki drugi glas. Većina zajednica preferira da botovi ne glasaju; nije omogućeno ni u jednom početnom šablonu. Ako to dozvolite, glasanje je reverzibilno.

#### Pričvrsti / otkini komentar

Omogućava agentu da pričvrsti komentar na vrh stranice ili da otkine već pričvršćeni komentar. Platforma ne primenjuje pravilo jedan-pričvršćeni-po-niti, pa agent koji pričvršćuje treba da bude instruktovan da prvo otkine prethodni pričvršćeni komentar. Koristi se u šablonu Top Comment Pinner. Reverzibilno; obično dozvoljeno bez odobrenja.

#### Zaključaj / otključaj komentar

Omogućava agentu da spreči dalja odgovaranja ispod komentara, ili da obnovi odgovore. Zaključani komentar ostaje vidljiv. Korisno za hlađenje žestokih tema, u paru sa odloženim otključavanjem. Reverzibilno, ali vidljivo vašoj zajednici; razmotrite stavljanje iza odobrenja u zajednicama visokog rizika.

#### Obeleži / ukloni obeležavanje kao spam

Omogućava agentu da označi komentar kao spam (skrivajući ga od čitaoca i prosleđujući ga klasifikatoru spama) ili da ukloni tu oznaku. Osnovni alat za bilo kog moderatornog agenta. Reverzibilno. Snažno razmotrite stavljanje iza odobrenja u prvim nedeljama dok ne izgradite poverenje u agenta.

#### Odobri / poništi odobrenje komentara

Omogućava agentu da prikaže zadržani komentar čitaocima, ili da sakrije već vidljiv komentar. Najkorisnije na zakupcima koji zadržavaju nove komentare za pregled moderatora. Visok nivo rizika pri poništavanju odobrenog komentara - razmotrite stavljanje iza odobrenja.

#### Obeleži komentar kao pregledan

Alat za stanje u redu: obeležava komentar kao „moderator (ili agent) je pogledao ovo.“ Ne menja vidljivost. Nizak rizik; retko je postavljeno iza odobrenja.

#### Dodeli bedž

Omogućava agentu da dodeli korisniku bedž koji ste konfigurisali za vaš zakup. Reverzibilno od strane moderatora. Retko je postavljeno iza odobrenja. Kada je ovaj alat omogućen, agent može videti bedževe vašeg zakupca i sam odabrati odgovarajući, pa ne morate da lepitate identifikatore bedževa u smernice zajednice ili početni prompt. Ako želite da usmeravate koji se bedž dodeljuje za koje ponašanje, pozivajte se na bedževe po njihovom **Display Label** u promptu.

#### Pošalji e‑poštu

Omogućava agentu da pošalje plain-text e‑poštu autoru komentara u opsegu koji je pokrenuo trik. Agent nikada ne vidi adresu primaoca - bira komentar, a platforma isporučuje na adresu koju je taj komentator ostavio prilikom objave. From-adresa je brendirani pošiljalac vašeg zakupca (sa DKIM) kada domen komentara odgovara konfigurisanim domenima, u suprotnom se koristi podrazumevani pošiljalac platforme. Koristite štedljivo - e‑pošta ima najveći prag trenja i loše e‑poruke je teško poništiti. Snažno razmotrite stavljanje iza odobrenja i usmerite mejlove za odobrenje onome ko poseduje inbox na koji će agent slati poruke.

#### Sačuvaj / pretraži memoriju agenta

Dva povezana alata koji čitaju i upisuju zajednički skup beleški o korisniku za kog je okidač pokrenut. Memorija se deli među svim agentima u vašem zakupcu, tako da beleške trijažnog agenta informišu odluke moderatornog agenta. Pretraga je samo za čitanje i uvek dostupna; snimanje se retko stavlja iza odobrenja. Pogledajte [Sistem memorije agenta](#agent-memory-system) za kompletan dizajn.

#### Upozori korisnika

Šalje privatnu DM opomenu korisniku u vezi sa konkretnim komentarom, i atomски beleži opomenu u memoriji agenta. Politika eskalacije platforme je izgrađena oko ovog alata - prvo opomeni, zabrani samo ako korisnik ponovi prekršaj. Ređe je postavljeno iza odobrenja nego `ban_user`, ali razmotrite postavljanje iza odobrenja tokom prvih nedelja rada agenta. Pogledajte [Upozori korisnika](#tool-warn-user) za celu stranicu.

#### Zabrani korisnika

Najteži alat koji agent može pozvati. Zabrani korisnika na fiksno trajanje, opcionalno kao shadow ban, opciono takođe zabrani IP, opciono takođe obriše sve komentare korisnika. Dve destruktivne opcije (IP, delete-all) su zaključane iza dodatnih opcija na formi za uređivanje. U EU regionu, sve zabrane zahtevaju ljudsko odobrenje (pogledajte [Usklađenost sa EU DSA članom 17](#eu-dsa-compliance)). Snažno razmotrite postavljanje iza odobrenja svuda. Pogledajte [Zabrani korisnika](#tool-ban-user) za celu stranicu.

### Podopcije alata za zabranu

Alat za zabranu izlaže dve destruktivne opcije - delete-all-comments i ban-by-IP - koje su potpuno skrivene modelu dok ih ne uključite preko sekcije **Opcije zabrane** na formi za izmenu. Čak i ako model halucinira parametar, platforma odbija vrednosti koje niste uključili. Pogledajte [Zabrani korisnika](#tool-ban-user).