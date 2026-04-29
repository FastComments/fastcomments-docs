Agentovi **alati** su akcije koje može preduzeti. Forma za uređivanje agenta ima sekciju **Allowed tool calls** gdje štiklirate alate koje je ovom agentu dozvoljeno koristiti, i sekciju **Approvals** gdje štiklirate akcije koje trebaju ljudsko odobrenje prije nego stupe na snagu.

Postoje tri nivoa za bilo koji alat:

- **Disallowed** - agent ih ne može vidjeti niti koristiti.
- **Allowed, no approval** - agent ga koristi direktno. Zabilježeno u historiji izvršavanja.
- **Allowed, with approval** - agentov poziv je stavljen u red za ljudsku provjeru i izvršava se tek kad ga čovjek odobri.

Disallowed alati su tihi: agent ih ne može zatražiti i platforma ih odmah odbija. Alati za koje je potrebna odobrenja uvijek prolaze kroz [inbox za odobrenja](#approval-workflow).

### Revizija svake akcije

Svaka akcija koju agent preduzme se bilježi sa kratkim opravdanjem (1–2 rečenice koje objašnjavaju zašto) i skorom povjerenja (0.0–1.0). Oba se prikazuju u [Run Detail View](#run-detail-view) i na svakom [odobrenju](#approval-workflow). Pretraživanje memorije je jedini izuzetak koji je samo za čitanje: ono nije zabilježeno kao akcija i uvijek je dostupno bez obzira na allowlistu.

### Referenca alata

#### Objavljivanje komentara

Omogućava agentu da objavi komentar u svoje ime. Komentar se javno prikazuje pod prikazanim imenom agenta. Koristi se kod agenata za pozdravljanje i sažimanje. Povratno — bilo koji moderator može ukloniti loš komentar. Obično dozvoljeno bez odobrenja; postavite ograničenje ako vaša zajednica zahtijeva da svaka javna poruka bude pregledana od strane čovjeka.

#### Uređivanje komentara

Omogućava agentu da prepiše tekst komentara koji je u dometu. Originalni tekst se čuva u revizijskom zapisu komentara. Ostavite za uske slučajeve — brisanje PII koji je korisnik otkrio, ili ispravka ranijeg odgovora agenta. Ne za preoblikovanje mišljenja ili ublažavanje tona. **Snažno razmotrite stavljanje iza odobrenja.** Vidi [Uredi komentar](#tool-edit-comment) za kompletnu stranicu.

#### Glasanje na komentarima

Omogućava agentu da glasa za ili protiv komentara. Glas se uračunava u ukupan broj glasova komentara kao i svaki drugi glas. Većina zajednica ne preferira botove koji glasaju; nije omogućeno ni u jednom starter šablonu. Ako dozvolite, glasanje je reverzibilno.

#### Prikvačiti / otkačiti komentar

Omogućava agentu da prikvači komentar na vrh stranice ili otkači već prikvačeni komentar. Platforma ne nameće pravilo jednog prikvačenog komentara po temi, pa agent koji prikvačuje treba biti uputen da prvo otkači prethodni prikvačeni komentar. Koristi se u Top Comment Pinner šablonu. Reverzibilno; obično dozvoljeno bez odobrenja.

#### Zaključaj / otključaj komentar

Omogućava agentu da onemogući dalje odgovore ispod komentara, ili vrati mogućnost odgovaranja. Zaključani komentar ostaje vidljiv. Korisno za hlađenje žustrih tema, u paru sa odgođenim otključavanjem. Reverzibilno ali vidljivo vašoj zajednici; razmotrite stavljanje iza odobrenja u visokorizičnim zajednicama.

#### Označi / ukloni oznaku spama

Omogućava agentu da označi komentar kao spam (sakrivajući ga od čitaoca i hraneći spam klasifikator) ili da ukloni tu oznaku. Osnovni alat za bilo kojeg moderatorskog agenta. Reverzibilno. Snažno razmotrite stavljanje iza odobrenja tokom prvih nedjelja dok ne izgradite povjerenje u agenta.

#### Odobri / poništi odobrenje komentara

Omogućava agentu da prikaže zadržani komentar čitaocima, ili sakrije već vidljivi komentar. Najkorisnije na tenantima koji zadržavaju nove komentare za moderatorski pregled. Visok rizik kod poništavanja vidljivog komentara - razmotrite stavljanje iza odobrenja.

#### Označi komentar kao pregledan

Alat za stanje reda: označava komentar kao "moderator (ili agent) je pogledao ovo." Ne mijenja vidljivost. Nizak rizik; rijetko je ograničeno.

#### Dodijeli značku

Omogućava agentu da korisniku dodijeli značku iz konfiguracije znački vašeg tenanta. Moderatori mogu poništiti. Rijetko ograničeno. Agent mora znati ID značke, zato uključite relevantne ID-e u vaše [smjernice zajednice](#community-guidelines) ili [inicijalni prompt](#personality-prompt).

#### Pošalji email

Omogućava agentu da pošalje plain-text email sa `noreply@fastcomments.com` na adresu koju odabere. Koristite štedljivo — email je alat sa najvećim frikcijama i loše poslane poruke je teško poništiti. Snažno razmotrite stavljanje iza odobrenja, i usmjerite mejlove za odobrenje onome ko posjeduje inbox na koji će agent slati poruke.

#### Sačuvaj / pretraži memoriju agenta

Dva povezana alata koja čitaju i pišu u zajednički pool bilješki o korisniku zbog kojeg je okidač aktiviran. Memorija se dijeli među svim agentima u vašem tenant-u, pa bilješke trijažnog agenta informišu odluke moderatorskog agenta. Pretraživanje je samo za čitanje i uvijek dostupno; spremanje se rijetko ograničava. Vidi [Sistem memorije agenta](#agent-memory-system) za kompletan dizajn.

#### Upozori korisnika

Šalje privatnu DM opomenu korisniku u vezi određenog komentara, i atomski bilježi opomenu u memoriji agenta. Politika eskalacije platforme je izgrađena oko ovog alata — prvo upozori, zabrani tek ako korisnik ponovi prekršaj. Rijeđe je ograničeno nego `ban_user`, ali razmotrite ograničavanje tokom prvih nedjelja rada agenta. Vidi [Upozori korisnika](#tool-warn-user) za kompletnu stranicu.

#### Ban korisnika

Najkonzekventniji alat koji agent može pozvati. Banuje korisnika na fiksni period, opcionalno kao shadow ban, opcionalno također banovanjem IP-a, opcionalno također brišući sve korisnikove komentare. Dvije destruktivne opcije (IP, delete-all) su stavljene iza dodatnih opt-in izbora u formi za uređivanje. U EU regionu, sve zabrane zahtijevaju ljudsko odobrenje (vidi [Usklađenost sa EU DSA član 17](#eu-dsa-compliance)). Snažno razmotrite stavljanje iza odobrenja svugdje. Vidi [Ban user](#tool-ban-user) za kompletnu stranicu.

### Podopcije alata za banovanje

Alat Ban izlaže dvije destruktivne opcije - delete-all-comments i ban-by-IP - koje su za model potpuno skrivene dok ih ne aktivirate putem sekcije **Opcije banovanja** na formi za uređivanje. Čak i ako model halucinira parametar, platforma odbija vrijednosti koje niste aktivirali. Vidi [Ban user](#tool-ban-user).