Nakon što administrator registrira FastComments kao LTI 1.3 Advantage alat i odobri pravila institucije, nastavnici ga dodaju u tečajeve kroz standardne Blackboard točke postavljanja. Točni koraci se razlikuju između Ultra Course View i Original Course View, pa su oba opisana u nastavku.

#### Ultra Course View

Ultra Course View je zadani prikaz u Blackboard Learn SaaS od 2026. godine.

1. Otvorite tečaj i idite na stranicu **Course Content**.
2. Zadržite pokazivač ili dodirnite mjesto gdje želite da se pojavi nit komentara u strukturi i kliknite ljubičasti gumb **+** (Add content).
3. Odaberite **Content Market**. Panel Content Market prikazuje sve odobrene LTI alate i Building Block postavljanja za vašu instituciju.
4. Pronađite pločicu **FastComments** i kliknite je. Blackboard stvara stavku sadržaja na poziciji gdje ste otvorili izbornik **+**.
5. Stavka se pojavljuje u strukturi kao unos "Visible to students" prema zadanim postavkama za nastavnike koji imaju opciju **Hide from students** isključenu kao svoj osobni zadani izbor. Ako je vaš zadani izbor **Hidden**, stavka se stvara skrivena, a vi promijenite odabir vidljivosti u retku stavke kada budete spremni.
6. Za preimenovanje stavke kliknite naslov u strukturi i upišite novu oznaku. Naslov koji studenti vide u strukturi neovisan je o identifikatoru niti FastComments, pa je preimenovanje sigurno u bilo kojem trenutku.

Ako ne vidite **Content Market** kao opciju, vaša institucija je sakrila to postavljanje. Isti odabirnik možete dohvatiti i kroz **More tools** u istom izborniku **+** pod grupom **LTI Tools**.

#### Original Course View

Original Course View je i dalje podržan u Learn SaaS i ostaje primarno iskustvo za samostalno hostirane Learn 9.1 stranice na liniji izdanja Q4 2024 CU.

1. Otvorite tečaj i uđite u **Content Area** (na primjer, zadani **Information** ili **Content** prostor u izborniku tečaja).
2. Uključite **Edit Mode** pomoću prekidača u gornjem desnom kutu stranice.
3. Kliknite **Build Content** u akcijskoj traci.
4. Pod podizbornikom **Learning Tools** kliknite **FastComments**. Podizbornik Learning Tools popunjava se iz LTI 1.3 postavljanja alata nakon što administrator registrira alat. Ako ga ne vidite, pogledajte odjeljak o zamkama u nastavku.
5. Na obrascu **Create FastComments** postavite:
   - **Name**: oznaku koju studenti vide u prostoru sadržaja.
   - **Description**: neobavezan tekst prikazan iznad ugrađene niti.
   - **Permit Users to View this Content**: prekidač dostupnosti Da/Ne.
   - **Track Number of Views**: omogućite ako želite Blackboard statistiku pregleda po stavci. FastComments vodi vlastitu analitiku neovisno.
   - **Date and Time Restrictions**: neobavezni prozori **Display After** / **Display Until**.
6. Pošaljite. Alat se pojavljuje kao klikabilna stavka u prostoru sadržaja.

#### Ugrađivanje unutar stavke ili dokumenta

U oba prikaza tečaja nastavnici ugrađuju FastComments u tijelo stavke, dokumenta ili bilo kojeg polja obogaćenog teksta kroz LTI Advantage gumb Content Editora.

Ultra Course View:

1. Stvorite ili uredite **Document**.
2. Kliknite **Add content** unutar tijela dokumenta na mjestu gdje želite da se nit pojavi.
3. U alatnoj traci uređivača otvorite izbornik **Insert content** i kliknite **Content Market** (ulazna točka za LTI Advantage / Deep Linking).
4. Odaberite **FastComments**. FastComments vraća deep-link payload i Blackboard ubacuje ugrađeni blok u tijelo dokumenta na poziciji kursora.
5. Spremite dokument. Studenti vide nit prikazanu u retku dok prolaze pored nje.

Original Course View:

1. Uredite bilo koju stavku s tijelom obogaćenog teksta.
2. U alatnoj traci Content Editora kliknite plus ikonu **Add Content** i odaberite **Content Market** (u starijim Q4 2024 CU verzijama označeno kao **Add Content from External Tool**).
3. Odaberite **FastComments**. Uređivač ubacuje blok rezerviranog mjesta koji upućuje na deep-link resurs.
4. Pošaljite stavku.

Svako deep-link ugrađivanje stvara vlastitu nit FastComments, tako da stavka s dva ugrađena FastComments bloka ima dva neovisna toka komentara.

#### Vidljivost, uvjeti objave i ograničenja grupa

FastComments stavke sadržaja ponašaju se kao svaka druga Blackboard stavka sadržaja s obzirom na pravila kontrole pristupa koja se na njih nanose.

- Ultra: kliknite odabirnik vidljivosti u retku (**Visible to students**, **Hidden from students**, **Conditional availability**). Conditional availability podržava vremenske prozore, pravila izvedbe nad stavkama evidencije ocjena i pravila članstva nad grupama tečaja.
- Original: otvorite kontekstni izbornik stavke i odaberite **Adaptive Release** ili **Adaptive Release: Advanced** za ograničavanje alata po datumu, članstvu, ocjeni ili statusu pregleda. Koristite **Set Group Availability** na stavci za ograničavanje na određene grupe tečaja.

FastComments poštuje ono što Blackboard ograničenje odluči. Ako Blackboard sakrije stavku od studenta, LTI pokretanje se nikada ne događa za tog studenta i on se ne pojavljuje u moderatorskom prikazu.

#### Ponašanje evidencije ocjena

FastComments ne šalje ocjene natrag putem LTI Advantage Assignment and Grade Services. Za FastComments stavke sadržaja ne stvara se automatski stupac ocjena.

Ako je vaš Blackboard tenant konfiguriran da automatski stvara stupac u evidenciji ocjena za svaku novu stavku sadržaja bez obzira na metapodatke ocjenjivanja, prazan stupac se svejedno pojavljuje. Da ga sakrijete:

- Ultra: otvorite **Gradebook**, kliknite zaglavlje stupca, odaberite **Edit** i isključite **Show to students** te **Include in calculations**. Ili koristite **Delete** ako vaša institucija dopušta brisanje stupaca za neocjenjene stavke.
- Original: otvorite **Grade Center**, kliknite strelicu stupca, odaberite **Hide from Users (on/off)** i neobavezno **Hide from Instructor View** pod **Column Organization**.

#### Što studenti vide

Kada student otvori FastComments stavku ili skrola do ugrađenog bloka:

1. Blackboard pokreće LTI 1.3 poruku prema FastComments. Student je prijavljen putem SSO-a koristeći svoj Blackboard identitet (ime, e-pošta, avatar, uloga) bez viđanja obrasca za prijavu.
2. Nit komentara prikazuje se u iframeu. Razgovori, odgovori, spominjanja i reakcije dostupni su prema postavkama widgeta za komentare konfiguriranim u FastComments.
3. Njihovi komentari pripisuju se njihovom Blackboard računu. Ako student kasnije promijeni ime ili fotografiju u Blackboardu, sljedeće pokretanje ažurira FastComments profil.

Mapiranje uloga iz Blackboarda u FastComments:

- **System Administrator** i **Course Builder** mapiraju se na FastComments **admin**.
- **Instructor** i **Teaching Assistant** mapiraju se na FastComments **moderator**.
- **Student**, **Guest** i **Observer** mapiraju se na FastComments **commenter**.

Moderatori vide kontrole za moderiranje (prikvači, sakrij, zabrani, izbriši) u retku na svakom komentaru u niti.

#### Zaključavanje javnog pristupa (preporučeno)

Prema zadanim postavkama, podaci o komentarima FastComments javno su čitljivi. Svatko tko može pogoditi URL niti ili API krajnju točku može vidjeti njezine komentare, čak i izvan Blackboarda. Za rasprave u tečajevima gotovo sigurno želite ograničiti pregledavanje samo na upisane studente.

Otvorite svoju <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">stranicu za prilagodbu widgeta</a> i stvorite pravilo s omogućenom opcijom **Require SSO To View Comments**, a zatim postavite sigurnosnu razinu na **Secure SSO** kako bi se niti mogle učitati samo kroz potpisano LTI pokretanje.

Pogledajte [Protecting Comment Threads With Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) za potpuno objašnjenje, uključujući kako ograničiti pravilo na jednu domenu ili stranicu.

#### Opseg niti

FastComments određuje opseg svake niti kombinacijom **(Blackboard host, ID tečaja, ID resursne poveznice)**. Dvije FastComments stavke u istom tečaju stvaraju dvije niti. Ista stavka kopirana u dva okvira tečaja (na primjer, putem kopiranja tečaja) stvara dvije niti, jer Blackboard izdaje novi ID resursne poveznice tijekom kopiranja. Da biste zadržali zajedničku nit kroz kopije tečaja, koristite Deep Linking s eksplicitnim thread URN-om konfiguriranim u FastComments prije pokretanja kopiranja.

#### Blackboard-specifične zamke

**FastComments pločica nedostaje u izborniku Build Content (Original) ili Content Market (Ultra).** Administrator je odobrio alat, ali je ostavio pravilo institucije koje blokira relevantno postavljanje. Idite na **Administrator Panel** > **Integrations** > **LTI Tool Providers**, uredite FastComments unos i potvrdite da su omogućena i postavljanja **Course Content Tool** (Original) i **Course Content Tool - allow students** / **Deep Linking content tool** (Ultra). Spremite i osvježite stranicu tečaja.

**Pogreška "Tool not configured for this context" ili "Tool is not deployed" pri pokretanju.** Opseg implementacije registriran tijekom dinamičke registracije ne odgovara kontekstu institucije kojoj tečaj pripada. U Blackboard unosu davatelja alata provjerite odgovara li **Deployment ID** onome što FastComments prikazuje na svojoj stranici LTI 1.3 Configuration za ovog tenanta. Ako se razlikuju, izbrišite postavljanje i ponovno pokrenite dinamičku registraciju s novog URL-a registracije (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">dohvatite ga ovdje</a>).

**Visina iframea izgleda fiksirana ili se sadržaj odrezuje.** Neki Blackboard tenant-i isporučuju se s strogim Content Security Policy koji blokira zadanu LTI iframe-resize postMessage poruku. FastComments emitira i Canvas-stil poruku `lti.frameResize` i poruku prema IMS specifikaciji `org.imsglobal.lti.frameResize` radi maksimalne kompatibilnosti, ali CSP postavka na razini tenanta blokira roditeljski slušatelj. Zatražite od administratora da potvrdi da je `*.fastcomments.com` na popisu dopuštenih LTI alata i da nijedno prilagođeno CSP zaglavlje ne uklanja postMessage događaje. Promjena veličine tada radi bez daljnje konfiguracije.

**Kopiranje tečaja duplicira niti.** Blackboard kopiranje tečaja izdaje nove ID-jeve resursnih poveznica za LTI postavljanja, pa kopirani tečajevi počinju s praznim nitima. To je očekivano. Ako trebate da kopirani tečaj naslijedi izvornu nit, postavite Deep Linking s eksplicitnim thread URN-om prije kopiranja ili kontaktirajte FastComments podršku za masovno remapiranje ID-jeva niti.

**Student vidi generičku Blackboard pogrešku pri pokretanju.** Uzrok je nedostajuća ili zastarjela `email` tvrdnja. Potvrdite da pravilo institucije za FastComments ima omogućene **Role**, **Name** i **Email Address** pod **User Fields to Send**. Spremite, a zatim ponovno pokrenite u novoj sesiji preglednika.