Ovaj vodič pokriva dodavanje FastComments-a u Moodle 4.x kurs nakon što je administrator sajta registrovao alat i podesio da se prikazuje u biraču aktivnosti. Ako FastComments još nije registrovan, prvo pogledajte vodič za registraciju na Moodle-u.

#### Otvorite kurs u režimu za uređivanje

1. Prijavite se na Moodle kao Editing Teacher (ili viša rola) za dati kurs.
2. Otvorite kurs.
3. Uključite **Edit mode** pomoću prekidača u gornjem desnom uglu zaglavlja kursa.

Moodle 4.x je zamenio stari padajući meni "Add an activity or resource" koji je 3.x koristio dijalogom za izbor aktivnosti preko celog ekrana. Moodle 4.5 zadržava isti birač, ali dodaje red sa označenim favoritima na vrhu, tako da jednokratno zakačivanje FastComments-a omogućava brži pristup u kasnijim odeljcima.

#### Dodajte FastComments aktivnost

1. Skrolujte do odeljka kursa (teme ili nedelje) gde pripada diskusija.
2. Kliknite **Add an activity or resource** na dnu tog odeljka.
3. U dijalogu birača izaberite **FastComments**. Ako ga ne vidite, pređite na odeljak sa zamkama ispod.

Otvara se forma sa podešavanjima aktivnosti. Bitna polja:

- **Activity name** (obavezno). Prikazuje se na stranici kursa i u evidenciji ocena. Primer: `Week 3 Discussion`.
- **Activity description**. Opcioni uvodni tekst koji se prikazuje iznad niti komentara.
- **Show description on course page**. Štiklirajte ovo ako želite da opis bude vidljiv bez klikanja na aktivnost.
- **Preconfigured tool**. Podesite na `FastComments` (automatski se bira kada se pokreće iz birača). Ne menjajte.
- **Launch container**. Podesite na **New window**. Pogledajte odeljak sa zamkama za objašnjenje zašto "Same window" ne radi u nekim Moodle implementacijama.
- **Tool URL**, **Public key**, **Shared secret**, **Custom parameters**. Ostavite prazno. Dynamic Registration je ovo obradio na nivou sajta.

Skrolujte na dno i kliknite **Save and return to course** (ili **Save and display** da odmah otvorite aktivnost).

Aktivnost se pojavljuje kao red u odeljku sa FastComments ikonicom. Studenti klikću na red da otvore nit komentara.

#### Ugradite FastComments u red pomoću editora

Za nit unutar stranice (Page), poglavlja knjige (Book), lekcije (Lesson) ili bilo kog drugog resursa koji koristi Atto ili TinyMCE editor:

1. Otvorite resurs u režimu za uređivanje.
2. Postavite kursor na mesto gde bi nit trebalo da se pojavi.
3. U traci sa alatkama editora kliknite na dugme **LTI** / **External tool**. U Atto-u je označeno kao "Insert LTI Advantage content". U TinyMCE-u (podrazumevano u Moodle 4.3+) nalazi se pod menijem **More** kao **External tools**.
4. Izaberite **FastComments** sa liste alata.
5. FastComments otvara deep-linking birač. Potvrdite naslov niti i kliknite **Embed**.
6. Editor ubacuje LTI placeholder blok. Sačuvajte resurs.

Svaka ugrađena instanca je zasebna nit koja se ključa na deep-link content item ID, tako da stranica sa tri FastComments ugradnje dobija tri nezavisne niti.

#### Ograničenje pristupa i podešavanja grupa

Standardna Moodle podešavanja aktivnosti primenjuju se na FastComments aktivnosti:

- **Common module settings** > **Group mode**. Postavljanje na **Separate groups** ili **Visible groups** samo po sebi ne deli FastComments na niti po grupama. Moodle-ov group mode filtrira samo evidenciju ocena i listu članova. Da biste vodili zasebnu nit po grupi, dodajte jednu FastComments aktivnost po grupi i koristite **Restrict access** da ograničite svaku.
- **Restrict access** > **Add restriction**. Podržava standardne Moodle uslove: **Date**, **Grade**, **Group**, **Grouping**, **User profile** i ugnežđene skupove ograničenja. Koristite **Group** da zaključate FastComments aktivnost na jednu grupu.
- **Activity completion**. Podesite na **Students must view this activity to complete it** ako želite praćenje završenosti. FastComments trenutno ne prijavljuje događaj završenosti nazad u Moodle, osim samog pokretanja.

#### Mapiranje rola

FastComments čita LTI `roles` claim koji Moodle šalje pri svakom pokretanju i mapira ga na sledeći način:

- Moodle **Manager** ili **Site administrator** -> FastComments **admin**
- Moodle **Editing teacher** ili **Non-editing teacher** -> FastComments **moderator**
- Moodle **Student** -> FastComments **commenter**
- Moodle **Guest** -> samo za čitanje

Administratori mogu da obrišu bilo koji komentar, banuju korisnike i uređuju podešavanja niti. Moderatori mogu da brišu i odobravaju komentare unutar niti u koju su se pokrenuli. Prilagođene Moodle role nasleđuju mapiranje arhetipa iz koga su klonirane.

#### Šta studenti vide

Studenti klikću na FastComments aktivnost (ili skroluju do ugrađenog bloka unutar stranice ili knjige). Moodle šalje njihov identitet FastComments-u putem LTI pokretanja:

- Nema ekrana za prijavu. FastComments ih prijavljuje koristeći Moodle nalog.
- Njihovo ime za prikaz, email i avatar dolaze iz Moodle-a.
- Nit je ograničena na `(Moodle site, course, resource link ID)`, tako da ista aktivnost duplicirana u drugi kurs dobija svežu nit.
- Odgovori u nizu, glasanje i notifikacije rade isto kao u samostalnoj FastComments niti.

#### Zatvorite javni pristup (preporučeno)

Po podrazumevanju, podaci o komentarima FastComments-a su javno čitljivi. Svako ko pogodi URL niti ili API endpoint može videti njene komentare, čak i van Moodle-a. Za diskusije na kursu gotovo sigurno želite da ograničite pregledanje samo na upisane studente.

Otvorite svoju <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">stranicu za prilagođavanje vidžeta</a> i napravite pravilo sa omogućenim **Require SSO To View Comments**, zatim postavite bezbednosni nivo na **Secure SSO** tako da niti mogu da se učitaju samo kroz potpisano LTI pokretanje.

Pogledajte [Protecting Comment Threads With Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) za kompletan vodič, uključujući kako da ograničite pravilo na jedan domen ili stranicu.

#### Moodle zamke

**FastComments nedostaje u biraču aktivnosti.** Administrator sajta je registrovao alat, ali nije postavio **Tool configuration usage** na **Show in activity chooser and as a preconfigured tool**. Ispravite ovo pod **Site administration** > **Plugins** > **Activity modules** > **External tool** > **Manage tools** > ikonica zupčanika na FastComments pločici.

**Pokretanje ne uspeva ili prikazuje prazan okvir kada je podešeno na "Same window".** Moodle-ovi session kolačići podrazumevano koriste `SameSite=Lax`, a neki pregledači ih uklanjaju pri cross-site POST zahtevu koji LTI 1.3 koristi za povratak iz FastComments-a. Postavite **Launch container** na **New window** za aktivnost. Ovo je obavezan zahtev za ugrađeni FastComments unutar stranice ili knjige, jer put pokretanja ugrađen u editor uvek otvara novi prozor.

**`iss` claim je URL Moodle sajta, a ne tenant ID.** FastComments koristi URL Moodle sajta (vrednost `wwwroot` konfiguracije) kao LTI issuer. Ako se vaša Moodle instanca preseli na novi domen ili promenite `wwwroot`, postojeće FastComments niti ostaju vezane za stari issuer i neće se poklapati sa novim pokretanjima. Ponovo registrujte alat na novi URL i po potrebi migrirajte nite kroz FastComments admin.

**Bekap i restore aktivnosti.** Bekapovanje kursa i njegov restore u novi kurs kreira nove resource link ID, tako da obnovljene FastComments aktivnosti počinju sa praznim nitima. Originalni kurs zadržava originalne niti. Ovo je namerno ponašanje, a ne greška.

**Moodle 4.5 TinyMCE podrazumevano.** Moodle 4.5 dolazi sa TinyMCE kao podrazumevanim editorom za nove instalacije. Dugme External tool nalazi se pod menijem **More** (`...`) umesto u glavnoj traci sa alatkama. Stariji sajtovi nadograđeni sa 4.1 zadržavaju Atto, osim ako administrator nije promenio podrazumevano.