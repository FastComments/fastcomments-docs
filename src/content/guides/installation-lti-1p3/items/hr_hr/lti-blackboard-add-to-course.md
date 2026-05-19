Nakon što je administrator registrirao FastComments kao LTI 1.3 Advantage alat i odobrio institucionalne politike, instruktori ga dodaju u kolegije putem standardnih Blackboard mjesta za postavljanje. Točni koraci razlikuju se između Ultra Course View i Original Course View, pa su oba opisana dolje.

#### Ultra Course View

Ultra Course View je zadani prikaz u Blackboard Learn SaaS od 2026. godine.

1. Otvorite kolegij i idite na stranicu **Course Content**.
2. Zadržite pokazivač ili dodirnite mjesto u kojem želite da se pojavi nit komentara u sadržaju i kliknite ljubičasti gumb **+** (Add content).
3. Odaberite **Content Market**. Panel Content Market prikazuje svaki odobreni LTI alat i Building Block postavljanje za vašu ustanovu.
4. Pronađite pločicu **FastComments** i kliknite je. Blackboard stvara stavku sadržaja na mjestu gdje ste otvorili izbornik **+**.
5. Stavka se prema zadanim postavkama pojavljuje u sadržaju kao unos "Visible to students" za instruktore koji imaju osobnu zadanu postavku **Hide from students** isključenu. Ako vam je zadana postavka **Hidden**, stavka se kreira skrivena i uključite preklopnik vidljivosti na retku stavke kad budete spremni.
6. Za promjenu imena stavke kliknite naslov u sadržaju i upišite novu oznaku. Naslov koji učenici vide u sadržaju neovisno je o identifikatoru niti u FastCommentsu, pa je promjena imena sigurna u bilo kojem trenutku.

Ako ne vidite **Content Market** kao opciju, vaša ustanova je sakrila to postavljanje. Isto možete doći i putem **More tools** u istom **+** izborniku pod grupom **LTI Tools**.

#### Original Course View

Original Course View i dalje se podržava u Learn SaaS i ostaje primarno iskustvo za samostalno hostane Learn 9.1 stranice na Q4 2024 CU liniji izdanja.

1. Otvorite kolegij i uđite u **Content Area** (na primjer, zadano **Information** ili **Content** područje u izborniku kolegija).
2. Uključite **Edit Mode** pomoću preklopnika u gornjem desnom kutu stranice.
3. Kliknite **Build Content** u traci s akcijama.
4. U podizborniku **Learning Tools** kliknite **FastComments**. Podizbornik Learning Tools popunjava se iz LTI 1.3 postavljanja alata nakon što administrator registrira alat. Ako ga ne vidite, pogledajte odjeljak s poznatim problemima u nastavku.
5. Na obrascu **Create FastComments** postavite:
   - **Name**: oznaka koju učenici vide u području sadržaja.
   - **Description**: opcionalni tekst prikazan iznad ugrađene niti.
   - **Permit Users to View this Content**: preklopnik dostupnosti Da/Ne.
   - **Track Number of Views**: omogućite ako želite Blackboard statistiku pregleda po stavci. FastComments vodi vlastitu analitiku neovisno.
   - **Date and Time Restrictions**: opciona vremena **Display After** / **Display Until**.
6. Potvrdite. Alat se pojavljuje kao stavka na koju se može kliknuti u području sadržaja.

#### Ugrađivanje unutar stavke ili dokumenta

U oba prikaza kolegija, instruktori ugrađuju FastComments unutar tijela stavke, dokumenta ili bilo kojeg polja bogatog teksta putem LTI Advantage gumba Uređivača sadržaja.

Ultra Course View:

1. Kreirajte ili uredite **Document**.
2. Kliknite **Add content** unutar tijela dokumenta na mjesto gdje želite da se nit pojavi.
3. U alatnoj traci uređivača otvorite izbornik **Insert content** i kliknite **Content Market** (ulazna točka LTI Advantage / Deep Linking).
4. Odaberite **FastComments**. FastComments vraća deep-link payload i Blackboard umeće ugrađeni blok u tijelo dokumenta na poziciju kursora.
5. Spremite dokument. Učenici vide nit prikazanu unutar dokumenta dok skrolaju.

Original Course View:

1. Uredite bilo koju stavku s tijelom bogatog teksta.
2. U alatnoj traci Content Editora kliknite plus ikonu **Add Content** i odaberite **Content Market** (označeno **Add Content from External Tool** u starijim Q4 2024 CU izdanjima).
3. Odaberite **FastComments**. Uređivač umeće zamjenski blok koji referencira deep-linked resurs.
4. Potvrdite stavku.

Svaki deep-link embed proizvodi vlastitu FastComments nit, pa stavka s dva ugrađena FastComments bloka ima dva neovisna toka komentara.

#### Vidljivost, uvjeti objavljivanja i ograničenja grupa

Stavke sadržaja FastComments ponašaju se kao i bilo koja druga Blackboard stavka za pravila kontrole pristupa koja su primijenjena na njih.

- Ultra: kliknite selektor vidljivosti na retku (**Visible to students**, **Hidden from students**, **Conditional availability**). Conditional availability podržava vremenska prozora, pravila izvedbe protiv stavki u knjizi ocjena i pravila članstva protiv grupa kolegija.
- Original: otvorite kontekstni izbornik stavke i odaberite **Adaptive Release** ili **Adaptive Release: Advanced** za ograničavanje alata po datumu, članstvu, ocjeni ili statusu recenzije. Koristite **Set Group Availability** na stavci za ograničavanje na određene grupe kolegija.

FastComments poštuje što god Blackboardova ograničenja odluče. Ako Blackboard sakrije stavku od studenta, LTI pokretanje se nikada ne događa za tog studenta i on se ne pojavljuje u prikazu moderatora.

#### Ponašanje u knjizi ocjena (Gradebook)

FastComments ne vraća ocjene preko LTI Advantage Assignment and Grade Services. Za stavke sadržaja FastComments ne stvara se automatski stupac ocjena.

Ako je vaš Blackboard tenant konfiguriran da automatski stvara stupac u knjizi ocjena za svaku novu stavku sadržaja bez obzira na meta-podatke o ocjenjivanju, prazan stupac će se i dalje pojaviti. Da biste ga sakrili:

- Ultra: otvorite **Gradebook**, kliknite zaglavlje stupca, odaberite **Edit** i isključite **Show to students** plus **Include in calculations**. Ili upotrijebite **Delete** ako vaša ustanova dopušta brisanje stupaca za neocijenjene stavke.
- Original: otvorite **Grade Center**, kliknite chevron stupca, odaberite **Hide from Users (on/off)**, i opcionalno **Hide from Instructor View** pod **Column Organization**.

#### Što učenici vide

Kada student otvori FastComments stavku ili skrola do ugrađenog bloka:

1. Blackboard šalje LTI 1.3 poruku FastCommentsu. Student je prijavljen putem SSO-a koristeći svoj Blackboard identitet (ime, email, avatar, uloga) bez prikaza obrasca za prijavu.
2. Nit komentara se prikazuje u iframeu. Threading, odgovori, spominjanja i reakcije dostupni su ovisno o postavkama widgeta komentara konfiguriranim u FastCommentsu.
3. Njihovi komentari pripisuju se njihovom Blackboard računu. Ako student kasnije u Blackboardu uredi svoje ime ili fotografiju, sljedeće pokretanje ažurira FastComments profil.

Mapiranje uloga iz Blackbirda u FastComments:

- **System Administrator** i **Course Builder** mapiraju se na FastComments **admin**.
- **Instructor** i **Teaching Assistant** mapiraju se na FastComments **moderator**.
- **Student**, **Guest**, i **Observer** mapiraju se na FastComments **commenter**.

Moderatori vide kontrole moderacije (pin, hide, ban, delete) ugrađene uz svaki komentar u niti.

#### Opseg niti

FastComments definira opseg svake niti pomoću **(Blackboard host, course ID, resource link ID)**. Dvije FastComments stavke u istom kolegiju proizvode dvije niti. Ista stavka kopirana u dvije ljuske kolegija (na primjer, putem kopije kolegija) proizvodi dvije niti, jer Blackboard dodjeljuje novi resource link ID tijekom kopije. Da biste zadržali zajedničku nit kroz kopije kolegija, koristite Deep Linking s eksplicitnim thread URN-om konfiguriranim u FastComments prije pokretanja kopije.

#### Poteškoće specifične za Blackboard

**FastComments pločica nedostaje u izborniku Build Content (Original) ili Content Market (Ultra).** Administrator je odobrio alat, ali ostavio institucionalnu politiku koja blokira relevantno postavljanje. Idite na **Administrator Panel** > **Integrations** > **LTI Tool Providers**, uredite unos FastComments i potvrdite da su omogućena oba postavljanja: **Course Content Tool** (Original) i **Course Content Tool - allow students** / **Deep Linking content tool** (Ultra). Spremite i osvježite stranicu kolegija.

**Greška "Tool not configured for this context" ili "Tool is not deployed" pri pokretanju.** Opseg deploymenta registriran tijekom dinamičke registracije ne odgovara kontekstu ustanove kojem kolegij pripada. U unosu provajdera alata u Blackboardu provjerite odgovara li **Deployment ID** onome što FastComments pokazuje na svojoj LTI 1.3 Configuration stranici za ovaj tenant. Ako se razlikuju, izbrišite postavljanje i pokrenite dinamičku registraciju iz novog registraacijskog URL-a (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">nabavite ga ovdje</a>).

**Visina iframea izgleda fiksno ili se sadržaj reže.** Neki Blackboard tenant-i dolaze s strogo podešenim Content Security Policy koji blokira zadani LTI iframe-resize postMessage. FastComments emitira i Canvas-stil `lti.frameResize` poruku i IMS spec-formu `org.imsglobal.lti.frameResize` poruku za maksimalnu kompatibilnost, ali tenant-level CSP override može blokirati roditeljski listener. Zatražite od administratora da potvrdi da je `*.fastcomments.com` na popisu dopuštenih LTI alata i da nijedan prilagođeni CSP zaglavlje ne uklanja postMessage događaje. Nakon toga promjena veličine radi bez dodatne konfiguracije.

**Kopija kolegija duplicira niti.** Kopija kolegija u Blackboardu dodjeljuje nove resource link ID-je za LTI postavljanja, pa kopirani kolegiji počinju s praznim nitima. To je očekivano ponašanje. Ako želite da kopirani kolegij naslijedi izvornu nit, postavite Deep Linking s eksplicitnim thread URN-om prije kopiranja ili kontaktirajte FastComments podršku za masovno remapiranje ID-jeva niti.

**Student vidi generičku Blackboard grešku pri pokretanju.** Uzrok je nedostajući ili zastarjeli `email` claim. Potvrdite da institucionalna politika za FastComments ima omogućeno **Role**, **Name**, i **Email Address** pod **User Fields to Send**. Spremite, a zatim ponovno pokrenite u novoj sesiji preglednika.