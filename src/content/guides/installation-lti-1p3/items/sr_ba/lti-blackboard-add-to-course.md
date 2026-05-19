Once an administrator has registered FastComments as an LTI 1.3 Advantage tool and approved the institution policies, instructors add it to courses through the standard Blackboard placement points. The exact steps differ between Ultra Course View and Original Course View, so both are covered below.

#### Ultra Course View

Ultra Course View je podrazumijevano u Blackboard Learn SaaS od 2026.

1. Otvorite kurs i idite na stranicu **Sadržaj kursa**.
2. Zadržite pokazivač ili dodirnite mjesto u okviru outline-a gdje želite da se pojavi nit komentara i kliknite ljubičasto **+** (Dodaj sadržaj).
3. Izaberite **Tržište sadržaja**. Panel Tržišta sadržaja navodi svaki odobren LTI alat i Building Block postavljanje za vašu ustanovu.
4. Pronađite pločicu **FastComments** i kliknite je. Blackboard kreira stavku sadržaja na poziciji gdje ste otvorili **+** meni.
5. Stavka se po defaultu pojavljuje u outline-u kao unos "Vidljivo studentima" za instruktore koji imaju **Sakrij od studenata** isključen kao svoj lični default. Ako je vaš default **Skriveno**, stavka se kreira skrivena i uključite selektor vidljivosti na redu stavke kada budete spremni.
6. Da preimenujete stavku, kliknite naslov u outline-u i upišite novu oznaku. Naslov koji studenti vide u outline-u je nezavistan od identifikatora FastComments niti, pa je preimenovanje sigurno u bilo kojem trenutku.

Ako ne vidite **Tržište sadržaja** kao opciju, vaša ustanova je sakrila to postavljanje. Isti izbornik možete otvoriti i kroz **Više alata** u istom **+** meniju pod grupom **LTI Alati**.

#### Original Course View

Original Course View je i dalje podržan u Learn SaaS i ostaje primarno iskustvo za self-hosted Learn 9.1 sajtove na Q4 2024 CU liniji izdanja.

1. Otvorite kurs i uđite u **Područje sadržaja** (na primjer, podrazumijevano **Information** ili **Content** područje u meniju kursa).
2. Uključite **Režim uređivanja** koristeći prekidač u gornjem desnom uglu stranice.
3. Kliknite **Kreiraj sadržaj** u traci sa akcijama.
4. U podmeniju **Alati za učenje**, kliknite **FastComments**. Podmeni Alata za učenje popunjava se iz LTI 1.3 postavljanja alata nakon što administrator registruje alat. Ako ga ne vidite, pogledajte odjeljak sa problemima u nastavku.
5. Na formularu **Kreiraj FastComments**, podesite:
   - **Name**: oznaka koju studenti vide u području sadržaja.
   - **Description**: opcioni tekst koji se prikazuje iznad ugrađene niti.
   - **Permit Users to View this Content**: prekidač za dostupnost Da/Ne.
   - **Track Number of Views**: omogućite ako želite Blackboard-ovu statistiku pregleda po stavci. FastComments vodi vlastitu analitiku nezavisno.
   - **Date and Time Restrictions**: opciona polja **Display After** / **Display Until**.
6. Pošaljite. Alat se pojavljuje kao klikabilna stavka u području sadržaja.

#### Ugradnja unutar stavke ili dokumenta

U oba prikaza kursa, instruktori ugrađuju FastComments inline unutar tijela stavke, dokumenta, ili bilo kojeg polja sa bogatim tekstom putem LTI Advantage dugmeta u Uređivaču sadržaja.

Ultra Course View:

1. Kreirajte ili uredite **Dokument**.
2. Kliknite **Dodaj sadržaj** unutar tijela dokumenta tamo gdje želite da nit bude prikazana.
3. U alatnoj traci urednika otvorite meni **Umetni sadržaj** i kliknite **Tržište sadržaja** (ulazna tačka za LTI Advantage / Deep Linking).
4. Odaberite **FastComments**. FastComments vraća deep-link payload i Blackboard ubacuje ugrađeni blok u tijelo dokumenta na poziciju kursora.
5. Sačuvajte dokument. Studenti vide nit renderovanu inline dok skrolaju pored nje.

Original Course View:

1. Uredite bilo koju stavku koja ima tijelo sa bogatim tekstom.
2. U alatnoj traci Uređivača sadržaja kliknite plus ikonu **Dodaj sadržaj** i izaberite **Tržište sadržaja** (označeno **Dodaj sadržaj iz eksternog alata** u starijim Q4 2024 CU verzijama).
3. Odaberite **FastComments**. Urednik umeće placeholder blok koji referencira deep-linked resurs.
4. Pošaljite stavku.

Svaki deep-link embed proizvodi vlastitu FastComments nit, tako da stavka sa dva ugrađena FastComments bloka ima dva nezavisna toka komentara.

#### Vidljivost, uslovi objavljivanja i ograničenja grupa

Stavke sadržaja FastComments ponašaju se kao i svaka druga Blackboard stavka u pogledu pravila pristupa koja se primjenjuju na njih.

- Ultra: kliknite selektor vidljivosti na redu (**Vidljivo studentima**, **Skriveno od studenata**, **Uslovna dostupnost**). Uslovna dostupnost podržava vremenske prozore, pravila performansi protiv stavki dnevnika ocjena i pravila članstva protiv grupa kursa.
- Original: otvorite kontekst meni stavke i izaberite **Adaptive Release** ili **Adaptive Release: Advanced** da ograničite alat po datumu, članstvu, ocjeni ili statusu pregleda. Koristite **Set Group Availability** na stavci da ograničite na specifične grupe kursa.

FastComments poštuje šta god Blackboard-ova kontrola odluči. Ako Blackboard sakrije stavku od studenta, LTI pokretanje se nikada ne dešava za tog studenta i on se ne pojavljuje u moderator prikazu.

#### Ponašanje u dnevniku ocjena

FastComments ne šalje ocjene nazad putem LTI Advantage Assignment and Grade Services. Ne stvara se automatski kolona ocjena za FastComments stavke.

Ako je vaš Blackboard tenant konfigurisan da automatski kreira kolonu dnevnika ocjena za svaku novu stavku sadržaja bez obzira na metapodatke ocjenjivanja, prazna kolona će se i dalje pojaviti. Da je sakrijete:

- Ultra: otvorite **Dnevnik ocjena**, kliknite zaglavlje kolone, izaberite **Uredi**, i isključite **Pokaži studentima** i **Uključi u kalkulacije**. Ili koristite **Obriši** ako vaša ustanova dozvoljava brisanje kolona za neocjenjene stavke.
- Original: otvorite **Centar ocjena**, kliknite na strelicu kolone, izaberite **Sakrij od korisnika (uklj./isklj.)**, i po želji **Sakrij iz prikaza instruktora** unutar **Organizacija kolona**.

#### Šta studenti vide

Kada student otvori FastComments stavku ili skroluje do ugrađenog bloka:

1. Blackboard pokreće LTI 1.3 poruku ka FastComments. Student je prijavljen putem SSO koristeći svoj Blackboard identitet (ime, email, avatar, uloga) bez da vidi obrazac za prijavu.
2. Nit komentara se prikazuje unutar iframe-a. Threading, odgovori, spominjanja i reakcije su dostupni u zavisnosti od podešavanja widgeta za komentare konfigurisanih u FastComments.
3. Njihovi komentari su pridruženi njihovom Blackboard nalogu. Ako student kasnije izmijeni svoje ime ili fotografiju u Blackboard-u, naredno pokretanje ažurira FastComments profil.

Mapiranje uloga iz Blackboar-da u FastComments:

- **System Administrator** i **Course Builder** mapiraju se na FastComments **admin**.
- **Instructor** i **Teaching Assistant** mapiraju se na FastComments **moderator**.
- **Student**, **Guest**, i **Observer** mapiraju se na FastComments **commenter**.

Moderatori vide kontrolе moderacije (pin, sakrij, zabrani, obriši) inline na svakom komentaru u niti.

#### Opseg niti

FastComments definira opseg svake niti po **(Blackboard host, course ID, resource link ID)**. Dvije FastComments stavke u istom kursu proizvode dvije niti. Ista stavka kopirana u dvije ljuske kursa (na primjer, putem kopije kursa) proizvodi dvije niti, jer Blackboard dodjeljuje novi resource link ID tokom kopiranja. Da biste zadržali zajedničku nit kroz kopije kursa, koristite Deep Linking sa eksplicitnim thread URN konfiguriranim u FastComments prije pokretanja kopije.

#### Blackboard-specifični problemi

**FastComments pločica nedostaje u meniju Kreiraj sadržaj (Original) ili Tržištu sadržaja (Ultra).** Administrator je odobrio alat ali ostavio politiku ustanove koja blokira relevantno postavljanje. Idite na **Administrator Panel** > **Integrations** > **LTI Tool Providers**, uredite FastComments unos i potvrdite da su omogućena postavljanja **Course Content Tool** (Original) i **Course Content Tool - allow students** / **Deep Linking content tool** (Ultra). Sačuvajte i osvježite stranicu kursa.

**"Tool not configured for this context" or "Tool is not deployed" error on launch.** Deployment scope registrovan tokom dinamičke registracije ne odgovara kontekstu ustanove kojem kurs pripada. U Blackboard-ovom unosu provajdera alata, provjerite da li **Deployment ID** odgovara onome što FastComments prikazuje na svojoj stranici LTI 1.3 Configuration za ovog tenanta. Ako se razlikuju, izbrišite postavljanje i ponovo pokrenite dinamičku registraciju iz svježeg URL-a za registraciju (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">preuzmite ga ovdje</a>).

**Visina iframe-a izgleda fiksno ili se sadržaj odsjeca.** Neki Blackboard tenant-i dolaze sa strogo definiranim Content Security Policy koji blokira podrazumijevano LTI iframe-resize postMessage. FastComments emituje i Canvas-stil `lti.frameResize` poruku i IMS spec-formu `org.imsglobal.lti.frameResize` poruku da maksimizuje kompatibilnost, ali tenant-level CSP override može blokirati parent listener. Zamolite administratora da potvrdi da je `*.fastcomments.com` na allowlisti LTI alata i da nijedan custom CSP header ne uklanja postMessage događaje. Nakon toga promjena veličine radi bez dodatne konfiguracije.

**Kopija kursa duplicira niti.** Blackboard kopija kursa izdaje nove resource link ID-jeve za LTI postavke, tako da kopirani kursevi počinju sa praznim nitima. Ovo je očekivano. Ako želite da kopirani kurs naslijedi originalnu nit, podesite Deep Linking sa eksplicitnim thread URN prije kopiranja, ili kontaktirajte FastComments podršku da remapira ID-jeve niti u bulk-u.

**Student vidi generičku Blackboard grešku prilikom pokretanja.** Uzrok je nedostajući ili zastarjeli `email` claim. Potvrdite da politika ustanove za FastComments ima omogućeno **Role**, **Name**, i **Email Address** pod **User Fields to Send**. Sačuvajte, pa ponovo pokrenite u novoj browser sesiji.