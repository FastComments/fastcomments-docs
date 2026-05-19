Ta stran pokriva dodajanje FastComments v tečaj Brightspace potem, ko je skrbnik registriral orodje in ustvaril nameščanje (deployment). Če orodje še ni registrirano, si najprej oglejte vodnik za registracijo D2L.

<div class="screenshot white-bg">
    <div class="title">FastComments vgrajen kot tema enote v Brightspace</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-d2l-comments-in-unit.png" alt="FastComments running inside a Brightspace unit, showing threaded comments and an @-mention picker" />
</div>

Brightspace ponuja dve izkušnji ustvarjanja vsebine: **Klasična vsebina** in **Nova izkušnja vsebine** (imenovana tudi **Lessons**). Oba omogočata FastComments, vendar se poti v meniju razlikujejo. Vsak razdelek spodaj obravnava obe, kjer se razlikujeta.

#### Poiščite orodje FastComments

Orodje FastComments se prikaže na dveh mestih v urejevalniku vsebine tečaja:

1. Izbirnik dejavnosti, do katerega pridete iz gumba **Dodaj obstoječe** modula/enote (v starejših različicah Brightspace označeno kot **Add Existing Activities**). FastComments se v novejših gradah Brightspace prikaže neposredno v izbirniku; v starejših različicah je v podmeniju **External Learning Tools**. Vsaka pot doda FastComments kot samostojno temo.
2. Pogovorno okno **Vstavi vsebino** znotraj HTML urejevalnika, pod **LTI Advantage**. To vgradi FastComments inline v HTML temo preko LTI deep linking toka.

Če se FastComments ne prikaže v nobenem izbirniku, nameščanje ni omogočeno za org enoto, ki drži tečaj. Prosite svojega skrbnika Brightspace, naj odpre **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments tool > **View Deployments**, odpre deployment in doda org enoto tečaja (ali nadrejeno org enoto) pod **Org Units**.

#### Dodajte FastComments kot temo v modul

Klasična vsebina:

1. Odprite tečaj in kliknite **Vsebina** v navigacijski vrstici.
2. Izberite modul, ki naj vsebuje razpravo (ali ga ustvarite preko **Dodaj modul**).
3. Kliknite **Dodaj obstoječe** (starejši Brightspace: **Add Existing Activities** > **External Learning Tools**).
4. V izbirniku kliknite **FastComments**. Brightspace ustvari temo v modulu in vas vrne v pogled vsebine.
5. Kliknite novo temo. Preimenujte jo v nekaj opisnega, na primer `FastComments Discussion` z uporabo urejevalnika naslova v vrstici.

Nova izkušnja vsebine (Lessons):

1. Odprite tečaj in kliknite **Vsebina**.
2. Odprite enoto in lekcijo, ki naj vsebujeta razpravo.
3. Kliknite **Dodaj** > **Obstoječa dejavnost** in izberite **FastComments** (starejši Brightspace: gnezdi se pod **External Learning Tools**).
4. Dejavnost se doda v lekcijo.
5. Kliknite naslov dejavnosti, da jo preimenujete.

Prvič, ko kateri koli uporabnik (inštruktor ali študent) odpre temo, FastComments inicializira nit za to povezavo vira. Nit je vezana na ID resource link, zato preimenovanje ali premikanje teme ne spremeni, katera nit se naloži.

#### Vgradite FastComments inline v HTML temo

Uporabite ta pot, ko želite, da se komentarji prikažejo pod besedilom, videom ali drugo vsebino znotraj iste strani teme namesto kot ločena tema.

1. Odprite ali ustvarite HTML temo v modulu/lekciji.
2. Kliknite **Uredi HTML**, da odprete Brightspace HTML urejevalnik.
3. Postavite kurzor na mesto, kjer naj se prikaže nit komentarjev.
4. Kliknite gumb **Vstavi vsebino** (ikonica sestavljanke v orodni vrstici urejevalnika).
5. V pogovornem oknu Vstavi vsebino se pomaknite do **LTI Advantage** in kliknite **FastComments**.
6. FastComments odpre picker za deep linking. Potrdite položaj (privzete možnosti delujejo za razprave o vsebini); kliknite **Vstavi** ali **Nadaljuj**.
7. Brightspace se vrne v HTML urejevalnik s prikaznim blokom, ki predstavlja LTI zagon. Kliknite **Shrani in zapri** na temi.

Ko se tema naloži, Brightspace nadomesti prikazni blok z iframe-om, ki samodejno zažene FastComments preko LTI. Študenti vidijo nit razprave inline.

Ena HTML tema lahko vsebuje več deep-linked FastComments vgraditev. Vsaka vgraditev dobi svojo nit, ker vsak deep link ustvarja ločen resource link ID.

#### Tema modula proti inline hitri povezavi

Izberite pristop **tema modula**, kadar:

- Je razprava glavna dejavnost za ta korak v modulu.
- Želite, da se tema prikaže v kazalu vsebine Brightspace, pri sledenju dokončanja in v Class Progress.

Izberite pristop **inline vgradnje**, kadar:

- Naj bodo komentarji pod drugo vsebino na isti strani.
- Ne želite ločene postavke, sledljive v kazalu vsebine.

#### Vidnost, osnutek in pogoji sprostitve

Nova tema FastComments je privzeto vidna študentom. Če jo želite skriti med nastavljanjem:

1. V urejevalniku vsebine kliknite naslov teme (Klasično) ali meni s tremi pikami na dejavnosti (Nova izkušnja vsebine).
2. Nastavite status na **Osnutek** (Klasično) ali izklopite preklopnik **Vidnost** (Nova izkušnja vsebine).

Osnutke teme študenti ne vidijo. Inštruktorji in asistenti jih še vedno vidijo z značko "Osnutek".

Za omejitev teme na določeno skupino ali oddelek:

1. Odprite temo.
2. Kliknite meni naslova teme > **Uredi lastnosti na mestu** (Klasično) ali **Uredi** > **Omejitve** (Nova izkušnja vsebine).
3. Pod **Pogoji sprostitve** kliknite **Ustvari**.
4. Izberite **Group enrollment** ali **Section enrollment**, izberite skupino/sekcijo in shranite.

Pogoji sprostitve se seštevajo s FastComments-ovim lastnim preslikavanjem vlog. Študenti, ki ne morejo videti teme, ne dobijo LTI zagona.

#### Kaj študent vidi ob prvem zagonu

Ko študent klikne temo (ali naloži HTML temo z vgradnjo):

1. Brightspace izvede LTI 1.3 zagon v ozadju.
2. FastComments prejme študentovo ime, e-pošto, URL avatarja in vlogo v LMS ter ga samodejno prijavi. Ni poziva za prijavo v FastComments.
3. Nit komentarjev za to povezavo vira se prikaže znotraj Brightspace iframe-a.

Preslikava vlog ob zagonu:

- Brightspace `Administrator` postane FastComments **admin** za nit (polno moderiranje, brisanje, prepoved in dostop do konfiguracije).
- Brightspace `Instructor` postane FastComments **moderator** (pripni, skrij, izbriši, prepovej).
- Vse druge vloge (`Learner`, `TeachingAssistant`, itd.) postanejo običajni komentatorji.

Komentarji so pripisani študentovemu Brightspace računu. Če študent spremeni svoje ime ali avatar v Brightspace, naslednji LTI zagon sinhronizira spremembo.

#### Omejite javni dostop (priporočeno)

Privzeto so podatki komentarjev FastComments javno berljivi. Vsak, ki lahko uganiti URL niti ali API končno točko, lahko vidi komentarje, tudi zunaj Brightspace. Za razprave v tečaju skoraj zagotovo želite omejiti ogled samo na vpisane udeležence.

Odprite svojo <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">stran za prilagajanje vtičnika</a> in ustvarite pravilo z omogočeno možnostjo **Require SSO To View Comments**, nato nastavite raven varnosti na **Secure SSO**, da se niti lahko naložijo le preko podpisanega LTI zagona.

Oglejte si [Zaščita nitk komentarjev z enotno prijavo (SSO)](/guide-customizations-and-configuration.html#sso-require-to-view-comments) za celoten vodič, vključno s tem, kako omejiti pravilo na en sam domeno ali stran.

#### Višina iframe-a in spreminjanje velikosti

FastComments pošlje postMessage `org.imsglobal.lti.frameResize` ob vsakem renderiranju nitke in ob spremembah vsebine (nov komentar, razširi odgovore). Brightspace posluša to sporočilo in prilagodi višino iframe-a, da nit ni odrezana in da se ne prikaže notranji drsnik.

Če iframe ostane na fiksni nizki višini:

- Potrdite, da je tečaj naložen preko HTTPS. Poslušalec postMessage v Brightspace zavrne iframe-e z mešano vsebino.
- Potrdite, da nobeno razširitev brskalnika ne blokira kanala postMessage.
- Za inline vgradnje v HTML temi ne sme biti nadrejenega elementa, ki zavije iframe v vsebnik s fiksno višino. Odstranite kakršen koli inline `style="height: ..."` iz nadrejenega elementa.

#### Brightspace-specifične pasti

**Orodje se ne prikaže v izbirniku Dodaj obstoječe.** Nameščanje ni omogočeno za org enoto tega tečaja. Skrbnik mora dodati org enoto (ali nadrejeno) na seznam Org Units nameščanja. Sama registracija orodja ni dovolj; deployment določa, kateri tečaji vidijo orodje.

**`deployment_id` ne ustreza ob zagonu.** FastComments TOFU-pripne prvi `deployment_id`, ki ga zazna za registracijo. Če skrbnik izbriše izvorno deployment in ustvari novega, se zagoni iz novega deploymenta zavrnejo z napako neusklajenosti deploymenta. Rešitev je ponovno registrirati FastComments (ustvarite nov URL za registracijo (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">pridobite ga tukaj</a>) in znova zaženite dinamično registracijo); stari zapis konfiguracije se nadomesti.

**Orodje se zažene, vendar prikaže "Invalid LTI launch".** Tečaj je v drugi strukturi najemnika/org, kot jo pokriva deployment, ali pa je bil deployment onemogočen po registraciji. Ponovno preverite **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments > preklopnik **Enabled** in seznam org enot deploymenta.

**Imena in vloge manjkajo znotraj FastComments.** Brightspace pošilja LTI zagone z zahtevki Names and Role Provisioning Services (NRPS). Če je bil tečaj nadgrajen iz starejše LTI 1.1 povezave, zagoni nimajo zahtevkov `name` in `email`. Ponovno dodajte FastComments temo preko **Dodaj obstoječe** (ne migrirajte stare povezave), da zagoni uporabljajo LTI 1.3.

**Vgradnja prikaže zaslon za prijavo namesto samodejne SSO.** HTML tema je bila vstavljena kot običajen `<iframe>`, ki kaže na FastComments, namesto preko **Vstavi vsebino** > **LTI Advantage**. Običajni iframe-i preskočijo LTI zagon in uporabnike usmerijo na javno stran FastComments. Izbrišite iframe in ga ponovno vstavite preko toka Vstavi vsebino.

---