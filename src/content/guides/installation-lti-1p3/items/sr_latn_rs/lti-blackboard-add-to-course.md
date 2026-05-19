Kada je administrator registrovao FastComments kao LTI 1.3 Advantage alat i odobrio politike institucije, nastavnici ga dodaju u kurseve kroz standardne Blackboard tačke za postavljanje. Tačni koraci se razlikuju između Ultra Course View i Original Course View, pa su oba opisana ispod.

#### Ultra Course View

Ultra Course View je podrazumevani u Blackboard Learn SaaS od 2026.

1. Otvorite kurs i idite na stranicu **Course Content**.
2. Pređite mišem ili dodirnite mesto u strukturi gde želite da se nalepi nit komentara i kliknite ljubičasti **+** (Add content) dugme.
3. Izaberite **Content Market**. Panel Content Market prikazuje svaki odobreni LTI alat i Building Block postavljanje za vašu instituciju.
4. Pronađite pločicu **FastComments** i kliknite je. Blackboard kreira sadržaj na poziciji gde ste otvorili **+** meni.
5. Stavka se po podrazumevanju pojavljuje u strukturi kao unos „Visible to students“ za nastavnike koji imaju ličnu podrazumevanu opciju **Hide from students** isključenu. Ako vam je podrazumevano **Hidden**, stavka se kreira kao skrivena i uključite selektor vidljivosti na redu stavke kada budete spremni.
6. Da preimenujete stavku, kliknite naslov u strukturi i unesite novu oznaku. Naslov koji studenti vide u strukturi je nezavistan od FastComments identifikatora niti, tako da je bezbedno preimenovati u bilo kom trenutku.

Ako ne vidite **Content Market** kao opciju, vaša institucija ima to postavljanje sakriveno. Isti izbornik takođe možete otvoriti kroz **More tools** u istom **+** meniju pod grupom **LTI Tools**.

#### Original Course View

Original Course View je i dalje podržan u Learn SaaS i ostaje primarno iskustvo za samohostovane Learn 9.1 sajtove na Q4 2024 CU liniji izdanja.

1. Otvorite kurs i uđite u **Content Area** (na primer, podrazumevani **Information** ili **Content** deo u meniju kursa).
2. Uključite **Edit Mode** pomoću prekidača u gornjem desnom uglu stranice.
3. Kliknite **Build Content** u traci sa akcijama.
4. U podmeniju **Learning Tools**, kliknite **FastComments**. Podmeni Learning Tools popunjava se iz LTI 1.3 postavljanja alata nakon što administrator registruje alat. Ako ga ne vidite, pogledajte odeljak o problemima ispod.
5. Na formi **Create FastComments** podesite:
   - **Name**: oznaka koju studenti vide u sadržajnoj oblasti.
   - **Description**: opcioni tekst koji se prikazuje iznad ugrađene niti.
   - **Permit Users to View this Content**: prekidač za dostupnost Da/Ne.
   - **Track Number of Views**: omogućite ako želite Blackboard statistiku pregleda po stavci. FastComments vodi sopstvenu analitiku nezavisno.
   - **Date and Time Restrictions**: opcioni okviri **Display After** / **Display Until**.
6. Potvrdite. Alat se pojavljuje kao klikabilna stavka u sadržajnoj oblasti.

#### Ugradnja unutar stavke ili dokumenta

U oba prikaza kursa, nastavnici ugrađuju FastComments inline unutar tela stavke, dokumenta ili bilo kog rich-text polja preko Content Editor-ovog LTI Advantage dugmeta.

Ultra Course View:

1. Kreirajte ili izmenite **Document**.
2. Kliknite **Add content** unutar tela dokumenta gde želite da se nit pojavi.
3. U alatnoj traci editora, otvorite meni **Insert content** i kliknite **Content Market** (ulazna tačka za LTI Advantage / Deep Linking).
4. Izaberite **FastComments**. FastComments vraća deep-link payload i Blackboard umeće ugrađeni blok u telo dokumenta na poziciju kursora.
5. Sačuvajte dokument. Studenti vide nit prikazanu inline dok skroluju pored nje.

Original Course View:

1. Izmenite bilo koju stavku sa rich-text telom.
2. U alatnoj traci Content Editora, kliknite ikonu plus **Add Content** i izaberite **Content Market** (označeno kao **Add Content from External Tool** u starijim Q4 2024 CU izdanjima).
3. Izaberite **FastComments**. Editor umeće rezervisani blok koji referencira deep-linked resurs.
4. Predajte stavku.

Svaki deep-link embed proizvodi sopstvenu FastComments nit, tako da stavka sa dva ugrađena FastComments bloka ima dva nezavisna toka komentara.

#### Vidljivost, uslovi objavljivanja i ograničenja grupa

FastComments sadržajne stavke se ponašaju kao i bilo koja druga Blackboard sadržajna stavka u pogledu pravila pristupa koja se primenjuju na njih.

- Ultra: kliknite selektor vidljivosti na redu (**Visible to students**, **Hidden from students**, **Conditional availability**). Conditional availability podržava vremenske prozore, pravila zasnovana na performansama u odnosu na stavke u gradebook-u i pravila članstva u odnosu na grupe kursa.
- Original: otvorite kontekst meni stavke i izaberite **Adaptive Release** ili **Adaptive Release: Advanced** da ograničite alat po datumu, članstvu, oceni ili statusu pregleda. Koristite **Set Group Availability** na stavci da ograničite pristup na određene grupe kursa.

FastComments poštuje šta god Blackboard-ova kontrola odluči. Ako Blackboard sakrije stavku od studenta, LTI pokretanje nikada ne ide za tog studenta i on se ne pojavljuje u prikazu moderatora.

#### Ponašanje u Gradebook-u

FastComments ne izveštava ocene nazad preko LTI Advantage Assignment and Grade Services. Ni jedan kolona za ocene se automatski ne kreira za FastComments sadržajne stavke.

Ako je vaš Blackboard tenant konfigurisan da automatski kreira kolonu u gradebook-u za svaku novu stavku bez obzira na metapodatke za ocenjivanje, prazan kolona će se ipak pojaviti. Da ga sakrijete:

- Ultra: otvorite **Gradebook**, kliknite zaglavlje kolone, izaberite **Edit**, i isključite **Show to students** i **Include in calculations**. Ili koristite **Delete** ako vaša institucija dozvoljava brisanje kolona za neocenjene stavke.
- Original: otvorite **Grade Center**, kliknite chevron kolone, izaberite **Hide from Users (on/off)**, i po potrebi **Hide from Instructor View** pod **Column Organization**.

#### Šta studenti vide

Kada student otvori FastComments stavku ili skroluje do ugrađenog bloka:

1. Blackboard pokreće LTI 1.3 poruku ka FastComments. Student se prijavljuje putem SSO koristeći svoj Blackboard identitet (ime, email, avatar, uloga) bez prikaza forme za prijavu.
2. Nit komentara se prikazuje u iframe-u. Threading, odgovori, pominjanja i reakcije su svi dostupni u zavisnosti od podešavanja komentarskog widgeta konfigurisanog u FastComments.
3. Njihovi komentari su pripisani njihovom Blackboard nalogu. Ako student kasnije izmeni svoje ime ili fotografiju u Blackboard-u, sledeće pokretanje ažurira FastComments profil.

Mapiranje uloga iz Blackboard-a u FastComments:

- **System Administrator** i **Course Builder** mapiraju se na FastComments **admin**.
- **Instructor** i **Teaching Assistant** mapiraju se na FastComments **moderator**.
- **Student**, **Guest**, i **Observer** mapiraju se na FastComments **commenter**.

Moderatori vide kontrole moderacije (pin, hide, ban, delete) inline na svakom komentaru u niti.

#### Ograničite javni pristup (preporučeno)

Po podrazumevanju, podaci komentara u FastComments su javno čitljivi. Bilo ko ko pogodi URL niti ili API endpoint može videti komentare, čak i izvan Blackboard-a. Za diskusije na kursu gotovo sigurno želite da ograničite pregled samo na upisane studente.

Otvorite vašu <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">stranicu za prilagođavanje widgeta</a> i napravite pravilo sa uključenim **Require SSO To View Comments**, zatim postavite nivo bezbednosti na **Secure SSO** tako da se niti mogu učitavati samo kroz potpisano LTI pokretanje.

Pogledajte [Protecting Comment Threads With Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) za pun vodič, uključujući kako da ograničite pravilo na jedan domen ili stranicu.

#### Opseg niti

FastComments opseg svake niti određuje po **(Blackboard host, course ID, resource link ID)**. Dve FastComments stavke u istom kursu proizvode dve niti. Ista stavka kopirana u dve školjke kursa (na primer, kroz kopiju kursa) proizvodi dve niti, jer Blackboard izdaje novi resource link ID tokom kopije. Da biste zadržali zajedničku nit preko kopija kursa, koristite Deep Linking sa eksplicitnim thread URN-om konfiguriranim u FastComments pre pokretanja kopije.

#### Blackboard-specifični problemi

**FastComments pločica nedostaje u meniju Build Content (Original) ili Content Market (Ultra).** Administrator je odobrio alat ali je ostavio institucionalnu politiku koja blokira odgovarajuće postavljanje. Idite na **Administrator Panel** > **Integrations** > **LTI Tool Providers**, izmenite FastComments unos i potvrdite da su omogućena postavljenja **Course Content Tool** (Original) i **Course Content Tool - allow students** / **Deep Linking content tool** (Ultra). Sačuvajte i osvežite stranicu kursa.

**Greška "Tool not configured for this context" ili "Tool is not deployed" prilikom pokretanja.** Opseg deployment-a registrovan tokom dinamičke registracije ne odgovara institucionalnom kontekstu kojem kurs pripada. U Blackboard-ovom unosu za provider alata, proverite da li se **Deployment ID** poklapa sa onim što FastComments prikazuje na svojoj LTI 1.3 Configuration stranici za ovog tenant-a. Ako se razlikuju, izbrišite postavljanje i ponovo pokrenite dinamičku registraciju sa svežim registracionim URL-om (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">nabavite ga ovde</a>).

**Visina iframe-a izgleda fiksno ili se sadržaj seče.** Neki Blackboard tenanti koriste strogu Content Security Policy koja blokira podrazumevano LTI iframe-resize postMessage. FastComments emituje i Canvas-stil `lti.frameResize` poruku i IMS spec-formu `org.imsglobal.lti.frameResize` poruku kako bi se maksimizirala kompatibilnost, ali tenant-nivo CSP override blokira parent listener. Zamolite vašeg administratora da potvrdi da je `*.fastcomments.com` na allowlisti LTI alata i da nijedan prilagođeni CSP header ne uklanja postMessage događaje. Nakon toga resize funkcioniše bez dodatne konfiguracije.

**Kopija kursa duplira niti.** Blackboard copy kursa izdaje nove resource link ID-jeve za LTI postavke, pa kopirani kursevi počinju sa praznim nitima. To je očekivano. Ako želite da kopirani kurs nasledi originalnu nit, podesite Deep Linking sa eksplicitnim thread URN-om pre kopiranja, ili kontaktirajte FastComments podršku da remapiraju ID-jeve niti u bulk-u.

**Student vidi generičku Blackboard grešku prilikom pokretanja.** Uzrok je nedostajući ili zastareli `email` claim. Potvrdite da institucijska politika za FastComments ima omogućeno **Role**, **Name**, i **Email Address** pod **User Fields to Send**. Sačuvajte, zatim ponovo pokrenite u novoj sesiji pregledača.