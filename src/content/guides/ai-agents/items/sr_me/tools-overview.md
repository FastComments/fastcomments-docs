Alati agenta su akcije koje može preduzeti. Forma za izmenu agenta ima sekciju **Dozvoljeni pozivi alata** gde čekirate alate koje ovaj agent sme da koristi, i sekciju **Odobrenja** gde čekirate akcije koje bi trebalo da zahtevaju da ih čovek odobri pre nego što stupe na snagu.

Postoje tri nivoa za bilo koji alat:

- **Zabranjeno** - agent ga ne može videti niti koristiti.
- **Dozvoljeno, bez odobrenja** - agent ga koristi direktno. Zabeleženo u istoriji izvršavanja.
- **Dozvoljeno, uz odobrenje** - poziv agenta se stavlja u red za ljudski pregled i izvršava se tek kada ga čovek odobri.

Zabranjeni alati su tihi: agent ih ne može zatražiti i platforma ih odmah odbija. Alati koji zahtevaju odobrenje uvek prolaze kroz [inbox za odobrenja](#approval-workflow).

### Revizorski trag za svaku akciju

Svaka akcija koju agent preduzme se beleži sa kratkim opravdanjem (1–2 rečenice koje objašnjavaju zašto) i skorom poverenja (0.0–1.0). Obe stavke se prikazuju u [Pregledu detalja izvršavanja](#run-detail-view) i pri svakom [odobrenju](#approval-workflow). Pretraživanje memorije je jedini izuzetak koji je samo za čitanje: ono nije zabeleženo kao akcija i uvek je dostupno bez obzira na listu dozvoljenih alata.

### Referenca alata

#### Objavljivanje komentara

Dozvoljava agentu da objavi komentar u svoje ime. Komentar se javno prikazuje pod imenom agenta. Koristi se kod agenata za pozdravljanje i rezimiranje. Moguće opozvati - svaki moderator može ukloniti loš komentar. Stavite iza odobrenja ako vaša zajednica zahteva da svaka javna poruka ima ljudski pregled.

#### Uređivanje komentara

Dozvoljava agentu da preformuliše tekst komentara koji je u okviru opsega. Originalni tekst se čuva u revizorskom dnevniku komentara. Ostavite za uske slučajeve — brisanje PII koje je korisnik otkrio, ili ispravka prethodnog odgovora agenta. Ne koristiti za prepravljanje mišljenja ili ublažavanje tona. Pogledajte [Uredi komentar](#tool-edit-comment) za celu stranicu.

#### Glasanje o komentarima

Dozvoljava agentu da glasa za ili protiv komentara. Glas se uračunava u ukupan broj glasova kao i bilo koji drugi glas. Većina zajednica radije nema botove koji glasaju; nije omogućeno ni u jednom početnom šablonu. Ako dozvolite, glasanje je moguće opozvati.

#### Zakači / otkači komentar

Dozvoljava agentu da zakači komentar na vrh stranice ili da otkači već zakačeni komentar. Platforma ne primenjuje pravilo jednog zakačenog komentara po niti, pa agent koji zakačuje treba da bude uputjen da prvo otkači prethodno zakačeni komentar. Da otkrije šta je već zakačeno na istoj stranici, agent može pozvati alat samo za čitanje `get_pinned_comments` (vidi dole). Koristi se u šablonu Top Comment Pinner.

#### Zaključaj / otključaj komentar

Dozvoljava agentu da zabrani dalja odgovaranja ispod komentara, ili da obnovi mogućnost odgovaranja. Zaključani komentar ostaje vidljiv. Korisno za pauze za smirivanje u vrelim nitima, u paru sa odloženim otključavanjem. Da otkrije šta je trenutno zaključano na istoj stranici, agent može pozvati alat samo za čitanje `get_locked_comments` (vidi dole).

#### Obeleži / ukloni oznaku spama

Dozvoljava agentu da označi komentar kao spam (sakriva ga od čitalaca i prosleđuje klasifikatoru spama) ili da ukloni tu oznaku. Osnovni alat za bilo kog moderatorskog agenta. Moguće opozvati.

#### Odobri / poništi odobrenje komentara

Dozvoljava agentu da prikaže zadržani komentar čitaocima, ili sakrije već vidljiv komentar. Najkorisnije na tenant-ima koji zadržavaju nove komentare radi moderatorskog pregleda.

#### Obeleži komentar kao pregledan

Alat za stanje reda: označava komentar kao „moderator (ili agent) je pogledao ovo“. Ne menja vidljivost. Niska vrednost; retko se stavlja iza odobrenja.

#### Dodeli značku

Dozvoljava agentu da dodeli korisniku značku koju ste konfigurirali za vaš tenant. Moguće opozvati od strane moderatora. Kada je ovaj alat omogućen, agent može videti značke vašeg tena nta i sam izabrati odgovarajuću, pa ne morate da lepite identifikatore znački u smernice zajednice ili inicijalni prompt. Da usmerite koju značku dodeliti za koje ponašanje, referišite se na značke po njihovom **Display Label** u promptu.

#### Pošalji email

Dozvoljava agentu da pošalje običan tekstualni e‑mail autoru komentara unutar opsega okidača. Agent nikada ne vidi adresu primaoca — on bira komentar, a platforma dostavlja na adresu koju je taj komentator ostavio prilikom objave. From-adresa je brendirani pošiljalac vašeg tena nta (sa DKIM) kada domen komentara odgovara konfigurisanom domenu, inače se koristi podrazumevani pošiljalac platforme. Koristite štedljivo — e‑mail je alat sa najvećim otporom i loše poslate poruke je teško ispraviti.

#### Sačuvaj / pretraži memoriju agenta

Dva uparena alata koja čitaju i pišu u zajednički fond beleški o korisniku za kojeg je okidač pokrenut. Memorija je deljena među svim agentima u vašem tenant-u, tako da beleške agenta za trijažu utiču na odluke moderatorskog agenta. Pretraga je samo za čitanje i uvek dostupna; čuvanje je retko stavljeno iza odobrenja. Pogledajte [Sistem memorije agenta](#agent-memory-system) za ceo dizajn.

#### Get pinned comments / Get locked comments

Dva alata samo za čitanje za otkrivanje koja navode zakačene (ili zaključane) komentare na istoj stranici (`urlId`) na kojoj je okidač pokrenut. Ne uzimaju argumente — stranica se čita iz konteksta okidača, pa agent ne može da preusmeri akciju na druge stranice. Koristite ih kada agent treba da deluje na komentar koji je već zakačen ili zaključan — obično kao prvi poziv pre `unpin_comment` ili `unlock_comment`, ili pre zakačenja novog komentara kako bi se postojeći mogao prvo otkačiti.

Svaki alat se zasebno dopušta u sekciji **Dozvoljeni pozivi alata** (admin čekira `List pinned comments on the current page` ili `List locked comments on the current page`). Ne mogu biti stavljeni iza odobrenja — alati samo za čitanje nemaju sporedni efekat koji bi trebalo odobriti. Pozivanje ovih alata se ne beleži kao akcija u istoriji izvršavanja; samo rezultujući poziv `unpin_comment` / `unlock_comment` / `pin_comment` (ako postoji) se pojavljuje. Lista je ograničena na najnovijih 20 podudaranja po pozivu.

Važno za razumeti: kada jedan od ovih alata vrati commentId, taj commentId se dodaje u opseg agenta za to pokretanje, tako da sledeći poziv `unpin_comment` / `unlock_comment` prolazi validaciju protiv bezbednosne provere cilja alata na platformi. Bez prethodnog poziva alata za otkrivanje, agent ne može delovati na komentare koji nisu već u neposrednom opsegu okidača. Dakle, agent koji radi otkačivanje obično ima oba alata omogućena (npr. `get_pinned_comments` plus `unpin_comment`).

#### Upozori korisnika

Šalje privatnu DM opomenu korisniku u vezi sa određenim komentarom, i atomarno beleži upozorenje u memoriju agenta. Politika eskalacije platforme je izgrađena oko ovog alata — prvo upozori, banuj samo ako korisnik ponovi prekršaj. Pogledajte [Upozori korisnika](#tool-warn-user) za celu stranicu.

#### Banovanje korisnika

Najkonsekventniji alat koji agent može pozvati. Bannuje korisnika na fiksni period, opciono kao shadow ban, opciono i banovanjem IP-a, opciono i brisanjem svih korisnikovih komentara. Dve destruktivne opcije (IP, delete-all) su stavljene iza dodatnih opt-inova na formi za izmenu. U EU regionu, sva banovanja zahtevaju ljudsko odobrenje (vidi [Usklađenost sa EU DSA Članom 17](#eu-dsa-compliance)). Pogledajte [Ban user](#tool-ban-user) za celu stranicu.

### Podopcije alata za banovanje

Alat Ban izlaže dve destruktivne opcije — delete-all-comments i ban-by-IP — koje su potpuno sakrivene modelu dok ih ne uključite preko sekcije **Ban options** na formi za izmenu. Čak i ako model halucinira parametar, platforma odbija vrednosti u koje niste eksplicitno pristali. Pogledajte [Ban user](#tool-ban-user).