Agentovi alati su akcije koje može preduzeti. Formular za uređivanje agenta ima sekciju **Allowed tool calls** gde označite alate koje je ovom agentu dozvoljeno da koristi, i sekciju **Approvals** gde označite akcije koje treba da zahtevaju ljudsko odobrenje pre nego što stupe na snagu.

Postoje tri nivoa za bilo koji alat:

- **Disallowed** - agent ga ne može videti niti koristiti.
- **Allowed, no approval** - agent ga koristi direktno. Zabeleženo u istoriji izvršavanja.
- **Allowed, with approval** - poziv agenta se stavlja u red za ljudsku reviziju i izvršava se samo kada ga čovek odobri.

Zabranjeni alati su tihi: agent ih ne može tražiti i platforma ih odmah odbija. Alati koji zahtevaju odobrenje uvek prolaze kroz [approvals inbox](#approval-workflow).

### Audit trail on every action

Svaka akcija koju agent preduzme je zabeležena sa kratkim obrazloženjem (1–2 rečenice koje objašnjavaju zašto) i skorom poverenja (0.0–1.0). Obe informacije se pojavljuju u [Run Detail View](#run-detail-view) i na svakom [approval](#approval-workflow). Pretraživanje memorije je jedini izuzetak u režimu samo-za-čitati: ono se ne beleži kao akcija i uvek je dostupno bez obzira na allowlist.

### Tool reference

#### Posting comments

Dozvoljava agentu da postavi komentar kao on sam. Komentar se javno prikazuje pod prikazanim imenom agenta. Koristi se kod greeter i summarizer agenata. Povratno izvodljivo - bilo koji moderator može ukloniti loš komentar. Stavite iza odobrenja ako vaša zajednica zahteva da svaka poruka okrenuta javnosti bude pregleda od strane čoveka.

#### Editing a comment

Dozvoljava agentu da prepiše tekst komentara koji je u opsegu. Originalni tekst se čuva u audit logu komentara. Ograničite za uske slučajeve - brisanje PII koje je korisnik nenamerno otkrio, ili ispravka sopstvenog prethodnog odgovora agenta. Nije za prepisivanje mišljenja ili ublažavanje tona. Pogledajte [Edit comment](#tool-edit-comment) za celu stranicu.

#### Voting on comments

Dozvoljava agentu da glasa za ili protiv komentara. Glas se računa u ukupni broj glasova komentara kao i bilo koji drugi glas. Većina zajednica radije nema botove koji glasaju; nije omogućen ni u jednom starter šablonu. Ako dozvolite, glasanje je povratno izvodljivo.

#### Pin / unpin a comment

Dozvoljava agentu da zakači komentar na vrh stranice ili da odkači komentar koji je već zakačen. Platforma ne nameće pravilo jedan-zakačen-po-temi, tako da agent koji zakačuje treba da mu se naredi da prvo odkači prethodno zakačeni komentar. Da otkrije šta je već zakačeno na istoj stranici, agent može pozvati read-only `get_pinned_comments` alat (vidi dole). Koristi se u Top Comment Pinner template.

#### Lock / unlock a comment

Dozvoljava agentu da spreči dalje odgovore ispod komentara, ili da obnovi mogućnost odgovora. Zaključani komentar ostaje vidljiv. Korisno za hlađenje napetih tema, u paru sa odloženim otključavanjem. Da otkrije šta je trenutno zaključano na istoj stranici, agent može pozvati read-only `get_locked_comments` alat (vidi dole).

#### Mark / unmark spam

Dozvoljava agentu da označi komentar kao spam (sakrivajući ga od čitaoca i napajajući spam klasifikator) ili da ukloni tu oznaku. Osnovni alat za bilo kog agenta za moderaciju. Povratno izvodljivo.

#### Approve / un-approve a comment

Dozvoljava agentu da prikaže zadržani komentar čitaocima, ili da sakrije već vidljiv komentar. Najkorisnije na tenantima koji zadržavaju nove komentare za moderatorski pregled.

#### Mark a comment reviewed

Alat za stanja reda: označava komentar kao „moderator (ili agent) je pogledao ovo.“ Ne menja vidljivost. Niskog je rizika; retko se postavlja iza odobrenja.

#### Award a badge

Dozvoljava agentu da dodeli korisniku bedž koji ste konfigurisali za vaš tenant. Povratno izvodljivo od strane moderatora. Kada je ovaj alat omogućen, agent može videti bedževe vašeg tenant-a i sam izabrati odgovarajući, tako da ne morate da lepite identifikatore bedževa u smernice zajednice ili početni prompt. Da usmerite koji bedž se dodeljuje za koje ponašanje, referencirajte bedževe po njihovom **Display Label** u promptu.

#### Send email

Dozvoljava agentu da pošalje plain-text email autoru komentara koji je u opsegu trigera. Agent nikada ne vidi email adresu primaoca — on izabere komentar, a platforma isporučuje na adresu koju je taj komentator ostavio prilikom objave. From-adresa je brendirani pošiljalac vašeg tenant-a (sa DKIM) kada domen komentara odgovara konfigurisanom domenu, inače se koristi podrazumevano od platforme. Koristite štedljivo — email je alat sa najvećim trenjem i loše poslate poruke je teško povratiti.

#### Save / search agent memory

Dva povezana alata koji čitaju i upisuju zajednički pool beleški o korisniku za kog je triger aktiviran. Memorija je deljena među svim agentima u vašem tenant-u, tako da beleške trijažnog agenta informišu odluke moderatorskog agenta. Pretraga je read-only i uvek dostupna; čuvanje se retko stavlja iza odobrenja. Pogledajte [Agent Memory System](#agent-memory-system) za pun dizajn.

#### Get pinned comments / Get locked comments

Dva read-only alata za otkrivanje koja izlistavaju zakačene (ili zaključane) komentare na istoj stranici (`urlId`) na kojoj je triger aktiviran. Ne uzimaju argumente — stranica se čita iz konteksta trigera, tako da agent ne može da preusmeri na druge stranice. Koristite ih kada agent treba da deluje na komentar koji je već zakačen ili zaključan — tipično prvi poziv pre `unpin_comment` ili `unlock_comment`, ili pre zakačivanja novog komentara kako bi se prethodni prvo mogao odkačiti.

Svaki alat se zasebno dozira u **Allowed tool calls** (administrator označava `List pinned comments on the current page` ili `List locked comments on the current page`). Ne mogu biti stavljeni iza odobrenja - read-only alati nemaju nuspojava koju bi trebalo odobriti. Pozivanje njih se ne beleži kao akcija u istoriji izvršavanja; samo rezultatni `unpin_comment` / `unlock_comment` / `pin_comment` poziv (ako postoji) se pojavi. Lista je ograničena na najnovijih 20 podudaranja po pozivu.

Važno za razumevanje: kada jedan od ovih alata vrati commentId, taj commentId se dodaje u per-run opseg agenta, pa se naredni `unpin_comment` / `unlock_comment` poziv validira protiv platforminog bezbednosnog provere cilja alata. Bez prethodnog pozivanja alata za otkrivanje, agent ne može delovati na komentare koji nisu već u neposrednom opsegu trigera. Dakle, agent tipa za odkačivanje obično dobija oba alata omogućena (npr. `get_pinned_comments` plus `unpin_comment`).

#### Warn a user

Šalje privatnu DM opomenu korisniku u vezi konkretnog komentara, i atomski beleži opomenu u memoriji agenta. Politika eskalacije platforme je izgrađena oko ovog alata - prvo opomeni, zabrani samo ako korisnik ponovi prekršaj. Pogledajte [Warn user](#tool-warn-user) za celu stranicu.

#### Ban a user

Najteža posledica koju agent može pozvati. Zabrani korisnika na fiksni period, opcionalno kao shadow ban, opcionalno i zabrana IP-a, opcionalno i brisanje svih korisnikovih komentara. Dve destruktivne opcije (IP, delete-all) su stavljene iza dodatnih opt-inova na formularu za uređivanje. U EU regionu, sve zabrane zahtevaju ljudsko odobrenje (vidi [EU DSA Article 17 Compliance](#eu-dsa-compliance)). Pogledajte [Ban user](#tool-ban-user) za celu stranicu.

### Ban-tool sub-options

Ban alat otkriva dve destruktivne opcije - delete-all-comments i ban-by-IP - koje su potpuno skrivene modelu dok ih ne uključite putem sekcije **Ban options** na formularu za uređivanje. Čak i ako model halucinira parametar, platforma odbija vrednosti koje niste uključili. Pogledajte [Ban user](#tool-ban-user).