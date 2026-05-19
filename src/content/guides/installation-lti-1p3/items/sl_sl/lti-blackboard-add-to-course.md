Ko je skrbnik registriral FastComments kot orodje LTI 1.3 Advantage in odobril institucijske politike, ga inštruktorji dodajo v predmete prek standardnih mest za umestitev v Blackboardu. Natančni koraki se razlikujejo med Ultra Course View in Original Course View, zato sta obe različici opisani spodaj.

#### Pogled Ultra Course View

Ultra Course View je privzeto v Blackboard Learn SaaS od leta 2026.

1. Odprite predmet in pojdite na stran **Course Content**.
2. Postavite kazalec ali tapnite tam, kjer želite, da se prikaže nit komentarjev v orisu, in kliknite vijolični gumb **+** (Add content).
3. Izberite **Content Market**. Plošča Content Market našteva vsa odobrena LTI orodja in postavitve Building Block za vašo institucijo.
4. Poiščite ploščico **FastComments** in kliknite nanjo. Blackboard ustvari vsebinski element na mestu, kjer ste odprli meni **+**.
5. Element se v orisu privzeto prikaže kot vnos »Visible to students« za inštruktorje, ki imajo osebno privzeto nastavitev **Hide from students** izključeno. Če je vaša privzeta nastavitev **Hidden**, je element ustvarjen skrit in preklopnik vidnosti na vrstici elementa vklopite, ko ste pripravljeni.
6. Če želite element preimenovati, kliknite naslov v orisu in vnesite novo oznako. Naslov, ki ga študenti vidijo v orisu, je neodvisen od identifikatorja niti v FastComments, zato je preimenovanje varno kadarkoli.

Če ne vidite **Content Market** kot možnosti, je postavitev v vaši instituciji skrita. Do istega izbirnika dostopate tudi prek **More tools** v istem meniju **+** pod skupino **LTI Tools**.

#### Original Course View

Original Course View je še vedno podprt v Learn SaaS in ostaja primarna izkušnja za samostojno gostovane Learn 9.1 strani na izdaji Q4 2024 CU.

1. Odprite predmet in vstopite v **Content Area** (na primer privzeti **Information** ali **Content** območje v meniju predmeta).
2. V zgornjem desnem kotu strani vklopite **Edit Mode** s preklopnikom.
3. V akcijski vrstici kliknite **Build Content**.
4. V podmeniju **Learning Tools** kliknite **FastComments**. Podmeni Learning Tools se napolni iz LTI 1.3 postavitev orodij po tem, ko skrbnik registrira orodje. Če ga ne vidite, glejte spodnji razdelek o morebitnih težavah.
5. Na obrazcu **Create FastComments** nastavite:
   - **Name**: oznaka, ki jo študenti vidijo v območju z vsebino.
   - **Description**: izbirno besedilo, prikazano nad vdelano nitjo.
   - **Permit Users to View this Content**: preklop za razpoložljivost Da/Ne.
   - **Track Number of Views**: omogočite, če želite Blackboardovo statistiko ogledov za posamezen element. FastComments izvaja lastno analitiko neodvisno.
   - **Date and Time Restrictions**: izbirna okna **Display After** / **Display Until**.
6. Pošljite obrazec. Orodje se prikaže kot klikljiv element v območju z vsebino.

#### Vdelava znotraj elementa ali dokumenta

V obeh pogledih predmeta inštruktorji vdelajo FastComments neposredno v telo elementa, dokumenta ali katerokoli polje z bogatim besedilom prek gumba LTI Advantage v urejevalniku vsebine.

Ultra Course View:

1. Ustvarite ali uredite **Document**.
2. Kliknite **Add content** znotraj telesa dokumenta tam, kjer želite, da se prikaže nit.
3. V orodni vrstici urejevalnika odprite meni **Insert content** in kliknite **Content Market** (vhodna točka LTI Advantage / Deep Linking).
4. Izberite **FastComments**. FastComments vrne deep-link payload in Blackboard vstavi vdelani blok v telo dokumenta na mesto kurzorja.
5. Shranite dokument. Študenti vidijo nit vdelano in prikazano, ko se pomaknejo mimo nje.

Original Course View:

1. Uredite kateri koli element z bogatim besedilnim telesom.
2. V orodni vrstici Content Editor kliknite ikono plus **Add Content** in izberite **Content Market** (v starejših CU izdajah Q4 2024 označeno kot **Add Content from External Tool**).
3. Izberite **FastComments**. Urejevalnik vstavi začasni blok, ki se nanaša na deep-linked vir.
4. Pošljite element.

Vsaka deep-link vdelava ustvari svojo lastno nit FastComments, zato ima element z dvema vdelanima FastComments blokoma dve neodvisni tokovi komentarjev.

#### Vidnost, pogoji izdaje in omejitve na skupine

Vsebinski elementi FastComments se obnašajo kot kateri koli drugi Blackboard vsebinski element glede pravil dostopa, ki se nanje nanašajo.

- Ultra: kliknite izbirnik vidnosti na vrstici (**Visible to students**, **Hidden from students**, **Conditional availability**). Pogojna razpoložljivost podpira časovna okna, pravila uspešnosti glede ocen v gradebooku in pravila članstva glede skupin predmeta.
- Original: odprite kontekstni meni elementa in izberite **Adaptive Release** ali **Adaptive Release: Advanced**, da orodje omejite glede na datum, članstvo, oceno ali status pregleda. Uporabite **Set Group Availability** na elementu, da omejite dostop na določene skupine v predmetu.

FastComments spoštuje karkoli, kar določi Blackboardova blokada. Če Blackboard element skrije pred študentom, se LTI zagon za tega študenta nikoli ne izvede in se ta študent ne pojavi v pogledu moderatorja.

#### Obnašanje v Gradebooku

FastComments ne poroča ocen nazaj preko LTI Advantage Assignment and Grade Services. Za vsebinski element FastComments stolpec ocen ni samodejno ustvarjen.

Če je vaš Blackboard najemnik konfiguriran tako, da samodejno ustvari stolpec v gradebooku za vsak nov vsebinski element ne glede na metapodatke o ocenjevanju, se kljub temu prikaže prazen stolpec. Če ga želite skriti:

- Ultra: odprite **Gradebook**, kliknite glavo stolpca, izberite **Edit** in izklopite **Show to students** ter **Include in calculations**. Ali uporabite **Delete**, če vaša institucija dovoljuje brisanje stolpcev za neocenejane elemente.
- Original: odprite **Grade Center**, kliknite puščico na stolpcu, izberite **Hide from Users (on/off)** in po želji **Hide from Instructor View** pod **Column Organization**.

#### Kaj vidijo študenti

Ko študent odpre FastComments element ali se pomakne do vdelanega bloka:

1. Blackboard zažene LTI 1.3 sporočilo do FastComments. Študent je prijavljen prek SSO z uporabo njihove Blackboard identitete (ime, e-pošta, avatar, vloga) brez prikaza prijavnega obrazca.
2. Nit komentarjev se naloži v iframe. Večnavojnost, odgovori, omembe in reakcije so na voljo glede na nastavitve pripomočka za komentarje, konfigurirane v FastComments.
3. Njihovi komentarji so pripisani njihovemu Blackboard računu. Če študent kasneje uredi svoje ime ali fotografijo v BlackBoardu, naslednji zagon posodobi profil v FastComments.

Preslikava vlog iz Blackboarda v FastComments:

- **System Administrator** in **Course Builder** se preslikata na FastComments **admin**.
- **Instructor** in **Teaching Assistant** se preslikata na FastComments **moderator**.
- **Student**, **Guest** in **Observer** se preslikajo na FastComments **commenter**.

Moderatorji vidijo nadzorne elemente moderiranja (pin, hide, ban, delete) neposredno na vsakem komentarju v niti.

#### Zaklep javnega dostopa (priporočeno)

Privzeto so podatki komentarjev FastComments javno berljivi. Kdor koli, ki ugane URL niti ali API endpoint, lahko vidi komentarje, tudi zunaj BlackBoarda. Pri razpravah v predmetu boste skoraj zagotovo želeli omejiti ogled le na vpisane študente.

Odprite svojo <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">stran za prilagajanje pripomočka</a> in ustvarite pravilo z omogočeno možnostjo **Require SSO To View Comments**, nato nastavite varnostno raven na **Secure SSO**, da se niti lahko naložijo le preko podpisanega LTI zagona.

Oglejte si [Zaščita niti komentarjev s prijavo Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) za celoten vodnik, vključno s tem, kako omejiti pravilo na eno domeno ali stran.

#### Obseg niti

FastComments vsako nit omeji po **(Blackboard host, course ID, resource link ID)**. Dva FastComments elementa v istem predmetu ustvarita dve nitki. Enak element kopiran v dveh lupinah predmeta (na primer prek kopije predmeta) ustvari dve niti, ker Blackboard med kopiranjem izda nov resource link ID. Če želite ohraniti skupno nit preko kopij predmetov, uporabite Deep Linking z izrecnim URN niti, konfiguriranim v FastComments pred zagonom kopije.

#### Blackboard-specifične težave

**Ploščica FastComments manjka v meniju Build Content (Original) ali Content Market (Ultra).** Skrbnik je odobril orodje, vendar je v institucijski politiki blokiral ustrezno postavitev. Pojdite na **Administrator Panel** > **Integrations** > **LTI Tool Providers**, uredite vnos FastComments in potrdite, da sta omogočeni postavitvi **Course Content Tool** (Original) in **Course Content Tool - allow students** / **Deep Linking content tool** (Ultra). Shrani in osvežite stran predmeta.

**Pri zagonu se prikaže napaka "Tool not configured for this context" ali "Tool is not deployed".** Obseg nameščanja, registriran med dinamično registracijo, se ne ujema s kontekstom institucije, kateri predmet pripada. V vnosu ponudnika orodja v Blackboardu preverite, ali se **Deployment ID** ujema s tistim, kar FastComments prikazuje na svoji strani LTI 1.3 Configuration za tega najemnika. Če se razlikujeta, izbrišite postavitev in ponovno zaženite dinamično registracijo iz svežega URL-ja za registracijo (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">pridobite ga tukaj</a>).

**Višina iframe-a deluje fiksno ali je vsebina obrezana.** Nekateri Blackboard najemniki imajo strogo politiko varnosti vsebine (Content Security Policy), ki blokira privzeto LTI iframe-resize postMessage. FastComments pošilja tako sporočilo v stilu Canvas `lti.frameResize` kot tudi IM S specifikacijsko `org.imsglobal.lti.frameResize`, da maksimira združljivost, vendar nadomestek CSP na ravni najemnika blokira poslušalca na starševski strani. Prosite svojega skrbnika, naj potrdi, da je `*.fastcomments.com` na dovoljenem seznamu LTI orodij in da noben po meri dodan CSP naslov ne odstranjuje postMessage dogodkov. Nato velikost spet deluje brez dodatne konfiguracije.

**Kopija predmeta podvoji niti.** Kopija Blackboard predmeta izda nove resource link ID-je za LTI postavitve, zato kopirani predmeti začnejo z praznimi nitmi. To je pričakovano. Če želite, da kopirani predmet podeduje izvirno nit, nastavite Deep Linking z izrecnim URN niti pred kopiranjem ali kontaktirajte podporo FastComments za množično preslikavo ID-jev niti.

**Študent vidi splošno Blackboard napako ob zagonu.** Vzrok je manjkajoči ali zastarel claim `email`. Potrdite, da ima institucijska politika za FastComments omogočene **Role**, **Name** in **Email Address** pod **User Fields to Send**. Shrani in nato ponovno zaženite v novem brskalniškem sejenju.