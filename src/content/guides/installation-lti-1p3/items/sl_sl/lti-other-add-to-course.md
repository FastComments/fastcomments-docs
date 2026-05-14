Ko je FastComments registriran na platformi, ga inštruktorji dodajo v vsebino tečaja z uporabo standardnih tokov za zunanjo orodja platforme. Ta stran zajema Sakai 23.x in Schoology Enterprise.

#### Sakai

**1. Dodajte FastComments na spletno mesto**

Vzdrževalec mesta omogoči orodje za posamezno mesto:

1. Odprite mesto in v levem meniju kliknite **Site Info**.
2. Kliknite **Manage Tools**.
3. Pomaknite se do seznama **External Tools** in preklopite **FastComments** na vklop.
4. Kliknite **Continue**, pregledajte seznam orodij in nato kliknite **Finish**.

FastComments se zdaj prikaže kot element v levem meniju na mestu.

**2. Spremenite vrstni red vnosa v levem meniju**

Pojdite v **Site Info** > **Tool Order**. Povlecite **FastComments** na želeno mesto in kliknite **Save**. Na tem zaslonu lahko tudi preimenujete oznako v meniju in jo skrijete pred študenti.

**3. Vdelajte znotraj strani Lessons**

Če želite FastComments postaviti neposredno znotraj strani Lessons namesto kot samostojno orodje v levem meniju:

1. Odprite orodje **Lessons** na mestu.
2. Kliknite **Add Content** > **Add External Tool**.
3. Izberite **FastComments** s seznama.
4. Če je FastComments med registracijo oglaševal Deep Linking, Sakai odpre izbirnik vsebine orodja, da lahko izberete ali označite nit. Če Deep Linking ni bil oglaševan, Sakai vstavi privzeto lansirno povezavo.
5. Shrani element Lessons.

Vsaka vdelana instanca dobi svojo nit, omejeno na ta povezavni vir.

**4. Prilagoditve dovoljenj za dostop študentov**

Sakai omejuje zagone zunanjih orodij preko Realms. Da potrdite, da lahko študenti zaženejo FastComments:

1. Prijavite se kot Sakai skrbnik in odprite **Administration Workspace** > **Realms**.
2. Odprite ustrezen realm (na primer, `!site.template.course` ali specifičen realm mesta).
3. Potrdite, da ima vloga `access` omogočen `lti.launch` in da so dodeljena dovoljenja vlog v skupini **external.tools**.
4. Shrani realm.

Za nadpise na ravni mesta lahko vzdrževalec prilagodi vidnost orodja po vlogah v **Site Info** > **Tool Order** tako, da FastComments skrije ali prikaže za posamezno vlogo.

**5. Kaj vidijo študenti**

Študenti kliknejo element FastComments v levem meniju (ali se pomaknejo do vdelanega bloka v Lessons) in pristanejo neposredno v pogledu z nitmi komentarjev. SSO je samodejen: Sakai pošlje identiteto uporabnika v LTI launch in FastComments jih prijavi pod njihovim Sakai računom.

Preslikava vlog:

- Sakai `Instructor` -> FastComments moderator
- Sakai `Admin` (admin v Administration Workspace) -> FastComments admin
- Sakai `Student` / `access` -> FastComments commenter

**6. Sakai opozorila**

- **Orodje ni vidno v Manage Tools.** Če se FastComments ne pojavi na seznamu External Tools, mora Sakai skrbnik odpreti register orodij (**Administration Workspace** > **External Tools** > **FastComments**) in nastaviti **Stealthed** na `false`. Stealthed orodja so skrita iz izbirnika Manage Tools na nivoju mesta.
- **Zagoni se prekinjajo v brskalnikih z deljenimi sejami.** Portalni CSRF žeton Sakai je vezan na sejo brskalnika. Če je študent prijavljen v dva Sakai mesta v različnih zavihkih ali ima poteklo sejo, zagon vrne 403. Rešitev: zaprite druge zavihke Sakai, se odjavite, ponovno prijavite in ponovno zaženite. Skrbniki lahko tudi povečajo `sakai.csrf.token.cache.ttl`, če se to dogaja v celotnem gruči.
- **Vdelava v okvir.** Potrdite, da je `lti.frameheight` v `sakai.properties` dovolj velik (600 ali več), da nit komentarjev ni odrezana znotraj strani Lessons.

#### Schoology

Schoology Enterprise ima dva scenarija namestitve. Pred dodajanjem orodja potrdite, kateri se uporablja.

**1. Dva scenarija namestitve**

- **(a) Namestitev na ravni podjetja.** System Administrator Schoology je namestil FastComments na ravni organizacije in ga dodelil vsem tečajem ali določenim predlogam tečajev. Inštruktorji preskočijo namestitev in gredo neposredno na "Add Materials".
- **(b) Samostojna namestitev inštruktorja.** Inštruktor namesti orodje v posamezen tečaj iz **Course Options** > **External Tools** > **Install LTI Apps**. Samostojna namestitev zahteva, da je System Administrator najprej odobril aplikacijo FastComments na ravni organizacije.

**2. Dodajte FastComments kot gradivo tečaja**

Znotraj tečaja:

1. Odprite tečaj in pojdite na **Materials**.
2. Kliknite **Add Materials** > **Add File/Link/External Tool**.
3. Izberite **External Tool**.
4. Izberite **FastComments** s seznama registriranih orodij.
5. Nastavite **Name** (to je tisto, kar študenti vidijo na seznamu gradiv) in po želji **Description**.
6. Pustite **Enable Grading** (grade passback) **OFF**. FastComments ne poroča ocen nazaj v Schoology, zato omogočanje vrnitve ocen ustvari prazen stolpec v ocenjevalnici.
7. Kliknite **Submit**.

Gradivo se sedaj prikaže na seznamu gradiv v tečaju in ob kliku odpre nit FastComments.

**3. Vdelava v vrstico z Rich Text urejevalnikom**

Če je System Administrator med registracijo omogočil Deep Linking postavitev za FastComments, lahko inštruktorji vstavijo nit komentarjev v katerokoli polje Rich Text (navodila za nalogo, telo strani, pozive za razpravo):

1. Odprite Rich Text urejevalnik na ciljni strani.
2. Kliknite ikono **External Tool** (ikona sestavljanke) v orodni vrstici.
3. Izberite **FastComments**.
4. Konfigurirajte vdelavo v pogovornem oknu za deep-linking in kliknite **Insert**.
5. Shrani stran.

Če gumb External Tool ne prikazuje v Rich Text urejevalniku, je za to orodje na tej najemniški ravni onemogočen Deep Linking. Glejte spodnja opozorila.

**4. Vidnost in dodelitve sekcij**

Schoology omejuje razpoložljivost orodja po sekcijah znotraj Course Options:

1. Iz tečaja kliknite **Course Options** > **External Tools**.
2. Za vsako nameščeno LTI aplikacijo upravljate, ali je na voljo vsem sekcijam v tečaju ali samo določenim sekcijam.
3. Če želite omejiti FastComments na določene sekcije, počistite sekcije, ki ne bi smele videti orodja.
4. Dostop na ravni sekcij tudi določa, katere sekcije vidijo vnos **Add Materials** > **External Tool** za FastComments.

**5. Kaj vidijo študenti**

Študenti kliknejo gradivo FastComments (ali se pomaknejo do vdelave) in pristanejo v nitni razpravi. SSO je samodejno preko Schoology LTI launch pod njihovim Schoology računom.

Preslikava vlog:

- Schoology `Administrator` -> FastComments admin
- Schoology `Instructor` -> FastComments moderator
- Schoology `Student` -> FastComments commenter

**6. Schoology opozorila**

- **Samo za Enterprise.** Osebni in brezplačni Schoology računi ne morejo namestiti LTI 1.3 orodij. Če je vaš najemnik na brezplačnem nivoju, možnost **External Tools** manjka v Course Options. Nadgradite na Schoology Enterprise, da uporabljate FastComments.
- **Deep Linking onemogočen po privzetku najemnika.** Nekateri najemniki Schoology omejujejo Deep Linking postavitev na ravni organizacije. Ko je to tako, inštruktorji vidijo samo potek **Add Materials** > **External Tool** in ne gumb External Tool v Rich Text urejevalniku. Za omogočanje vdelave v vrstico mora System Administrator iti v **System Settings** > **Integration** > **LTI 1.3** > **FastComments** in omogočiti postavitev **Content Item / Deep Linking**, nato shraniti.
- **Nadpis dodelitve po sekcijah.** Če je FastComments dodeljen na ravni podjetja, vendar ga inštruktor ne vidi v **Add Materials**, je sekcija tečaja izključena v dodelitvi na ravni organizacije. Prosite System Administratorja, da doda sekcijo k dodelitvi aplikacije FastComments.
- **Ime gradiva v primerjavi z identiteto niti.** Preimenovanje gradiva v Schoology ne premakne niti komentarjev. Niti so ključene na LTI resource link ID, zato preimenovanje ohrani isto nit; brisanje in ponovno ustvarjanje gradiva ustvari novo, prazno nit.