Ko je FastComments registriran v platformi, ga inštruktorji dodajo v vsebino tečaja z uporabo standardnih postopkov platforme za zunanja orodja. Ta stran pokriva Sakai 23.x in Schoology Enterprise.

#### Zaklenite javni dostop (priporočeno)

Privzeto so podatki komentarjev FastComments na obeh platformah javno berljivi. Kdor koli, ki lahko ugane URL niti ali API končno točko, si lahko ogleda komentarje, tudi zunaj Sakai ali Schoology. Za razprave v tečajih boste skoraj zagotovo želeli omejiti ogled samo na vpisane študente.

Odprite vašo <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">stran za prilagoditev widgeta</a> in ustvarite pravilo z omogočeno možnostjo **Require SSO To View Comments**, nato nastavite varnostno raven na **Secure SSO**, da se niti naložijo samo preko podpisanega LTI zagona.

Oglejte si [Protecting Comment Threads With Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) za celoten vodič, vključno s tem, kako omejiti veljavnost pravila na eno domeno ali stran.

#### Sakai

**1. Add FastComments to a site**

Vzdrževalec spletnega mesta omogoči orodje za posamezno spletno mesto:

1. Odprite spletno mesto in kliknite **Site Info** v levem meniju.
2. Kliknite **Manage Tools**.
3. Pomaknite se do seznama **External Tools** in vklopite **FastComments**.
4. Kliknite **Continue**, preglejte seznam orodij, nato kliknite **Finish**.

FastComments se zdaj prikaže kot element v levem meniju na spletnem mestu.

**2. Reorder the left-nav entry**

Pojdite na **Site Info** > **Tool Order**. Povlecite **FastComments** na želeno mesto in kliknite **Save**. Iz istega zaslona lahko tudi preimenujete oznako v meniju in jo skrijete pred študenti.

**3. Embed inline in a Lessons page**

Če želite FastComments postaviti neposredno znotraj strani Lessons namesto kot samostojno orodje v levem meniju:

1. Odprite orodje **Lessons** na spletnem mestu.
2. Kliknite **Add Content** > **Add External Tool**.
3. Iz seznama izberite **FastComments**.
4. Če je FastComments med registracijo oglaševal Deep Linking, Sakai odpre izbirnik vsebine orodja, tako da lahko izberete ali označite nit. Če Deep Linking ni bil oglaševan, Sakai vstavi privzeto zagonsko povezavo.
5. Shrani element Lessons.

Vsaka vdelana instanca dobi svojo nit, omejeno na to povezavo do vira.

**4. Permission tweaks for student access**

Sakai nadzoruje zagone zunanjih orodij preko Realms. Za potrditev, da lahko študenti zaženete FastComments:

1. Prijavite se kot Sakai skrbnik in odprite **Administration Workspace** > **Realms**.
2. Odprite ustrezen realm (na primer `!site.template.course` ali specifičen realm za spletno mesto).
3. Potrdite, da ima vloga `access` omogočen `lti.launch` in da so dovoljenja vlog v skupini **external.tools** dodeljena.
4. Shrani realm.

Za spremembe na ravni spletnega mesta lahko vzdrževalec prilagodi vidnost orodja po vlogah v **Site Info** > **Tool Order** s skrivanjem ali prikazom FastComments za posamezne vloge.

**5. What students see**

Študenti kliknejo element FastComments v levem meniju (ali se pomaknejo do vdelanega bloka Lessons) in pristanejo neposredno v pogledu razprave z nitmi. SSO je samodejen: Sakai pošlje identiteto uporabnika v LTI zagon in FastComments ga prijavi pod njegovim Sakai računom.

Role mapping:

- Sakai `Instructor` -> FastComments moderator
- Sakai `Admin` (admin in Administration Workspace) -> FastComments admin
- Sakai `Student` / `access` -> FastComments commenter

**6. Pogoste težave pri Sakai**

- **Tool not visible in Manage Tools.** Če se FastComments ne prikaže na seznamu External Tools, mora Sakai skrbnik odpreti register orodij (**Administration Workspace** > **External Tools** > **FastComments**) in nastaviti **Stealthed** na `false`. Stealthed orodja so skrita pred izbirnikom Manage Tools za posamezno spletno mesto.
- **Launches breaking in shared-session browsers.** CSRF žeton portala Sakai je vezan na sejo brskalnika. Če je študent prijavljen v dveh Sakai mestih v različnih zavihkih ali ima zastarelo sejo, zagon vrne 403. Rešitev: zaprite druge zavihke Sakai, se odjavite, ponovno prijavite in znova zaženite. Skrbniki lahko tudi povečajo `sakai.csrf.token.cache.ttl`, če se to pojavlja po grozdu (cluster-wide).
- **Frame embedding.** Potrdite, da je `lti.frameheight` v `sakai.properties` dovolj velik (600 ali več), da nit komentarjev ne bo odrezana znotraj strani Lessons.

#### Schoology

Schoology Enterprise ima dva scenarija namestitve. Pred dodajanjem orodja v tečaj potrdite, kateri od njima velja.

**1. Two installation scenarios**

- **(a) Enterprise-level install.** Sistemski skrbnik Schoology je namestil FastComments na ravni organizacije in ga dodelil vsem tečajem ali specifičnim predlogam tečajev. Inštruktorji preskočijo namestitev in gredo neposredno na "Add Materials".
- **(b) Instructor self-install.** Inštruktor namesti orodje v posamezen tečaj iz **Course Options** > **External Tools** > **Install LTI Apps**. Samostojna namestitev zahteva, da je sistemski skrbnik najprej odobril aplikacijo FastComments na ravni organizacije.

**2. Add FastComments as a course material**

Znotraj tečaja:

1. Odprite tečaj in pojdite na **Materials**.
2. Kliknite **Add Materials** > **Add File/Link/External Tool**.
3. Izberite **External Tool**.
4. Iz seznama registriranih orodij izberite **FastComments**.
5. Nastavite **Name** (to je tisto, kar študenti vidijo na seznamu gradiv) in po želji dodajte **Description**.
6. Pustite **Enable Grading** (grade passback) **OFF**. FastComments ne pošilja ocen nazaj v Schoology, zato vklop povratka ocen ustvari prazen stolpec v ocenah.
7. Kliknite **Submit**.

Gradivo se zdaj prikaže na seznamu gradiv v tečaju in ob kliku odpre nit FastComments.

**3. Inline embedding via the Rich Text editor**

Če je sistemski skrbnik med registracijo omogočil Deep Linking placement za FastComments, lahko inštruktorji vstavijo nit komentarjev v katerokoli polje Rich Text (navodila za nalogo, vsebina strani, pozivi k razpravi):

1. Odprite Rich Text urejevalnik na ciljni strani.
2. Kliknite ikono **External Tool** (puzzle piece) v orodni vrstici.
3. Izberite **FastComments**.
4. Konfigurirajte vdelavo v pogovornem oknu za deep-linking in kliknite **Insert**.
5. Shrani stran.

Če gumb External Tool ne prikazuje v Rich Text urejevalniku, je za to orodje na tem najemniku onemogočen Deep Linking. Glejte spodnje nasvete za težave.

**4. Visibility and section assignments**

Schoology upravlja razpoložljivost orodij po sekcijah preko Course Options:

1. Iz tečaja kliknite **Course Options** > **External Tools**.
2. Za vsako nameščeno LTI aplikacijo nadzirate, ali je na voljo vsem sekcijam znotraj tečaja ali le določenim sekcijam.
3. Če želite omejiti FastComments na določene sekcije, odznačite sekcije, ki orodja ne bi smele videti.
4. Dostop na ravni sekcije tudi nadzira, katere sekcije vidijo vnos **Add Materials** > **External Tool** za FastComments.

**5. What students see**

Študenti kliknejo gradivo FastComments (ali se pomaknejo do vdelave) in pristanejo v razpravi z nitmi. SSO je samodejen preko Schoology LTI zagona pod njihovim Schoology računom.

Role mapping:

- Schoology `Administrator` -> FastComments admin
- Schoology `Instructor` -> FastComments moderator
- Schoology `Student` -> FastComments commenter

**6. Schoology gotchas**

- **Enterprise-only.** Osebni in brezplačni računi Schoology ne morejo namestiti orodij LTI 1.3. Če je vaš najemnik na brezplačnem nivoju, možnost **External Tools** ni prisotna v Course Options. Nadgradite na Schoology Enterprise za uporabo FastComments.
- **Deep Linking disabled by tenant default.** Nekateri najemniki Schoology omejijo Deep Linking placement na ravni organizacije. Ko je temu tako, inštruktorji vidijo le potek **Add Materials** > **External Tool** in ne gumb External Tool v Rich Text urejevalniku. Za omogočanje vdelave mora sistemski skrbnik iti v **System Settings** > **Integration** > **LTI 1.3** > **FastComments** in omogočiti postavko **Content Item / Deep Linking**, nato shraniti.
- **Per-section assignment override.** Če je FastComments dodeljen na ravni podjetja, vendar ga inštruktor ne more videti v **Add Materials**, je sekcija tečaja izključena v dodelitvi na ravni organizacije. Prosite sistemskega skrbnika, naj doda sekcijo v dodelitev aplikacije FastComments.
- **Material name vs. thread identity.** Preimenovanje gradiva v Schoology ne premakne niti komentarjev. Niti so ključene na ID-ju LTI resource link, tako da preimenovanje ohrani isto nit; brisanje in ponovno ustvarjanje gradiva pa ustvari novo, prazno nit.