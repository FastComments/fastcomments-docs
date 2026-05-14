Ko administrator registrira FastComments kot orodje LTI 1.3 Advantage in potrdi institucijske politike, ga inštruktorji dodajo v predmete preko standardnih mest za umeščanje v Blackboardu. Natančni koraki se razlikujejo med Ultra Course View in Original Course View, zato sta obe različici opisani spodaj.

#### Ultra Course View

Ultra Course View je privzeta možnost v Blackboard Learn SaaS od leta 2026.

1. Odprite predmet in pojdite na stran **Course Content**.
2. Premaknite kurzor ali tapnite mesto, kamor želite vstaviti nit komentarjev v orisu, in kliknite vijolični gumb **+** (Add content).
3. Izberite **Content Market**. Plošča Content Market navaja vsa odobrena LTI orodja in Building Block umestitve za vašo institucijo.
4. Poiščite ploščico **FastComments** in kliknite nanjo. Blackboard ustvari vsebinski element na mestu, kjer ste odprli meni **+**.
5. Element se privzeto prikaže v orisu kot vnos "Visible to students" za inštruktorje, ki imajo kot osebno privzeto nastavljeno **Hide from students** izklopljeno. Če je vaša privzeta nastavitev **Hidden**, se element ustvari skrit in vidnost preklopite na vrstici predmeta, ko ste pripravljeni.
6. Če želite spremeniti ime elementa, kliknite naslov v orisu in vnesite nov naziv. Naslov, ki ga študenti vidijo v orisu, je neodvisen od identifikatorja niti v FastComments, zato je preimenovanje varno kadarkoli.

Če ne vidite **Content Market** kot možnosti, ima vaša institucija to umestitev skrito. Enak izbirnik dosežete tudi preko **More tools** v istem meniju **+** pod skupino **LTI Tools**.

#### Original Course View

Original Course View je še vedno podprt v Learn SaaS in ostaja primarna izkušnja za samostojno gostovane Learn 9.1 strani na liniji izdaje Q4 2024 CU.

1. Odprite predmet in vstopite v **Content Area** (na primer privzeta področja **Information** ali **Content** v meniju predmeta).
2. V zgornjem desnem kotu strani vključite **Edit Mode** z drsnikom.
3. V vrstici dejanj kliknite **Build Content**.
4. Pod podmenijem **Learning Tools** kliknite **FastComments**. Podmenij Learning Tools se napolni iz LTI 1.3 umestitev orodij potem, ko administrator registrira orodje. Če ga ne vidite, glejte spodnji razdelek s pogostimi težavami.
5. Na obrazcu **Create FastComments** nastavite:
   - **Name**: oznaka, ki jo študenti vidijo v območju vsebine.
   - **Description**: izbirno besedilo, prikazano nad vdelano nitjo.
   - **Permit Users to View this Content**: preklop za razpoložljivost Da/Ne.
   - **Track Number of Views**: omogočite, če želite Blackboardove statistike ogledov za posamezen element. FastComments vodi lastno analitiko neodvisno.
   - **Date and Time Restrictions**: izbirni okviri **Display After** / **Display Until**.
6. Pošljite obrazec. Orodje se prikaže kot klikabilen element v območju vsebine.

#### Embedding Inside an Item or Document

V obeh pogledih predmeta inštruktorji vdelajo FastComments inline znotraj telesa Itema, Documenta ali katerega koli polja z bogatim besedilom preko gumba LTI Advantage v Content Editorju.

Ultra Course View:

1. Ustvarite ali uredite **Document**.
2. Kliknite **Add content** znotraj telesa dokumenta na mestu, kjer želite, da se nit pojavi.
3. V orodni vrstici urejevalnika odprite meni **Insert content** in kliknite **Content Market** (vstopna točka LTI Advantage / Deep Linking).
4. Izberite **FastComments**. FastComments vrne deep-link podatkovno vsebino in Blackboard vstavi vdelan blok v telo dokumenta na mesto kurzorja.
5. Shrani dokument. Študenti vidijo nit upodobljeno inline, ko se srolajo mimo nje.

Original Course View:

1. Uredite kateri koli element z bogatim besedilom.
2. V orodni vrstici Content Editorja kliknite ikono plus **Add Content** in izberite **Content Market** (v starejših Q4 2024 CU označeno kot **Add Content from External Tool**).
3. Izberite **FastComments**. Urejevalnik vstavi začasni blok, ki se sklicuje na deep-linked vir.
4. Pošljite element.

Vsaka deep-link vdelava ustvari svojo lastno nit FastComments, zato ima element z dvema vdelanima FastComments blokoma dva neodvisna toka komentarjev.

#### Visibility, Release Conditions, and Group Restrictions

Vsebine FastComments se obnašajo kot kateri koli drugi Blackboard vsebinski elementi glede pravil upravljanja dostopa, ki se nanašajo nanje.

- Ultra: kliknite izbirnik vidnosti na vrstici (**Visible to students**, **Hidden from students**, **Conditional availability**). Conditional availability podpira datumsko/časovna okna, pravila glede uspešnosti proti elementom v ocenjevalnici in pravila članstva v primerjavi s skupinami predmeta.
- Original: odprite kontekstni meni elementa in izberite **Adaptive Release** ali **Adaptive Release: Advanced** za omejitev orodja po datumu, članstvu, oceni ali statusu pregleda. Uporabite **Set Group Availability** na elementu za omejitev na določene skupine predmeta.

FastComments spoštuje, kar odloči Blackboardov vrata. Če Blackboard skrije element pred študentom, se zanj LTI zagon nikoli ne zgodi in se ne pojavijo v moderatorjevem pogledu.

#### Gradebook Behavior

FastComments ne poroča ocen nazaj preko LTI Advantage Assignment and Grade Services. Za FastComments vsebine se ne ustvari samodejno stolpec ocen.

Če je vaš Blackboard najemnik nastavljen tako, da samodejno ustvari stolpec v gradebooku za vsak nov vsebinski element ne glede na metapodatke o ocenjevanju, se vseeno prikaže prazen stolpec. Da ga skrijete:

- Ultra: odprite **Gradebook**, kliknite glavo stolpca, izberite **Edit**, in izklopite **Show to students** ter **Include in calculations**. Ali uporabite **Delete**, če vaša institucija dovoljuje brisanje stolpcev za neocenjene elemente.
- Original: odprite **Grade Center**, kliknite chevron stolpca, izberite **Hide from Users (on/off)**, in po potrebi **Hide from Instructor View** pod **Column Organization**.

#### What Students See

Ko študent odpre FastComments element ali se pomakne do vdelanega bloka:

1. Blackboard zažene LTI 1.3 sporočilo proti FastComments. Študent je prijavljen preko SSO z uporabo svoje Blackboard identitete (ime, e-pošta, avatar, vloga) brez prikaza obrazca za prijavo.
2. Nit komentarjev se izriše v iframeu. Opozorila, odgovori, omembe in reakcije so na voljo glede na nastavitve komentarnega pripomočka, konfigurirane v FastComments.
3. Njihovi komentarji so pripisani njihovi Blackboardov račun. Če študent kasneje uredi svoje ime ali fotografijo v Blackboardu, naslednji zagon posodobi profil v FastComments.

Preslikava vlog iz Blackboarda v FastComments:

- **System Administrator** in **Course Builder** se preslikata v FastComments **admin**.
- **Instructor** in **Teaching Assistant** se preslikata v FastComments **moderator**.
- **Student**, **Guest**, in **Observer** se preslikajo v FastComments **commenter**.

Moderatorji vidijo kontrolnike moderacije (pin, hide, ban, delete) inline pri vsakem komentarju v niti.

#### Thread Scoping

FastComments omeji vsako nit z **(Blackboard host, course ID, resource link ID)**. Dva FastComments elementa v istem predmetu ustvarita dve nitki. Enak element kopiran med dvema predmetnima lupinama (na primer preko kopije predmeta) ustvari dve nitki, ker Blackboard izda novo resource link ID med kopiranjem. Če želite ohraniti deljeno nit preko kopij predmeta, uporabite Deep Linking z eksplicitnim thread URN, konfiguriranim v FastComments pred zagonom kopije.

#### Blackboard-Specific Gotchas

**FastComments tile missing from the Build Content menu (Original) or Content Market (Ultra).** Administrator je orodje odobril, vendar je pustil institucijsko politiko, ki blokira ustrezno umestitev. Pojdite v **Administrator Panel** > **Integrations** > **LTI Tool Providers**, uredite vnos FastComments in potrdite, da sta omogočeni umestitvi **Course Content Tool** (Original) in **Course Content Tool - allow students** / **Deep Linking content tool** (Ultra). Shrani in osvežite stran predmeta.

**"Tool not configured for this context" ali "Tool is not deployed" napaka pri zagonu.** Območje namestitve, registrirano med dinamično registracijo, se ne ujema z institucijskim kontekstom, kateremu predmet pripada. V Blackboardovem vnosu ponudnika orodij preverite, ali se **Deployment ID** ujema s tem, kar FastComments prikazuje na svoji strani LTI 1.3 Configuration za tega najemnika. Če se razlikujeta, izbrišite umestitev in ponovno zaženite dinamično registracijo iz svežega registracijskega URL-ja.

**Višina iframea izgleda fiksna ali se vsebina prikaže odrezana.** Nekateri Blackboard najemniki vključujejo strogo Content Security Policy, ki blokira privzeto LTI iframe-resize postMessage. FastComments pošilja tako Canvas-slog `lti.frameResize` sporočilo kot IMS specifikacijsko `org.imsglobal.lti.frameResize` sporočilo za maksimalno združljivost, vendar najemniški nivo CSP preklic blokira poslušalca v nadrejenem oknu. Prosite administratorja, naj potrdi, da je `*.fastcomments.com` na LTI allowlisti orodij in da noben po meri dodan CSP header ne odstranjuje postMessage dogodkov. Nato se spreminjanje velikosti deluje brez nadaljnje konfiguracije.

**Course copy duplicates threads.** Blackboard kopija predmeta izda nove resource link ID za LTI umestitve, zato kopirani predmeti začnejo s praznimi nitmi. To je pričakovano. Če želite, da kopirani predmet podeduje izvorno nit, nastavite Deep Linking z eksplicitnim thread URN pred kopiranjem ali se obrnite na FastComments podporo za množično ponastavitev ID-jev nitk.

**Študent vidi generično Blackboard napako pri zagonu.** Vzrok je manjkajoča ali zastarela `email` claim. Potrdite institucijsko politiko za FastComments, da so pod **User Fields to Send** omogočeni **Role**, **Name**, in **Email Address**. Shrani in nato znova zaženite v novem brskalniškem seji.