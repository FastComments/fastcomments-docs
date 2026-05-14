Kada administrator registruje FastComments kao LTI 1.3 Advantage alat i odobri politike institucije, instruktori ga dodaju u kurseve kroz standardne Blackboard tačke za postavljanje. Tačni koraci se razlikuju između Ultra Course View i Original Course View, pa su oba objašnjena ispod.

#### Ultra Course View

Ultra Course View je podrazumevani prikaz u Blackboard Learn SaaS od 2026.

1. Otvorite kurs i idite na stranicu **Course Content**.
2. Zadržite kursor ili dodirnite mesto gde želite da niti komentara budu ubačene u okviru plana i kliknite ljubičasti **+** (Dodaj sadržaj) dugme.
3. Izaberite **Content Market**. Panel Content Market prikazuje svaki odobren LTI alat i Building Block postavljanje za vašu instituciju.
4. Pronađite pločicu **FastComments** i kliknite je. Blackboard kreira sadržaj na poziciji gde ste otvorili **+** meni.
5. Stavka se po podrazumevanju pojavljuje u planu kao unos „Visible to students“ za instruktore koji imaju lično podrazumevano podešavanje **Hide from students** isključeno. Ako vam je podrazumevano **Hidden**, stavka se kreira skrivena i uključite selektor vidljivosti na redu stavke kad budete spremni.
6. Da preimenujete stavku, kliknite naslov u planu i otkucajte novu oznaku. Naslov koji studenti vide u planu je nezavisan od FastComments identifikatora niti, tako da je bezbedno menjati ime u bilo kom trenutku.

Ako ne vidite **Content Market** kao opciju, vaša institucija je sakrila to postavljanje. Isti izbornik takođe možete otvoriti kroz **More tools** u istom **+** meniju pod grupom **LTI Tools**.

#### Original Course View

Original Course View je i dalje podržan u Learn SaaS i ostaje primarno iskustvo za samostalno hostovane Learn 9.1 sajtove na Q4 2024 CU verziji.

1. Otvorite kurs i uđite u **Content Area** (na primer, podrazumevana **Information** ili **Content** oblast u meniju kursa).
2. Uključite **Edit Mode** pomoću prekidača u gornjem desnom uglu stranice.
3. Kliknite **Build Content** u traci za akcije.
4. U podmeniju **Learning Tools**, kliknite **FastComments**. Podmeni Learning Tools se popunjava iz LTI 1.3 postavljanja alata nakon što administrator registruje alat. Ako ga ne vidite, pogledajte odeljak o problemima dole.
5. Na formi **Create FastComments**, podesite:
   - **Name**: oznaka koju studenti vide u oblasti sadržaja.
   - **Description**: opciona tekst koja se prikazuje iznad ugradjene niti.
   - **Permit Users to View this Content**: da/ne prekidač dostupnosti.
   - **Track Number of Views**: omogućite ako želite Blackboard statistiku pregleda po stavci. FastComments vodi sopstvenu analitiku nezavisno.
   - **Date and Time Restrictions**: opciona polja **Display After** / **Display Until**.
6. Pošaljite. Alat se pojavljuje kao klikabilna stavka u oblasti sadržaja.

#### Embedding Inside an Item or Document

U oba prikaza kursa, instruktori ugrađuju FastComments inline unutar tela Stavke (Item), Dokumenta ili bilo kog polja sa bogatim tekstom preko LTI Advantage dugmeta u Content Editoru.

Ultra Course View:

1. Kreirajte ili uredite **Document**.
2. Kliknite **Add content** unutar tela dokumenta na mestu gde želite da nit bude prikazana.
3. U alatnoj traci editora otvorite meni **Insert content** i kliknite **Content Market** (ulazna tačka za LTI Advantage / Deep Linking).
4. Izaberite **FastComments**. FastComments vraća deep-link payload i Blackboard umeće ugrađeni blok u telo dokumenta na poziciju kursora.
5. Sačuvajte dokument. Studenti vide nit prikazanu inline dok skroluju pored nje.

Original Course View:

1. Uredite bilo koju stavku sa telom koje podržava bogati tekst.
2. U alatnoj traci Content Editor-a kliknite plus ikonu **Add Content** i izaberite **Content Market** (označeno kao **Add Content from External Tool** u starijim Q4 2024 CU izdanjima).
3. Izaberite **FastComments**. Editor umeće zamenski blok koji referencira deep-linked resurs.
4. Pošaljite stavku.

Svaki deep-link embed kreira sopstvenu FastComments nit, tako da stavka sa dva ugrađena FastComments bloka ima dva nezavisna toka komentara.

#### Visibility, Release Conditions, and Group Restrictions

FastComments sadržajne stavke se ponašaju kao i bilo koja druga Blackboard stavka u pogledu pravila pristupa koja su primenjena na njih.

- Ultra: kliknite selektor vidljivosti na redu (**Visible to students**, **Hidden from students**, **Conditional availability**). Conditional availability podržava vremenske prozore, pravila performansi vezana za stavke u gradebook-u i pravila članstva vezana za grupe kursa.
- Original: otvorite kontekstni meni stavke i izaberite **Adaptive Release** ili **Adaptive Release: Advanced** da ograničite alat po datumu, članstvu, oceni ili statusu pregleda. Koristite **Set Group Availability** na stavci da ograničite pristup specifičnim grupama kursa.

FastComments poštuje šta god Blackboard-ova ograda odluči. Ako Blackboard sakrije stavku od studenta, LTI pokretanje se nikada ne događa za tog studenta i on se ne pojavljuje u moderator prikazu.

#### Gradebook Behavior

FastComments ne vraća ocene preko LTI Advantage Assignment and Grade Services. Ne kreira se automatski kolona za ocene za FastComments sadržajne stavke.

Ako je vaš Blackboard tenant podešen da automatski kreira kolonu u gradebook-u za svaku novu stavku bez obzira na metapodatke o ocenjivanju, prazna kolona se pojavljuje ipak. Da je sakrijete:

- Ultra: otvorite **Gradebook**, kliknite na zaglavlje kolone, izaberite **Edit**, i isključite **Show to students** plus **Include in calculations**. Ili koristite **Delete** ako vaša institucija dopušta brisanje kolona za neocenjene stavke.
- Original: otvorite **Grade Center**, kliknite strelicu pored kolone, izaberite **Hide from Users (on/off)**, i opciono **Hide from Instructor View** u okviru **Column Organization**.

#### What Students See

Kada student otvori FastComments stavku ili skroluje do ugrađenog bloka:

1. Blackboard pokreće LTI 1.3 poruku ka FastComments. Student je prijavljen putem SSO koristeći svoj Blackboard identitet (ime, email, avatar, uloga) bez prikazivanja formulara za prijavu.
2. Nit komentara se renderuje u iframe-u. Threading, odgovori, pominjanja i reakcije su svi dostupni u zavisnosti od podešavanja komentara u FastComments widgetu.
3. Njihovi komentari se pripisuju njihovom Blackboard nalogu. Ako student kasnije izmeni svoje ime ili sliku u Blackboard-u, sledeće pokretanje ažurira FastComments profil.

Mapiranje uloga iz Blackboard-a na FastComments:

- **System Administrator** i **Course Builder** mapiraju se na FastComments **admin**.
- **Instructor** i **Teaching Assistant** mapiraju se na FastComments **moderator**.
- **Student**, **Guest**, i **Observer** mapiraju se na FastComments **commenter**.

Moderatori vide kontrole moderacije (pin, hide, ban, delete) inline na svakom komentaru u niti.

#### Thread Scoping

FastComments razgraničava svaku nit po **(Blackboard host, course ID, resource link ID)**. Dve FastComments stavke u istom kursu proizvode dve niti. Ista stavka kopirana preko dva kursa (na primer, kroz kopiju kursa) proizvodi dve niti, zato što Blackboard izdaje novi resource link ID tokom kopiranja. Da biste zadržali zajedničku nit preko kopija kursa, koristite Deep Linking sa eksplicitnim thread URN podešenim u FastComments pre pokretanja kopije.

#### Blackboard-Specific Gotchas

**FastComments pločica nedostaje iz menija Build Content (Original) ili Content Market (Ultra).** Administrator je odobrio alat ali ostavio politiku institucije koja blokira relevantno postavljanje. Idite na **Administrator Panel** > **Integrations** > **LTI Tool Providers**, uredite FastComments unos i potvrdite da su omogućena oba postavljanja **Course Content Tool** (Original) i **Course Content Tool - allow students** / **Deep Linking content tool** (Ultra). Sačuvajte i osvežite stranicu kursa.

**Greška "Tool not configured for this context" ili "Tool is not deployed" pri pokretanju.** Deployment scope registrovan tokom dinamičke registracije ne odgovara kontekstu institucije kojoj kurs pripada. U Blackboard-ovom unosu provajdera alata, proverite da li se **Deployment ID** poklapa sa onim što FastComments prikazuje na svojoj LTI 1.3 Configuration stranici za ovog tenant-a. Ako se razlikuju, obrišite postavljanje i ponovo pokrenite dinamičku registraciju koristeći svežu registracionu URL adresu.

**Visina iframe-a izgleda fiksno ili se sadržaj preseče.** Neki Blackboard tenant-i dolaze sa strogom Content Security Policy koja blokira podrazumevano LTI iframe-resize postMessage. FastComments emituje i Canvas-style `lti.frameResize` poruku i IMS spec-formu `org.imsglobal.lti.frameResize` poruku radi maksimalne kompatibilnosti, ali tenant-nivo CSP override blokira parent listener. Zamolite vašeg administratora da potvrdi da je `*.fastcomments.com` na allowlisti LTI alata i da nijedan prilagođeni CSP header ne uklanja postMessage događaje. Nakon toga promena veličine radi bez dodatne konfiguracije.

**Kopija kursa duplira niti.** Blackboard course copy izdaje nove resource link ID-jeve za LTI postavke, tako da kopirani kursevi počinju sa praznim nitima. Ovo je očekivano. Ako želite da kopirani kurs nasledi originalnu nit, podesite Deep Linking sa eksplicitnim thread URN pre kopiranja, ili kontaktirajte FastComments podršku da u bulk-u remapiraju ID-jeve niti.

**Student vidi generičku Blackboard grešku pri pokretanju.** Uzrok je nedostajući ili zastareo `email` claim. Potvrdite da institucijska politika za FastComments ima omogućena polja **Role**, **Name**, i **Email Address** pod **User Fields to Send**. Sačuvajte, zatim ponovo pokrenite u novoj sesiji pretraživača.