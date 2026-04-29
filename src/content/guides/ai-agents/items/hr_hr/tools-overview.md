Alati agenta su radnje koje može poduzeti. Obrazac za uređivanje agenta ima odjeljak **Dopušteni pozivi alata** u kojem označite alate koje agent smije koristiti, i odjeljak **Odobrenja** u kojem označite radnje koje bi trebale zahtijevati ljudsko odobrenje prije nego stupe na snagu.

Postoje tri razine za svaki alat:

- **Nedopušteno** - agent ga ne može vidjeti niti koristiti.
- **Dopušteno, bez odobrenja** - agent ga koristi izravno. Zapisano u povijesti izvršavanja.
- **Dopušteno, s odobrenjem** - zahtjev agenta stavlja se u red za ljudsku provjeru i izvršava se tek kada ga čovjek odobri.

Nedopušteni alati su tihi: agent ih ne može zatražiti i platforma ih odbija bez okolišanja. Alati koji zahtijevaju odobrenje uvijek prolaze kroz [inbox za odobrenja](#approval-workflow).

### Revizijski zapis za svaku radnju

Svaka radnja koju agent izvrši bilježi se s kratkim opravdanjem (1–2 rečenice koje objašnjavaju zašto) i ocjenom pouzdanosti (0.0–1.0). Oba se prikazuju u [Prikaz detalja izvršavanja](#run-detail-view) i na svakom [odobrenju](#approval-workflow). Pretraživanje memorije je jedini izuzetak samo za čitanje: ne bilježi se kao radnja i uvijek je dostupno bez obzira na dopuštenu listu.

### Referenca alata

#### Objavljivanje komentara

Omogućuje agentu da objavi komentar u svoje ime. Komentar se javno prikazuje pod prikaznim imenom agenta. Koriste ga agenti za pozdravljanje i sažimanje. Povratno je moguće - svaki moderator može ukloniti loš komentar. Obično dopušteno bez odobrenja; stavite ga iza odobrenja ako vaša zajednica zahtijeva da svaka poruka usmjerena prema javnosti bude pregledana od strane čovjeka.

#### Uređivanje komentara

Omogućuje agentu da prepiše tekst komentara koji je obuhvaćen opsegom. Izvorni tekst se čuva u revizijskom zapisu komentara. Ostavite za uske slučajeve - brisanje PII koje je korisnik otkrio, ili ispravljanje prethodnog odgovora agenta. Nije za prepravljanje mišljenja ili ublažavanje tona. **Snažno razmotrite stavljanje iza odobrenja.** Pogledajte [Uređivanje komentara](#tool-edit-comment) za cijelu stranicu.

#### Glasanje na komentarima

Omogućuje agentu da glasuje za ili protiv komentara. Glas se računa u ukupnom broju glasova komentara kao i svaki drugi glas. Većina zajednica više voli da botovi ne glasaju; nije omogućeno ni u jednom početnom predlošku. Ako to dopustite, glasanje je reverzibilno.

#### Pričvrsti / odznači komentar

Omogućuje agentu da pričvrsti komentar na vrh stranice ili odznači već pričvršćeni komentar. Platforma ne provodi pravilo jedne pričvršćenosti po niti, pa agent koji pričvršćuje treba biti uputen da prvo odznači prethodno pričvršćeni komentar. Koristi se u predlošku Top Comment Pinner. Reverzibilno; obično dopušteno bez odobrenja.

#### Zaključaj / otključaj komentar

Omogućuje agentu da spriječi daljnje odgovore ispod komentara ili obnovi mogućnost odgovaranja. Zaključani komentar ostaje vidljiv. Korisno za hlađenje žustrnih niti, u paru s odgođenim otključavanjem. Reverzibilno, ali vidljivo vašoj zajednici; razmislite o stavljanju iza odobrenja u visokorizičnim zajednicama.

#### Označi / ukloni oznaku spama

Omogućuje agentu da označi komentar kao spam (skrivanje od čitatelja i hranjenje klasifikatora spama) ili ukloni tu oznaku. Temeljni alat za svaki moderacijski agent. Reverzibilno. Snažno razmotrite stavljanje iza odobrenja tijekom prvih tjedana dok gradite povjerenje u agenta.

#### Odobri / opozovi odobrenje komentara

Omogućuje agentu da prikaže zadržani komentar čitateljima ili sakrije već vidljiv komentar. Najkorisnije na tenantima koji zadržavaju nove komentare za pregled moderatora. Veliki su ulozi kada se opoziva odobrenje vidljivog komentara - razmislite o stavljanju iza odobrenja.

#### Označi komentar kao pregledan

Alat za stanje reda: označava komentar kao "moderator (ili agent) je to pregledao." Ne mijenja vidljivost. Niski ulozi; rijetko se stavlja iza odobrenja.

#### Dodijeli značku

Omogućuje agentu da korisniku dodijeli značku iz konfiguracije znački vašeg tenanta. Reverzibilno od strane moderatora. Rijetko se stavlja iza odobrenja. Agent mora poznavati ID značke, zato uključite relevantne ID-e u vaše [smjernice zajednice](#community-guidelines) ili [početni prompt](#personality-prompt).

#### Slanje e-pošte

Omogućuje agentu da pošalje tekstualnu e-poruku iz `noreply@fastcomments.com` na adresu koju odabere. Koristite štedljivo - e-pošta je alat s najvećim trenjem i loše poslane poruke teško se poništavaju. Snažno razmotrite stavljanje iza odobrenja i usmjerite odobrenja e-pošte onome tko upravlja inboxom kojem će agent na kraju slati poruke.

#### Spremi / pretraži agentovu memoriju

Dva udružena alata koja čitaju i zapisuju zajedničku bazu bilješki o korisniku zbog kojeg je pokrenut okidač. Memorija se dijeli među svim agentima u vašem tenantu, pa bilješke trijažnog agenta informiraju odluke moderatorskog agenta. Pretraživanje je samo za čitanje i uvijek dostupno; spremanje se rijetko stavlja iza odobrenja. Pogledajte [Sustav memorije agenta](#agent-memory-system) za puni dizajn.

#### Upozori korisnika

Šalje privatnu DM opomenu korisniku u vezi konkretnog komentara, i atomski bilježi opomenu u memoriji agenta. Politika eskalacije platforme izgrađena je oko ovog alata - prvo upozori, zabrani samo ako korisnik ponovi prekršaj. Rjeđe se stavlja iza odobrenja nego `ban_user`, ali razmotrite stavljanje iza odobrenja tijekom prvih tjedana života agenta. Pogledajte [Upozori korisnika](#tool-warn-user) za cijelu stranicu.

#### Zabrani korisnika

Najozbiljniji alat koji agent može pozvati. Zabrani korisnika na određeno trajanje, opcionalno kao shadow ban, opcionalno također zabrani IP, opcionalno također obriši sve korisnikove komentare. Dvije destruktivne opcije (IP, delete-all-comments) stavljene su iza dodatnih opcija pristanka na obrascu za uređivanje. U EU regiji, sve zabrane zahtijevaju ljudsko odobrenje (vidi [Usklađenost s EU DSA člankom 17](#eu-dsa-compliance)). Snažno razmotrite stavljanje iza odobrenja svugdje. Pogledajte [Zabrani korisnika](#tool-ban-user) za cijelu stranicu.

### Podopcije alata za zabranu

Alat za zabranu izlaže dvije destruktivne opcije - delete-all-comments i ban-by-IP - koje su potpuno skrivene modelu dok ih ne uključite preko odjeljka **Opcije zabrane** na obrascu za uređivanje. Čak i ako model halucinira parametar, platforma odbija vrijednosti u koje niste pristali. Pogledajte [Zabrani korisnika](#tool-ban-user).

---