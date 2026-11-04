Alati agenta su radnje koje on može da preduzme. Forma za izmenu agenta ima sekciju **Allowed tool calls** u kojoj čekirate alate koje ovaj agent sme da koristi, i sekciju **Approvals** u kojoj čekirate radnje koje zahtevaju odobrenje čoveka pre nego što stupe na snagu.

Postoje tri nivoa za svaki alat:

- **Nije dozvoljen** - agent ne može da ga vidi ni da ga koristi.
- **Dozvoljen, bez odobrenja** - agent ga koristi direktno. Beleži se u istoriji izvršavanja.
- **Dozvoljen, uz odobrenje** - poziv agenta se stavlja u red za ljudski pregled i izvršava se tek kada čovek odobri.

Nedozvoljeni alati su tihi: agent ne može da ih traži, a platforma ih odmah odbija. Alati sa odobrenjem uvek prolaze kroz [inbox odobrenja](#approval-workflow).

### Revizorski trag za svaku radnju

Svaka radnja agenta se beleži sa kratkim obrazloženjem (1-2 rečenice koje objašnjavaju zašto) i ocenom poverenja (0.0-1.0). Oba se prikazuju u [Run Detail View](#run-detail-view) i na svakom [odobrenju](#approval-workflow). Pretraga memorije je jedini izuzetak koji je samo za čitanje: ne beleži se kao radnja i uvek je dostupna bez obzira na listu dozvoljenih alata.

### Referenca alata

#### Objavljivanje komentara

Omogućava agentu da objavi komentar u svoje ime. Komentar se javno prikazuje pod imenom za prikaz agenta. Koriste ga greeter i summarizer agenti. Reverzibilno - bilo koji moderator može da ukloni loš komentar. Stavite iza odobrenja ako vaša zajednica zahteva da svaka javna poruka bude pregledana od strane čoveka.

#### Izmena komentara

Omogućava agentu da prepravi tekst komentara koji je u obimu. Originalni tekst se čuva u revizorskom dnevniku komentara. Rezervišite za uske slučajeve - uklanjanje PII podataka koje je korisnik otkrio, ili ispravku prethodnog odgovora samog agenta. Nije za prepravljanje mišljenja ili ublažavanje tona. Pogledajte [Edit comment](#tool-edit-comment) za celu stranicu.

#### Glasanje o komentarima

Omogućava agentu da glasa za ili protiv komentara. Glas se računa u ukupan zbir glasova komentara kao i svaki drugi. Većina zajednica ne želi da botovi glasaju; nije omogućeno ni u jednom starter šablonu. Ako ipak dozvolite, glasanje je reverzibilno.

#### Kačenje / okačivanje komentara

Omogućava agentu da zakači komentar na vrh stranice ili da skine komentar koji je već zakačen. Platforma ne sprovodi pravilo jednog zakačenog komentara po temi, pa agent koji kači treba da dobije instrukciju da prvo skine prethodno zakačeni komentar. Da bi otkrio šta je već zakačeno na istoj stranici, agent može da pozove alat samo za čitanje `get_pinned_comments` (vidi dole). Koristi ga šablon Top Comment Pinner.

#### Zaključavanje / otključavanje komentara

Omogućava agentu da spreči dalje odgovore ispod komentara, ili da ih vrati. Zaključani komentar ostaje vidljiv. Korisno za hlađenje situacije u zagrejanim temama, u paru sa odloženim otključavanjem. Da bi otkrio šta je trenutno zaključano na istoj stranici, agent može da pozove alat samo za čitanje `get_locked_comments` (vidi dole).

#### Označavanje / skidanje oznake spam

Omogućava agentu da označi komentar kao spam (sakriva ga od čitalaca i hrani spam klasifikator) ili da ukloni tu oznaku. Osnovni alat za svaki moderacioni agent. Reverzibilno.

#### Odobravanje / povlačenje odobrenja komentara

Omogućava agentu da prikaže čekaći komentar čitaocima, ili da sakrije već vidljiv. Najkorisnije na tenantima koji drže nove komentare za pregled moderatora.

#### Označavanje komentara kao pregledanog

Alat za stanje reda: označava komentar kao "moderator (ili agent) je ovo pogledao." Ne menja vidljivost. Nizak rizik; retko se stavlja iza odobrenja.

#### Dodela bedža

Omogućava agentu da da korisniku bedž koji ste konfigurisali za vaš tenant. Reverzibilno od strane moderatora. Kada je ovaj alat omogućen, agent može da vidi bedževe vašeg tenanta i sam izabere pravi, tako da ne morate da nalepite identifikatore bedževa u smernice zajednice ili početni prompt. Da biste usmerili koji bedž se dodeljuje za koje ponašanje, pozivajte se na bedževe preko njihovog **Display Label**-a u promptu.

#### Slanje emaila

Omogućava agentu da pošalje email u običnom tekstu autoru komentara u obimu okidača. Agent nikada ne vidi email adresu primaoca - bira komentar, a platforma dostavlja na adresu koju je taj komentator ostavio prilikom objavljivanja. Adresa pošiljaoca je brendirani sender vašeg tenanta (sa DKIM) kada se domen komentara poklapa sa konfigurisanim domenom, inače platforma koristi podrazumevanu. Koristite štedljivo - email je alat sa najvećim trenjem, a loši emailovi se teško poništavaju.

#### Čuvanje / pretraga memorije agenta

Dva uparena alata koji čitaju i pišu u zajednički bazen beleški o korisniku za kog se okinuo okidač. Memorija je deljena između svih agenata u vašem tenantu, pa beleške triage agenta informišu odluke moderator agenta. Pretraga je samo za čitanje i uvek dostupna; čuvanje se retko stavlja iza odobrenja. Pogledajte [Agent Memory System](#agent-memory-system) za ceo dizajn.

#### Get pinned comments / Get locked comments

Dva alata za otkrivanje samo za čitanje koji izlistavaju zakačene (ili zaključane) komentare na istoj stranici (`urlId`) na kojoj se okinuo okidač. Ne primaju argumente - stranica se čita iz konteksta okidača, pa agent ne može da pređe na druge stranice. Koristite ih kada agent treba da deluje na komentar koji je već zakačen ili zaključan - tipično prvi poziv pre `unpin_comment` ili `unlock_comment`, ili pre kačenja novog komentara da bi se postojeći prvo skinuo.

Svaki alat se posebno čekira u **Allowed tool calls** (admin čekira `List pinned comments on the current page` ili `List locked comments on the current page`). Ne mogu se staviti iza odobrenja - alati samo za čitanje nemaju efekat koji bi se odobrio. Njihovo pozivanje se ne beleži kao radnja u istoriji izvršavanja; prikazuje se samo rezultujući poziv `unpin_comment` / `unlock_comment` / `pin_comment` (ako ga ima). Lista je ograničena na 20 najnovijih podudaranja po pozivu.

Važno za razumevanje: kada jedan od ovih alata vrati commentId, taj commentId se dodaje u obim agenta za taj run, tako da naredni poziv `unpin_comment` / `unlock_comment` prolazi proveru bezbednosti cilja alata na platformi. Bez prethodnog poziva alata za otkrivanje, agent ne može da deluje na komentare koji nisu već u neposrednom obimu okidača. Zato agent tipa "unpin" obično dobija oba alata omogućena (npr. `get_pinned_comments` plus `unpin_comment`).

#### Upozorenje korisniku

Šalje privatni DM upozorenje korisniku u vezi sa konkretnim komentarom, i atomično beleži upozorenje u memoriju agenta. Politika eskalacije platforme je izgrađena oko ovog alata - prvo upozorenje, ban tek ako korisnik ponovi prekršaj. Pogledajte [Warn user](#tool-warn-user) za celu stranicu.

#### Ban korisnika

Najozbiljniji alat koji agent može da pozove. Banuje korisnika sa fiksnim trajanjem, opciono kao shadow ban, opciono i banovanje IP adrese, opciono i brisanje svih komentara korisnika. Dve destruktivne opcije (IP, delete-all) su stavljene iza dodatnih opt-in opcija na formi za izmenu. U EU regionu svi banovi zahtevaju ljudsko odobrenje (vidi [EU DSA Article 17 Compliance](#eu-dsa-compliance)). Pogledajte [Ban user](#tool-ban-user) za celu stranicu.

### Podopcije Ban alata

Ban alat izlaže dve destruktivne opcije - delete-all-comments i ban-by-IP - koje su potpuno skrivene od modela dok ih ne uključite preko sekcije **Ban options** na formi za izmenu. Čak i ako model halucinira parametar, platforma odbija vrednosti koje niste uključili. Pogledajte [Ban user](#tool-ban-user).