Alati agenta su radnje koje može poduzeti. Obrascu za uređivanje agenta ima odjeljak **Dozvoljeni pozivi alata** u kojem označite alate koje je ovom agentu dopušteno koristiti, i odjeljak **Odobrenja** u kojem označite radnje koje bi trebale zahtijevati ljudsko odobrenje prije nego stupe na snagu.

Postoje tri razine za bilo koji alat:

- **Zabranjeno** - agent ga ne može vidjeti niti koristiti.
- **Dozvoljeno, bez odobrenja** - agent ga koristi izravno. Zapisano u povijesti pokretanja.
- **Dozvoljeno, s odobrenjem** - poziv agenta stavi se u red za ljudski pregled i izvršava se tek kada ga čovjek odobri.

Zabranjeni alati su tihi: agent ih ne može tražiti i platforma ih odbija bez objašnjenja. Alati koji zahtijevaju odobrenje uvijek prolaze kroz [pretinac odobrenja](#approval-workflow).

### Revizijski trag za svaku radnju

Svaka radnja koju agent poduzme zabilježi se s kratkim opravdanjem (1–2 rečenice koje objašnjavaju zašto) i rezultatom povjerenja (0.0–1.0). Oba se prikazuju u [Pregledu detalja pokretanja](#run-detail-view) i na svakom [odobrenju](#approval-workflow). Pretraživanje memorije je jedina iznimka samo za čitanje: ne bilježi se kao radnja i uvijek je dostupno bez obzira na listu dopuštenih.

### Referenca alata

#### Objavljivanje komentara

Dopušta agentu da objavi komentar kao on sam. Komentar se javno prikazuje pod prikaznim imenom agenta. Koriste ga agenti za pozdravljanje i sažimanje. Povratno - svaki moderator može ukloniti neprimjeren komentar. Stavljajte iza odobrenja ako vaša zajednica zahtijeva da svaka javna poruka bude pregledana od strane čovjeka.

#### Uređivanje komentara

Dopušta agentu da prepiše tekst komentara unutar opsega. Izvorni tekst se čuva u revizijskom zapisu komentara. Rezervirajte za uske slučajeve - brisanje osobnih podataka koje je korisnik nenamjerno otkrio ili ispravak vlastitog prethodnog odgovora agenta. Nije za prepisivanje mišljenja ili ublažavanje tona. Pogledajte [Uredi komentar](#tool-edit-comment) za cijelu stranicu.

#### Glasanje o komentarima

Dopušta agentu da podigne ili spusti glas za komentar. Glas se uračunava u ukupan broj glasova komentara kao i bilo koji drugi glas. Većina zajednica radije nema botove koji glasaju; nije omogućeno ni u jednom početnom predlošku. Ako to dopustite, glasanje je povratno.

#### Prikvači / otpni komentar

Dopušta agentu da prikvači komentar na vrh stranice ili otpne već prikvačen komentar. Platforma ne nameće pravilo jednog prikvačenog komentara po niti, pa agent koji prikvačuje treba biti uputljen da prvo otpne prethodno prikvačeni komentar. Da bi otkrio što je već prikvačeno na istoj stranici, agent može pozvati alat samo za čitanje `get_pinned_comments` (vidi dolje). Koristi se u predlošku Top Comment Pinner.

#### Zaključaj / otključaj komentar

Dopušta agentu da spriječi daljnje odgovore ispod komentara ili da obnovi odgovore. Zaključani komentar ostaje vidljiv. Korisno za smirivanje žustrih tema, u kombinaciji s odgođenim otključavanjem. Da bi otkrio što je trenutno zaključano na istoj stranici, agent može pozvati alat samo za čitanje `get_locked_comments` (vidi dolje).

#### Označi / ukloni oznaku neželjene pošte

Dopušta agentu da označi komentar kao neželjenu poštu (sakriva ga od čitatelja i šalje u klasifikator neželjene pošte) ili ukloni tu oznaku. Temeljni alat za bilo kojeg moderacijskog agenta. Povratno.

#### Odobri / poništi odobrenje komentara

Dopušta agentu da prikaže zadržani komentar čitateljima ili sakrije već vidljivi. Najkorisnije za instance koje drže nove komentare na čekanju za pregled moderatora.

#### Označi komentar kao pregledan

Alat za stanje u redu: označava komentar kao "moderator (ili agent) je to pogledao." Ne mijenja vidljivost. Niska rizičnost; rijetko se stavlja iza odobrenja.

#### Dodijeli značku

Dopušta agentu da korisniku dodeli značku koju ste konfigurirali za vaš tenant. Moderator može poništiti. Kada je ovaj alat omogućen, agent može vidjeti značke vašeg tenanta i sam odabrati pravu, tako da ne morate zalijepiti identifikatore znački u smjernice zajednice ili početni prompt. Da biste usmjerili koju značku treba dodijeliti za koje ponašanje, u promptu se pozivajte na značke prema njihovom **Prikaznom oznaci**.

#### Pošalji e-poštu

Dopušta agentu da pošalje e-poštu u običnom tekstu autoru komentara unutar opsega okidača. Agent nikada ne vidi adresu primatelja - odabere komentar, a platforma dostavi na adresu koju je taj komentator ostavio pri objavi. Adresa pošiljatelja je brendirani pošiljatelj vašeg tenanta (s DKIM-om) kada domena komentara odgovara konfiguriranoj domeni, inače platformin zadani pošiljatelj. Koristite štedljivo - e-pošta je alat s najvećim trenjem i loše poslane poruke teško je poništiti.

#### Spremi / pretraži memoriju agenta

Dva povezana alata koji čitaju i pišu zajednički skup bilješki o korisniku za kojeg je pokrenut okidač. Memorija je dijeljena među svim agentima u vašem tenantu, pa bilješke agenta za trijažu informiraju odluke moderacijskog agenta. Pretraživanje je samo za čitanje i uvijek dostupno; spremanje se rijetko stavlja iza odobrenja. Pogledajte [Sustav memorije agenta](#agent-memory-system) za cjelokupni dizajn.

#### Dohvati prikvačene komentare / Dohvati zaključane komentare

Dva alata samo za čitanje koja navode prikvačene (ili zaključane) komentare na istoj stranici (`urlId`) na kojoj je okidač pokrenut. Ne uzimaju argumente - stranica se čita iz konteksta okidača, tako da agent ne može prebaciti na druge stranice. Koristite ih kad agent treba djelovati na komentaru koji je već prikvačen ili zaključan - obično prvi poziv prije `unpin_comment` ili `unlock_comment`, ili prije prikvačivanja novog komentara kako bi se postojeći prvo mogao otpneiti.

Svaki alat posebno se kontrolira u **Dozvoljeni pozivi alata** (administrator označuje `List pinned comments on the current page` ili `List locked comments on the current page`). Ne mogu se staviti iza odobrenja - alati samo za čitanje nemaju nuspojave koje bi trebalo odobriti. Njihovo pozivanje se ne bilježi kao radnja u povijesti pokretanja; samo nastali poziv `unpin_comment` / `unlock_comment` / `pin_comment` (ako postoji) se prikazuje. Popis je ograničen na najnovijih 20 podudaranja po pozivu.

Važno za razumjeti: kada jedan od tih alata vrati commentId, taj je commentId dodan u opseg agenta za pojedino pokretanje, pa naknadni poziv `unpin_comment` / `unlock_comment` provjerava se prema sigurnosnoj provjeri cilja alata na platformi. Bez prethodnog poziva alata za otkrivanje, agent ne može djelovati na komentare koji nisu već u neposrednom opsegu okidača. Zato agent koji radi otpinivanje obično ima omogućen oba alata (npr. `get_pinned_comments` plus `unpin_comment`).

#### Upozori korisnika

Šalje privatnu DM opomenu korisniku u vezi s određenim komentarom i atomarno bilježi opomenu u memoriji agenta. Politika eskalacije platforme izgrađena je oko ovog alata - prvo upozori, zabrani samo ako korisnik ponovno prekrši pravila. Pogledajte [Upozori korisnika](#tool-warn-user) za cijelu stranicu.

#### Banuj korisnika

Najkonsekventniji alat koji agent može pozvati. Banuje korisnika na određeno vrijeme, opcionalno kao shadow ban, opcionalno također blokira IP, opcionalno također briše sve korisnikove komentare. Dvije destruktivne opcije (IP, brisanje svih komentara) zahtijevaju dodatne potvrde na obrascu za uređivanje. U EU regiji sve zabrane zahtijevaju ljudsko odobrenje (vidi [Usklađenost s člankom 17 EU DSA](#eu-dsa-compliance)). Pogledajte [Banuj korisnika](#tool-ban-user) za cijelu stranicu.

### Podopcije alata za zabranu

Alat Ban izlaže dvije destruktivne opcije - delete-all-comments i ban-by-IP - koje su potpuno skrivene modelu dok ih ne uključite putem odjeljka **Ban options** na obrascu za uređivanje. Čak i ako model halucinira parametar, platforma odbija vrijednosti koje niste uključili. Pogledajte [Banuj korisnika](#tool-ban-user).