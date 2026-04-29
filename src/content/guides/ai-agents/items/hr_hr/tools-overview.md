Agentovi **alat(i)** su radnje koje može poduzeti. Na obrascu za uređivanje agenta postoji odjeljak **Dozvoljeni pozivi alata** u kojem označite alate koje ovaj agent smije koristiti, i odjeljak **Odobrenja** u kojem označite radnje koje bi trebale zahtijevati da ih čovjek odobri prije nego što stupe na snagu.

Postoje tri razine za bilo koji alat:

- **Zabranjeno** - agent ga ne vidi niti smije koristiti.
- **Dozvoljeno, bez odobrenja** - agent ga koristi izravno. Zapisuje se u povijest pokretanja.
- **Dozvoljeno, uz odobrenje** - poziv agenta se stavlja u red za ljudsku provjeru i izvršava se tek kada ga čovjek odobri.

Zabranjeni alati su tihi: agent ih ne može tražiti i platforma ih odmah odbija. Alati koji zahtijevaju odobrenje uvijek prolaze kroz [inbox za odobrenja](#approval-workflow).

### Revizijski trag za svaku radnju

Svaka radnja koju agent poduzme bilježi se s kratkim opravdanjem (1–2 rečenice koje objašnjavaju zašto) i ocjenom pouzdanosti (0.0–1.0). Oboje se pojavljuje u [Run Detail View](#run-detail-view) i na svakom [odobrenju](#approval-workflow). Pretraživanje memorije je jedini izuzetak koji je samo za čitanje: ono se ne zapisuje kao radnja i uvijek je dostupno bez obzira na bijelu listu.

### Referenca alata

#### Objavljivanje komentara

Omogućuje agentu da objavi komentar u svoje ime. Komentar se javno prikazuje pod prikaznim imenom agenta. Koristi se za agente dočekivače i sažimače. Povratno je - svaki moderator može ukloniti loš komentar. Obično se dopušta bez odobrenja; postavite ograničenje ako vaša zajednica zahtijeva da svaka javna poruka bude pregledana od strane čovjeka.

#### Uređivanje komentara

Omogućuje agentu da preformulira tekst komentara koji je u opsegu. Izvorni tekst se čuva u revizijskom zapisu komentara. Ostavite za uske slučajeve - brisanje PII koji je korisnik otkrio ili ispravljanje vlastitog prethodnog odgovora agenta. Nije namijenjeno za prepravljanje mišljenja ili ublažavanje tona. **Snažno razmislite o stavljanju iza odobrenja.** Pogledajte [Edit comment](#tool-edit-comment) za cjelovitu stranicu.

#### Glasovanje o komentarima

Omogućuje agentu da glasuje za ili protiv komentara. Glas se računa u ukupan broj glasova komentara kao i svaki drugi glas. Većina zajednica radije nema botove koji glasaju; nije omogućeno ni u jednom početnom predlošku. Ako to dopustite, glasanje je povratno.

#### Pričvrstiti / otkačiti komentar

Omogućuje agentu da pričvrsti komentar na vrh stranice ili otkači onaj koji je već pričvršćen. Platforma ne nameće pravilo jednog pričvršćenog komentara po niti, pa agent za pričvršćivanje treba biti uputljen da prvo otkači prethodno pričvršćeni komentar. Koristi se u predlošku Top Comment Pinner. Povratno; obično dopušteno bez odobrenja.

#### Zaključati / otključati komentar

Omogućuje agentu da spriječi daljnje odgovore ispod komentara, ili vrati mogućnost odgovora. Zaključani komentar ostaje vidljiv. Korisno za hlađenje zagrijanih niti, upareno s odgođenim otključavanjem. Povratno, ali vidljivo vašoj zajednici; razmotrite stavljanje iza odobrenja u zajednicama visokog rizika.

#### Označiti / ukloniti oznaku neželjene pošte

Omogućuje agentu da označi komentar kao spam (sakriva ga od čitatelja i prosljeđuje u klasifikator spama) ili da ukloni tu oznaku. Osnovni alat za bilo kojeg moderacijskog agenta. Povratno. Snažno razmislite o stavljanju iza odobrenja tijekom prvih tjedana dok ne izgradite povjerenje u agenta.

#### Odobriti / opozvati odobrenje komentara

Omogućuje agentu da prikaže zadržani komentar čitateljima, ili sakrije već vidljiv komentar. Najkorisnije na tenantima koji drže nove komentare na pregledu moderatora. Veliki rizik pri opozivu vidljivog komentara - razmotrite stavljanje iza odobrenja.

#### Označiti komentar kao pregledan

Alat za stanje u redu: označava komentar kao "moderator (ili agent) je pogledao ovo." Ne mijenja vidljivost. Mali rizik; rijetko se stavlja iza odobrenja.

#### Dodijeliti bedž

Omogućuje agentu da korisniku dâ bedž koji ste konfigurirali za svoj tenant. Može ga opozvati moderator. Rijetko stavljen iza odobrenja. Kad je ovaj alat omogućen, agent vidi bedževe vašeg tenanta i sam odabire odgovarajući, pa ne morate lijepiti identifikatore bedževa u smjernice zajednice ili početni prompt. Ako želite usmjeriti koji se bedž dodjeljuje za koje ponašanje, pozovite se na bedževe po njihovom **Display Label** u promptu.

#### Slanje e-pošte

Omogućuje agentu da pošalje tekstualni e-mail autoru komentara koji je u opsegu okidača. Agent nikada ne vidi adresu primatelja - odabire komentar, a platforma isporučuje na adresu koju je taj komentator ostavio prilikom objave. Adresa pošiljatelja je brendirani pošiljatelj vašeg tenanta (s DKIM-om) kada domena komentara odgovara konfiguriranoj domeni, u suprotnom se koristi zadana platforma. Koristite umjereno - e-pošta je alat s najvećim otporom, a loše e-poruke je teško poništiti. Snažno razmislite o stavljanju iza odobrenja i usmjerite odobrenja e-pošte prema osobi koja upravlja inboxom kojem će agent zapravo slati.

#### Spremiti / pretražiti memoriju agenta

Dva povezana alata koja čitaju i zapisuju zajednički skup bilješki o korisniku za kojeg je okidač aktiviran. Memorija je zajednička svim agentima u vašem tenant-u, pa bilješke agenta za triage utječu na odluke moderatora. Pretraživanje je samo za čitanje i uvijek dostupno; spremanje se rijetko stavlja iza odobrenja. Pogledajte [Agent Memory System](#agent-memory-system) za cjeloviti dizajn.

#### Upozoriti korisnika

Šalje privatnu DM opomenu korisniku u vezi određenog komentara i atomski bilježi opomenu u memoriji agenta. Politika eskalacije platforme izgrađena je oko ovog alata - prvo upozori, zabrani tek ako korisnik ponovi prekršaj. Rjeđe se stavlja iza odobrenja nego `ban_user`, ali razmotrite stavljanje iza odobrenja tijekom prvih tjedana rada agenta. Pogledajte [Warn user](#tool-warn-user) za cjelovitu stranicu.

#### Zabraniti korisnika

Najkonzekventniji alat koji agent može pozvati. Zabrani korisnika na određeno trajanje, opcionalno kao shadow ban, opcionalno također zabrani IP, opcionalno također izbriše sve korisnikove komentare. Dvije destruktivne opcije (IP, delete-all-comments) su stavljene iza dodatnih pristanka na obrascu za uređivanje. U regiji EU, sve zabrane zahtijevaju ljudsko odobrenje (pogledajte [EU DSA Article 17 Compliance](#eu-dsa-compliance)). Snažno razmislite o stavljanju iza odobrenja svugdje. Pogledajte [Ban user](#tool-ban-user) za cjelovitu stranicu.

### Pod-opcije alata za zabranu

Alat Ban izlaže dvije destruktivne opcije - delete-all-comments i ban-by-IP - koje su modelu u potpunosti skrivene dok ih ne uključite putem odjeljka **Ban options** na obrascu za uređivanje. Čak i ako model halucinira parametar, platforma odbija vrijednosti koje niste uključili. Pogledajte [Ban user](#tool-ban-user).