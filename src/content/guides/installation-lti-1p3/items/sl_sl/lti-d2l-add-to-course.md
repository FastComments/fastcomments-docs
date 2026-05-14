Ta stran razlaga, kako dodati FastComments v Brightspace predmet po tem, ko je skrbnik registriral orodje in ustvaril deployment. Če orodje še ni registrirano, si najprej oglejte D2L vodnik za registracijo.

<div class="screenshot white-bg">
    <div class="title">FastComments vdelan kot tema enote v Brightspace</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-d2l-comments-in-unit.png" alt="FastComments, vgrajen v enoto Brightspace, prikazuje nitaste komentarje in izbirnik @-omenitev" />
</div>

Brightspace ponuja dve izkušnji ustvarjanja vsebin: **Classic Content** in **New Content Experience** (imenovano tudi **Lessons**). Obe vključujeta FastComments, vendar se poti v menijih razlikujejo. Vsak spodnji oddelek pokriva obe različici tam, kjer se razlikujeta.

#### Poiščite orodje FastComments

Orodje FastComments se pojavi na dveh mestih v urejevalniku vsebine predmeta:

1. V izbirniku aktivnosti, do katerega pridete z gumbom **Dodaj obstoječe** modula/enote (v starejših različicah Brightspace je označeno **Dodaj obstoječe aktivnosti**). V trenutnih različicah Brightspace se FastComments prikaže neposredno v izbirniku; v starejših različicah je zložen pod podmeni **Zunanja učna orodja**. Katera koli pot doda FastComments kot samostojno temo.
2. V pogovornem oknu **Vstavi vsebino** v HTML urejevalniku, pod **LTI Advantage**. To vgradi FastComments znotraj HTML teme prek LTI toka globinskega povezovanja.

Če se FastComments ne prikaže v nobenem izbirniku, deployment ni omogočen za organizacijsko enoto, ki vsebuje predmet. Prosite skrbnika Brightspace, naj odpre **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments tool > **View Deployments**, odpre deployment in doda organizacijsko enoto predmeta (ali nadrejeno org enoto) pod **Org Units**.

#### Dodajte FastComments kot temo v modulu

Classic Content:

1. Odprite predmet in kliknite **Content** v navigacijski vrstici.
2. Izberite modul, ki naj vsebuje razpravo (ali ga ustvarite z **Add a module**).
3. Kliknite **Dodaj obstoječe** (starejši Brightspace: **Dodaj obstoječe aktivnosti** > **Zunanja učna orodja**).
4. V izbirniku kliknite **FastComments**. Brightspace ustvari temo v modulu in vas vrne v pogled vsebine.
5. Kliknite novo temo. Preimenujte jo v nekaj opisnega, na primer `FastComments Discussion`, z urejevalnikom imena v vrstici.

New Content Experience (Lessons):

1. Odprite predmet in kliknite **Content**.
2. Odprite enoto in lekcijo, ki naj vsebujeta razpravo.
3. Kliknite **Dodaj** > **Obstoječa aktivnost** in izberite **FastComments** (starejši Brightspace: zložen pod **Zunanja učna orodja**).
4. Aktivnost se doda v lekcijo.
5. Kliknite naslov aktivnosti, da ga preimenujete.

Prvič, ko katerikoli uporabnik (inštruktor ali študent) odpre temo, FastComments inicializira nit za to povezavo vira. Nit je vezana na ID povezave vira, zato preimenovanje ali premikanje teme ne spremeni, katera nit se naloži.

#### Vdelajte FastComments znotraj HTML teme

Uporabite ta potek, kadar želite, da se komentarji prikažejo pod besedilom, videom ali drugo vsebino znotraj iste strani teme namesto kot ločena tema.

1. Odprite ali ustvarite HTML temo v modulu/lekciji.
2. Kliknite **Uredi HTML**, da odprete Brightspace HTML urejevalnik.
3. Postavite kazalec na mesto, kjer naj se prikaže nit komentarjev.
4. Kliknite gumb **Vstavi vsebino** (ikona sestavljanke v orodni vrstici urejevalnika).
5. V pogovornem oknu Vstavi vsebino se pomaknite do **LTI Advantage** in kliknite **FastComments**.
6. FastComments odpre izbirnik za globinsko povezovanje. Potrdite postavitev (privzete možnosti ustrezajo razpravam o vsebini); kliknite **Vstavi** ali **Nadaljuj**.
7. Brightspace vas vrne v HTML urejevalnik z rezervirnim blokom, ki predstavlja LTI zagon. Kliknite **Shrani in zapri** na temi.

Ko se tema naloži, Brightspace rezervni blok zamenja z iframe-om, ki samodejno sproži FastComments prek LTI. Študenti vidijo nit razprave neposredno na strani.

En HTML dokument lahko vsebuje več globinsko povezanih FastComments vnosov. Vsak vnos dobi svojo nit, ker vsak globinski link ustvari ločen ID povezave vira.

#### Tema v modulu proti vdelani kratki povezavi

Izberite pristop **tema v modulu**, ko:

- Je razprava primarna dejavnost za ta korak v modulu.
- Želite, da se tema prikaže v vsebini Brightspace, s sledenjem dokončanja in v Class Progress.

Izberite pristop **vdelava na strani**, ko:

- Naj bodo komentarji pod drugo vsebino na isti strani.
- Ne želite ločenega elementa v vsebini, ki bi bil sledljiv pri dokončanju.

#### Vidnost, osnutek in pogoji izpusta

Nova FastComments tema je privzeto vidna študentom. Če jo želite skriti med nastavitvijo:

1. V urejevalniku vsebine kliknite naslov teme (Classic) ali meni s tremi pikami pri aktivnosti (New Content Experience).
2. Nastavite status na **Osnutek** (Classic) ali izklopite **Vidnost** (New Content Experience).

Teme v stanju osnutka so za študente nevidne. Inštruktorji in asistenti jih še vedno vidijo z značko "Osnutek".

Če želite temo omejiti na določeno skupino ali oddelek:

1. Odprite temo.
2. Kliknite meni naslova teme > **Uredi lastnosti v mestu** (Classic) ali **Uredi** > **Omejitve** (New Content Experience).
3. Pod **Pogoji izpusta** kliknite **Ustvari**.
4. Izberite **Vpis v skupino** ali **Vpis v oddelek**, izberite skupino/oddelek in shranite.

Pogoji izpusta se seštevajo s FastComments lastnim preslikavanjem vlog. Študenti, ki ne vidijo teme, ne dobijo LTI zagona.

#### Kaj študent vidi ob prvem zagonu

Ko študent klikne temo (ali naloži HTML temo z vdelavo):

1. Brightspace v ozadju izvede LTI 1.3 zagon.
2. FastComments prejme študentovo ime, e-pošto, URL avatarja in vlogo v LMS ter jih samodejno prijavi. Ni poziva za prijavo v FastComments.
3. Nit komentarjev za to povezavo vira se prikaže znotraj Brightspace iframe-a.

Preslikava vlog ob zagonu:

- Brightspace `Administrator` postane FastComments **admin** za nit (polno moderiranje, brisanje, blokiranje in dostop do konfiguracije).
- Brightspace `Instructor` postane FastComments **moderator** (pripenjanje, skrivanje, brisanje, blokiranje).
- Vse druge vloge (`Learner`, `TeachingAssistant`, itd.) postanejo običajni komentatorji.

Komentarji so pripisani študentovemu računu v Brightspace. Če študent spremeni svoje ime ali avatar v Brightspace, naslednji LTI zagon sinhronizira spremembo.

#### Višina iframe-a in spreminjanje velikosti

FastComments pošlje sporočilo `org.imsglobal.lti.frameResize` preko postMessage ob vsakem upodabljanju niti in ob spremembah vsebine (nov komentar, razširi odgovore). Brightspace posluša to sporočilo in prilagodi višino iframe-a, da nit ne bo obrezana in da se ne bo prikazal notranji drsnik.

Če iframe ostane na fiksni nizki višini:

- Potrdite, da je predmet naložen prek HTTPS. Brightspace-ov poslušalec postMessage zavrača mešano vsebino.
- Potrdite, da nobena razširitev brskalnika ne blokira kanala postMessage.
- Za vdelave znotraj HTML teme naj zunanji HTML ne ovija iframe-a v vsebnik s fiksno višino. Odstranite morebitni inline `style="height: ..."` iz nadrejenega elementa.

#### Posebnosti Brightspace

**Orodje se ne prikaže v izbirniku Dodaj obstoječe.** Deployment ni omogočen za org enoto tega predmeta. Skrbnik mora dodati org enoto (ali nadrejeno) na seznam Org Units deploymenta. Sama registracija orodja ni dovolj; deployment določa, kateri predmeti vidijo orodje.

**Neujemanje `deployment_id` ob zagonu.** FastComments TOFU-pins prvi `deployment_id`, ki ga vidi za registracijo. Če skrbnik izbriše izvorni deployment in ustvari novega, so zagoni iz novega deploymenta zavrnjeni z napako neujemanja deploymenta. Rešitev je ponovna registracija FastComments (generirajte nov URL za registracijo in ponovno izvedite Dynamic Registration); stara konfiguracija se zamenja.

**Orodje se zažene, vendar prikaže "Neveljaven LTI zagon".** Predmet je v drugačni strukturi najemnika/org kot deployment, ali pa je bil deployment onemogočen po registraciji. Ponovno preverite **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments > stikalo **Enabled** in seznam org enot v deploymentu.

**Imena in vloge manjkajo znotraj FastComments.** Brightspace pošilja LTI zagone z zahtevki Names and Role Provisioning Services (NRPS). Če je bil predmet nadgrajen iz starejše LTI 1.1 povezave, zagon nima zahtevkov `name` in `email`. Ponovno dodajte FastComments temo prek **Dodaj obstoječe** (ne migrirajte stare povezave), da zagon uporablja LTI 1.3.

**Vdelava prikaže zaslon za prijavo namesto samodejne SSO.** HTML tema je bila vstavljena kot navaden <iframe>, ki kaže neposredno na FastComments, namesto prek **Vstavi vsebino** > **LTI Advantage**. Navadni iframe-ji preskočijo LTI zagon in uporabnike preusmerijo na javno stran FastComments. Izbrišite iframe in ga ponovno vstavite preko postopka Vstavi vsebino.