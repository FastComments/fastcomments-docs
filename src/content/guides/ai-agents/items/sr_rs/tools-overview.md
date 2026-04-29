Agentovi **alati** su akcije koje može preduzeti. Obrazac za uređivanje agenta ima odeljak **Dozvoljeni pozivi alata** gde štiklirate alate koje je agentu dozvoljeno da koristi, i odeljak **Odobrenja** gde štiklirate akcije koje bi morale da budu odobrene od strane čoveka pre nego što stupe na snagu.

Postoje tri nivoa za bilo koji alat:

- **Zabranjeno** - agent ga ne može videti niti koristiti.
- **Dozvoljeno, bez odobrenja** - agent ga koristi direktno. Zabeleženo je u istoriji izvršavanja.
- **Dozvoljeno, uz odobrenje** - poziv agenta je stavljen u red za ljudsku reviziju i izvršava se samo kada ga čovek odobri.

Zabranjeni alati su nevidljivi: agent ih ne može tražiti i platforma ih odmah odbacuje. Alati koji zahtevaju odobrenje uvek prolaze kroz [inbox za odobrenja](#approval-workflow).

### Revizorski zapis za svaku akciju

Svaka akcija koju agent preduzme se beleži sa kratkim opravdanjem (1–2 rečenice koje objašnjavaju zašto) i skorom poverenja (0.0–1.0). Obe informacije se pojavljuju u [Pregledu detalja izvršavanja](#run-detail-view) i na svakom [odobrenju](#approval-workflow). Pretraživanje memorije je jedini izuzetak koji je samo za čitanje: ono se ne beleži kao akcija i uvek je dostupno bez obzira na listu dozvoljenih.

### Referenca alata

#### Objavljivanje komentara

Dozvoljava agentu da objavi komentar kao on sam. Komentar se javno prikazuje pod prikazanim imenom agenta. Koristi ga agenti za pozdravljanje i za sažimanje. Moguće opozvati — bilo koji moderator može ukloniti loš komentar. Obično se dozvoljava bez odobrenja; zahtevajte odobrenje ako vaša zajednica zahteva da svaka javna poruka bude pregledana od strane čoveka.

#### Izmena komentara

Dozvoljava agentu da prepiše tekst komentara koji je u opsegu. Originalni tekst se čuva u revizorskom zapisu komentara. Ostavite za uske slučajeve — brisanje otkrivenih PII koje je korisnik otkrio, ili ispravka prethodnog odgovora agenta. Nije za prepravljanje stavova ili ublažavanje tona. **Snažno razmislite o stavljanju iza odobrenja.** Pogledajte [Uredi komentar](#tool-edit-comment) za celu stranicu.

#### Glasanje na komentarima

Dozvoljava agentu da glasа za ili protiv komentara. Glas se računa u ukupan broj glasova komentara kao i svaki drugi glas. Većina zajednica ne preferira da botovi glasaju; nije omogućeno ni u jednom početnom šablonu. Ako to dozvolite, glasanje se može opozvati.

#### Pinovanje / uklanjanje pinovanja komentara

Dozvoljava agentu da pinuje komentar na vrh stranice ili da ukloni pin sa već pinovanog komentara. Platforma ne nameće pravilo jedan-pin-po-niti, tako da agent za pinovanje treba da bude uputstvo da prvo ukloni prethodno pinovani komentar. Koristi ga šablon Top Comment Pinner. Moguće opozvati; obično dozvoljeno bez odobrenja.

#### Zaključavanje / otključavanje komentara

Dozvoljava agentu da onemogući dalja odgovaranja ispod komentara ili da obnovi mogućnost odgovaranja. Zaključani komentar ostaje vidljiv. Korisno za hlađenje žustrih niti, u paru sa odloženim otključavanjem. Moguće opozvati, ali vidljivo vašoj zajednici; razmotrite zahtev za odobrenje u zajednicama sa visokim ulozima.

#### Označi / ukloni oznaku spam

Dozvoljava agentu da označi komentar kao spam (sakriva ga od čitaoca i daje podsticaj klasifikatoru spama) ili da ukloni tu oznaku. Osnovni alat za bilo kog agenta za moderaciju. Moguće opozvati. Snažno razmislite o zahtevanju odobrenja tokom prvih nedelja dok ne izgradite poverenje u agenta.

#### Odobri / opozovi odobrenje komentara

Dozvoljava agentu da prikaže zadržani komentar čitaocima ili da sakrije već vidljiv komentar. Najkorisnije na tenantima koji zadržavaju nove komentare na pregled moderatora. Visok rizik kada se opozove odobrenje već vidljivog komentara — razmotrite zahtev za odobrenje.

#### Označi komentar kao pregledan

Alat za stanje reda: označava komentar kao „moderator (ili agent) je pogledao ovo“. Ne menja vidljivost. Mali rizik; retko se stavlja iza odobrenja.

#### Dodeli značku

Dozvoljava agentu da dodeli korisniku značku iz konfiguracije značaka vašeg tenanta. Moguće opozvati od strane moderatora. Retko se stavlja iza odobrenja. Agent mora znati ID značke, zato uključite relevantne ID-e u vaše [smernice zajednice](#community-guidelines) ili [početni prompt](#personality-prompt).

#### Slanje e-pošte

Dozvoljava agentu da pošalje običan tekstualni e-mail sa `noreply@fastcomments.com` na adresu koju odabere. Koristite štedljivo — e-pošta je alat sa najvećim trenjem i loše poslate poruke je teško poništiti. Snažno razmislite o zahtevanju odobrenja, i usmerite odobrenja za e-mail ka osobi koja poseduje inbox na koji će agent slati poruke.

#### Čuvanje / pretraga memorije agenta

Dva uparena alata koja čitaju i pišu zajednički bazen beleški o korisniku za koga je okidač aktiviran. Memorija je deljena između svih agenata u vašem tenant-u, tako da beleške agenta za trijažu utiču na odluke agenta-moderatora. Pretraga je samo za čitanje i uvek dostupna; čuvanje se retko stavlja iza odobrenja. Pogledajte [Sistem memorije agenata](#agent-memory-system) za kompletan dizajn.

#### Upozori korisnika

Šalje privatnu direktnu poruku upozorenja korisniku u vezi sa određenim komentarom, i atomarno beleži upozorenje u memoriji agenta. Politika eskalacije platforme je izgrađena oko ovog alata — prvo upozorite, banovanje samo ako korisnik ponovi prestup. Ređe se stavlja iza odobrenja nego `ban_user`, ali razmotrite stavljanje iza odobrenja tokom prvih nedelja rada agenta. Pogledajte [Upozori korisnika](#tool-warn-user) za celu stranicu.

#### Zabrani korisnika

Najjači alat koji agent može pozvati. Zabrani korisnika na fiksno vreme, opciono kao shadow ban, opciono i banovanjem IP-a, opciono i brisanjem svih korisnikovih komentara. Dve destruktivne opcije (IP, delete-all-comments) su zaključane iza dodatnih opcija koje morate ručno uključiti na obrascu za uređivanje. U regionu EU, sva banovanja zahtevaju ljudsko odobrenje (pogledajte [Usklađenost sa članom 17 EU DSA](#eu-dsa-compliance)). Snažno razmislite o zahtevanju odobrenja svuda. Pogledajte [Zabrani korisnika](#tool-ban-user) za celu stranicu.

### Podopcije alata za banovanje

Alat Ban izlaže dve destruktivne opcije - delete-all-comments i ban-by-IP - koje su potpuno skrivene modelu dok ih vi ne uključite putem sekcije **Ban options** na obrascu za uređivanje. Čak i ako model halucinira parametar, platforma odbija vrednosti koje niste eksplicitno uključili. Pogledajte [Zabrani korisnika](#tool-ban-user).