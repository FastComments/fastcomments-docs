Ta stran opisuje dodajanje FastComments v tečaj Brightspace potem, ko je skrbnik registriral orodje in ustvaril deployment. Če orodje še ni registrirano, si najprej oglejte D2L registration guide.

<div class="screenshot white-bg">
    <div class="title">FastComments embedded as a unit topic in Brightspace</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-d2l-comments-in-unit.png" alt="FastComments deluje znotraj Brightspace enote, prikazuje niti komentarjev in izbirnik @-omenjanja" />
</div>

Brightspace ponuja dva načina ustvarjanja vsebin: **Classic Content** in **New Content Experience** (imenovano tudi **Lessons**). Oba izpostavljata FastComments, vendar se poti v menijih razlikujejo. Vsak spodnji razdelek pokriva obe različici tam, kjer se razlikujeta.

#### Locate the FastComments Tool

Orodje FastComments se v urejevalniku vsebin tečaja pojavi na dveh mestih:

1. V izbirniku aktivnosti, dostopnem iz gumba **Add Existing** v modulu/enoti (v starejših različicah Brightspace označeno kot **Add Existing Activities**). V novejših različicah Brightspace se FastComments prikaže neposredno v izbirniku; v starejših različicah je v podmeniju **External Learning Tools**. Obe poti dodata FastComments kot samostojno temo.
2. V pogovornem oknu **Insert Stuff** znotraj HTML urejevalnika, pod **LTI Advantage**. To vgradi FastComments inline v HTML temo preko LTI deep linking toka.

Če FastComments ni v nobenem izbirniku, deployment ni omogočen za org unit, ki hrani tečaj. Prosite svojega Brightspace skrbnika, naj odpre **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments tool > **View Deployments**, odpre deployment in doda org unit tečaja (ali nadrejeni org unit) pod **Org Units**.

#### Add FastComments as a Topic in a Module

Classic Content:

1. Odprite tečaj in kliknite **Content** v navigacijski vrstici.
2. Izberite modul, ki naj vsebuje razpravo (ali ga ustvarite z **Add a module**).
3. Kliknite **Add Existing** (starejši Brightspace: **Add Existing Activities** > **External Learning Tools**).
4. V izbirniku kliknite **FastComments**. Brightspace ustvari temo v modulu in vas vrne v pogled vsebin.
5. Kliknite novo temo. Preimenujte jo v nekaj opisnega, na primer `FastComments Discussion`, z uporabo inline urejevalnika naslova.

New Content Experience (Lessons):

1. Odprite tečaj in kliknite **Content**.
2. Odprite enoto in lekcijo, ki naj vsebujeta razpravo.
3. Kliknite **Add** > **Existing Activity** in izberite **FastComments** (v starejših različicah Brightspace: gnezdi se pod **External Learning Tools**).
4. Aktivnost je dodana v lekcijo.
5. Kliknite naslov aktivnosti, da ga preimenujete.

Prvič, ko katerikoli uporabnik (inštruktor ali študent) odpre temo, FastComments inicializira nit za ta resource link. Nit je vezana na resource link ID, zato preimenovanje ali premikanje teme ne spremeni, katera nit se naloži.

#### Embed FastComments Inline in an HTML Topic

Uporabite ta pot, ko želite, da se komentarji pojavijo pod besedilom, videom ali drugo vsebino znotraj iste strani teme, namesto kot ločena tema.

1. Odprite ali ustvarite HTML temo v modulu/lekciji.
2. Kliknite **Edit HTML**, da odprete Brightspace HTML urejevalnik.
3. Postavite kurzor, kjer naj se pojavi nit komentarjev.
4. Kliknite gumb **Insert Stuff** (ikona sestavljanke v orodni vrstici urejevalnika).
5. V pogovornem oknu Insert Stuff se pomaknite do **LTI Advantage** in kliknite **FastComments**.
6. FastComments odpre deep linking izbirnik. Potrdite postavitev (privzete možnosti delujejo za razprave o vsebinah); kliknite **Insert** ali **Continue**.
7. Brightspace vas vrne v HTML urejevalnik s pripono, ki predstavlja LTI zagon. Kliknite **Save and Close** na temi.

Ko se tema naloži, Brightspace zamenja pripono z iframe-om, ki samodejno sproži FastComments preko LTI. Študenti vidijo nit razprave inline.

En HTML tema lahko vsebuje več deep-linked FastComments vnosov. Vsak vnos dobi svojo nit, ker vsak deep link ustvari ločen resource link ID.

#### Module Topic vs Inline Quicklink

Izberite pristop **module topic**, kadar:

- je razprava primarna dejavnost za ta korak v modulu.
- želite, da se tema pojavi v Brightspace-ovem seznamu vsebin, spremljanju dokončanosti in Class Progress.

Izberite pristop **inline embed**, kadar:

- naj komentarji stojijo pod drugo vsebino na isti strani.
- ne želite ločene postavke, sledljive v tabeli vsebin.

#### Visibility, Draft, and Release Conditions

Nova FastComments tema je privzeto vidna študentom. Če jo želite skriti med urejanjem:

1. V urejevalniku vsebin kliknite naslov teme (Classic) ali meni s tremi pikami na aktivnosti (New Content Experience).
2. Nastavite status na **Draft** (Classic) ali izklopite **Visibility** (New Content Experience).

Draft teme so nevidne študentom. Inštruktorji in asistenti jih še vedno vidijo z značko "Draft".

Če želite omejiti temo na določeno skupino ali oddelek:

1. Odprite temo.
2. Kliknite meni naslova teme > **Edit Properties In-place** (Classic) ali **Edit** > **Restrictions** (New Content Experience).
3. Pod **Release Conditions** kliknite **Create**.
4. Izberite **Group enrollment** ali **Section enrollment**, izberite skupino/oddelek in shranite.

Pogoji sprostitve se seštevajo s FastComments-ovim lastnim preslikavanjem vlog. Študenti, ki ne morejo videti teme, ne prejmejo LTI zagona.

#### What Students See on First Launch

Ko študent klikne temo (ali naloži HTML temo z vgrajenim vnosom):

1. Brightspace izvede LTI 1.3 launch v ozadju.
2. FastComments prejme študentovo ime, `email`, URL avatarja in LMS vlogo ter ga samodejno prijavi. Ni poziva za prijavo v FastComments.
3. Nit komentarjev za ta resource link se prikaže znotraj Brightspace iframe-a.

Preslikava vlog pri zagonu:

- Brightspace `Administrator` postane FastComments **admin** za nit (polno moderiranje, brisanje, blokiranje in dostop do nastavitev).
- Brightspace `Instructor` postane FastComments **moderator** (pin, skrij, izbriši, blokiraj).
- Vse ostale vloge (`Learner`, `TeachingAssistant`, itd.) postanejo standardni komentatorji.

Komentarji so pripisani študentovemu Brightspace računu. Če študent spremeni svoje ime ali avatar v Brightspace, naslednji LTI zagon sinhronizira spremembo.

#### Iframe Height and Resize

FastComments po vsakem prikazu niti in pri spremembah vsebine (nov komentar, razširi odgovore) pošilja postMessage `org.imsglobal.lti.frameResize`. Brightspace posluša to sporočilo in prilagodi višino iframe-a, da nit ne bo odrezana in da se ne prikaže notranji drsnik.

Če iframe ostane na fiksni nizki višini:

- Potrdite, da je tečaj naložen preko HTTPS. Brightspace-ov poslušalec postMessage zavrne mešane vsebine.
- Potrdite, da nobena razširitev brskalnika ne blokira postMessage kanala.
- Za inline vgradnje v HTML temi ne smejo biti iframe oviti v starševski element s fiksno višino. Odstranite morebitni inline `style="height: ..."` iz nadrejenega elementa.

#### Brightspace-Specific Gotchas

**Tool not showing in the Add Existing picker.** Deployment ni omogočen za org unit tega tečaja. Skrbnik mora dodati org unit (ali nadrejenega) na seznam Org Units v deploymentu. Sama registracija orodja ni dovolj; deployment omejuje, kateri tečaji vidijo orodje.

**`deployment_id` mismatch on launch.** FastComments TOFU-pins prvo `deployment_id`, ki ga vidi za registracijo. Če skrbnik izbriše izvorni deployment in ustvari novega, se zagoni iz novega deploymenta zavrnejo z napako neskladja deploymenta. Rešitev je ponovno registrirati FastComments (ustvarite nov registration URL (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">get it here</a>) in ponovno zaženite Dynamic Registration); stari zapis konfiguracije se zamenja.

**Tool launches but shows "Invalid LTI launch".** Tečaj je v drugačni strukturi najemnika/organizacije, kot jo pokriva deployment, ali je bil deployment onemogočen po registraciji. Preverite **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments > preklop **Enabled** in seznam org unitov v deploymentu.

**Names and roles missing inside FastComments.** Brightspace pošlje LTI zagone z Names and Role Provisioning Services (NRPS) trditvami. Če je bil tečaj nadgrajen iz starejše LTI 1.1 povezave, zagon ne vsebuje trditev `name` in `email`. Ponovno dodajte FastComments temo preko **Add Existing** (ne migrirajte stare povezave), da zagon uporablja LTI 1.3.

**Embed shows a login screen instead of auto-SSO.** HTML tema je bila vstavljena kot navaden `<iframe>`, ki kaže na FastComments, namesto preko **Insert Stuff** > **LTI Advantage**. Navadni iframe-i preskočijo LTI zagon in uporabnike peljejo na javno stran FastComments. Izbrišite iframe in ga ponovno vstavite preko toka Insert Stuff.