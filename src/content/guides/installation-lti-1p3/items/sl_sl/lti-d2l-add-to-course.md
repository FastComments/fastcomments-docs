Ta stran opisuje dodajanje FastComments v tečaj Brightspace potem, ko je skrbnik registriral orodje in ustvaril deployment. Če orodje še ni registrirano, si najprej oglejte vodnik za registracijo D2L.

Brightspace ponuja dve izkušnji ustvarjanja vsebin: **Classic Content** in **New Content Experience** (also called **Lessons**). Obe izkušnji omogočata FastComments, vendar se poti v menijih razlikujejo. Vsak odsek spodaj obravnava obe, kjer se razlikujeta.

#### Locate the FastComments Tool

Orodje FastComments se pojavi na dveh mestih v urejevalniku vsebin tečaja:

1. Aktivnostni izbirnik, do katerega pridete s klikom na modul/unit gumb **Add Existing** (v starejših različicah Brightspace označeno kot **Add Existing Activities**). V novejših različicah se FastComments prikaže neposredno v izbirniku; v starejših različicah je gnezden pod podmenijem **External Learning Tools**. Vsaka od poti doda FastComments kot samostojno temo.
2. Pogovorno okno **Insert Stuff** znotraj HTML urejevalnika, pod **LTI Advantage**. To vgradi FastComments neposredno v HTML temo prek LTI deep linking pretoka.

Če se FastComments ne pojavi v nobenem izbirniku, deployment ni omogočen za org unit, ki vsebuje tečaj. Prosite svojega Brightspace skrbnika, naj odpre **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments tool > **View Deployments**, odpre deployment in doda org unit tečaja (ali nadrejen org unit) pod **Org Units**.

#### Add FastComments as a Topic in a Module

Classic Content:

1. Odprite tečaj in kliknite **Content** v navigacijski vrstici.
2. Izberite modul, ki naj vsebuje razpravo (ali ga ustvarite z **Add a module**).
3. Kliknite **Add Existing** (starejši Brightspace: **Add Existing Activities** > **External Learning Tools**).
4. V izbirniku kliknite **FastComments**. Brightspace ustvari temo v modulu in vas vrne na pogled vsebine.
5. Kliknite novo temo. Preimenujte jo v nekaj opisnega, na primer `FastComments Discussion`, z uporabo vstavnega urejevalnika naslova.

New Content Experience (Lessons):

1. Odprite tečaj in kliknite **Content**.
2. Odprite unit in lesson, ki naj vsebujeta razpravo.
3. Kliknite **Add** > **Existing Activity** in izberite **FastComments** (v starejših različicah Brightspace: gnezdeno pod **External Learning Tools**).
4. Aktivnost se doda v lesson.
5. Kliknite naslov aktivnosti, da ga preimenujete.

Prvič, ko katerikoli uporabnik (inštruktor ali študent) odpre temo, FastComments inicializira thread za to resource link. Thread je vezan na resource link ID, zato preimenovanje ali premik teme ne spremeni, kateri thread se naloži.

#### Embed FastComments Inline in an HTML Topic

Uporabite ta pot, kadar želite, da se komentarji pojavijo pod besedilom, videom ali drugo vsebino znotraj iste strani teme, namesto kot ločena tema.

1. Odprite ali ustvarite HTML temo v modulu/lesson.
2. Kliknite **Edit HTML**, da odprete Brightspace HTML urejevalnik.
3. Postavite kazalec na mesto, kjer naj se pojavi thread komentarjev.
4. Kliknite gumb **Insert Stuff** (ikona sestavljanke v orodni vrstici urejevalnika).
5. V pogovornem oknu Insert Stuff se pomaknite do **LTI Advantage** in kliknite **FastComments**.
6. FastComments odpre deep linking izbirnik. Potrdite umestitev (privzete možnosti delujejo za vsebinske razprave); kliknite **Insert** ali **Continue**.
7. Brightspace se vrne v HTML urejevalnik s pripomočkom (placeholder) bloka, ki predstavlja LTI zagon. Kliknite **Save and Close** na temi.

Ko se tema naloži, Brightspace nadomesti pripomoček z iframe-om, ki samodejno zažene FastComments prek LTI. Študenti vidijo thread razprave neposredno na strani.

Ena HTML tema lahko vsebuje več deep-linked FastComments vnosov. Vsak vnos dobi svoj thread, ker vsak deep link ustvari svoj edinstven resource link ID.

#### Module Topic vs Inline Quicklink

Izberite pristop **module topic**, kadar:

- je razprava primarna dejavnost za ta korak v modulu.
- želite, da se tema prikaže v Brightspaceu v vsebini, s sledilnimi možnostmi dokončanja in Class Progress.

Izberite pristop **inline embed**, kadar:

- naj se komentarji nahajajo pod drugo vsebino na isti strani.
- ne želite ločene postavke v tabeli vsebin, ki jo je mogoče slediti kot dokončano.

#### Visibility, Draft, and Release Conditions

Nova FastComments tema je privzeto vidna študentom. Če jo želite skriti med urejanjem:

1. V urejevalniku vsebin kliknite naslov teme (Classic) ali trikratni meni na aktivnosti (New Content Experience).
2. Nastavite status na **Draft** (Classic) ali izklopite preklop **Visibility** (New Content Experience).

Draft teme so za študente nevidne. Inštruktorji in asistenti jih še vedno vidijo z oznako "Draft".

Če želite omejiti temo na določeno skupino ali oddelek:

1. Odprite temo.
2. Kliknite meni naslova teme > **Edit Properties In-place** (Classic) ali **Edit** > **Restrictions** (New Content Experience).
3. Pod **Release Conditions** kliknite **Create**.
4. Izberite **Group enrollment** ali **Section enrollment**, izberite skupino/sekcijo in shranite.

Pogoji sprostitve se seštevajo z lastnim mapiranjem vlog FastComments. Študenti, ki ne morejo videti teme, ne dobijo LTI zagona.

#### What Students See on First Launch

Ko študent klikne temo (ali naloži HTML temo z vnosom):

1. Brightspace izvede LTI 1.3 launch v ozadju.
2. FastComments prejme študentovo ime, e-pošto, URL avatarja in LMS vlogo ter jih samodejno prijavi. Ni potrebe po prijavi v FastComments.
3. Thread komentarjev za ta resource link se prikaže znotraj Brightspace iframe-a.

Mapiranje vlog ob zagonu:

- Brightspace `Administrator` postane FastComments **admin** za thread (poln dostop do moderacije, brisanja, prepovedi in konfiguracije).
- Brightspace `Instructor` postane FastComments **moderator** (pin, hide, delete, ban).
- Vse druge vloge (`Learner`, `TeachingAssistant`, itd.) postanejo običajni komentatorji.

Komentarji so pripisani študentovemu Brightspace računu. Če študent uredi svoje ime ali avatar v Brightspace, naslednji LTI launch sinhronizira spremembo.

#### Iframe Height and Resize

FastComments pošlje postMessage `org.imsglobal.lti.frameResize` ob vsaki upodobitvi threada in ob spremembah vsebine (nov komentar, širjenje odgovorov). Brightspace posluša to sporočilo in prilagodi višino iframe-a, da thread ni odrezan in da se ne pojavi notranji drsnik.

Če iframe ostane fiksno kratek:

- Potrdite, da se tečaj nalaga preko HTTPS. Brightspace-ov postMessage poslušalec zavrne mešano vsebino (mixed-content) okvirje.
- Preverite, ali katero od razširitev brskalnika ne blokira postMessage kanala.
- Pri inline vstavkih v HTML temi ne sme okolna HTML struktura zavijati iframe-a v vsebnik s fiksno višino. Odstranite kakršenkoli inline atribut `style="height: ..."` iz nadrejenega elementa.

#### Brightspace-Specific Gotchas

**Tool not showing in the Add Existing picker.** Deployment ni omogočen za org unit tega tečaja. Skrbnik mora dodati org unit (ali nadrejenega) na seznam Org Units v deploymentu. Samo registracija orodja ni dovolj; deployment omejuje, kateri tečaji vidijo orodje.

**`deployment_id` mismatch on launch.** FastComments TOFU-pins prvi `deployment_id`, ki ga vidi za registracijo. Če skrbnik izbriše izvirni deployment in ustvari novega, so zagoni iz novega deploymenta zavrnjeni z napako neusklajenosti deploymenta. Rešitev je ponovno registrirati FastComments (generirati nov registration URL in znova zagnati Dynamic Registration); stari zapis konfiguracije se zamenja.

**Tool launches but shows "Invalid LTI launch".** Tečaj je v drugačni strukturi najemnika/organizacije, kot je deployment pokrival, ali je bil deployment onemogočen po registraciji. Ponovno preverite **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments > preklop **Enabled** in seznam org unitov v deploymentu.

**Names and roles missing inside FastComments.** Brightspace pošilja LTI zagone z Names and Role Provisioning Services (NRPS) trditvami. Če je bil tečaj nadgrajen iz starejše LTI 1.1 povezave, zagoni nimajo `name` in `email` trditev. Ponovno dodajte FastComments temo preko **Add Existing** (ne migrirajte stare povezave), tako da zagon uporablja LTI 1.3.

**Embed shows a login screen instead of auto-SSO.** HTML tema je bila vstavljena kot navaden `<iframe>`, ki kaže na FastComments, namesto prek **Insert Stuff** > **LTI Advantage**. Navadni iframe-i preskočijo LTI zagon in uporabnike pripeljejo na javno FastComments stran. Izbrišite iframe in ga ponovno vstavite preko poteka Insert Stuff.