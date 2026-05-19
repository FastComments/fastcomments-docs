Ta vodič pokriva dodajanje FastComments v Moodle 4.x predmet po tem, ko je skrbnik spletnega mesta registriral orodje in ga nastavil, da se prikazuje v izbirniku aktivnosti. Če FastComments še ni registriran, si najprej oglejte vodič za registracijo Moodla.

#### Odprite predmet v načinu urejanja

1. Prijavite se v Moodle kot Urejevalni učitelj (ali višje) za ta predmet.
2. Odprite predmet.
3. Vklopite **Način urejanja** s stikalom v zgornjem desnem kotu glave predmeta.

Moodle 4.x je nadomestil staro spustno vrstico "Dodaj aktivnost ali vir", ki jo je uporabljal 3.x, z dialogom izbora aktivnosti na celozaslonskem oknu. Moodle 4.5 ohranja isti izbirnik, vendar doda vrstico z zvezdicami/priljubljenimi na vrhu, zato je pripenjanje FastComments enkrat koristno, da ga boste lažje dosegli v kasnejših razdelkih.

#### Dodajte aktivnost FastComments

1. Pomaknite se do razdelka predmeta (teme ali tedna), kjer pripada razprava.
2. Kliknite **Dodaj aktivnost ali vir** na dnu tega razdelka.
3. V dialogu izbora izberite **FastComments**. Če ga ne vidite, skočite na razdelek o težavah spodaj.

Odpre se obrazec z nastavitvami aktivnosti. Polja, ki so pomembna:

- **Activity name** (obvezno). Prikazano na strani predmeta in v dnevniku ocen. Primer: `Week 3 Discussion`.
- **Activity description**. Neobvezno uvodno besedilo, prikazano nad nitjo komentarjev.
- **Show description on course page**. Označite, če želite, da je opis viden brez klika v aktivnost.
- **Preconfigured tool**. Nastavljeno na `FastComments` (samodejno izbrano ob zagonu iz izbirnika). Ne spreminjajte.
- **Launch container**. Nastavite na **New window**. Oglejte si razdelek o težavah, zakaj "Same window" v nekaterih namestitvah Moodla povzroča težave.
- **Tool URL**, **Public key**, **Shared secret**, **Custom parameters**. Pustite prazno. Dinamična registracija je to uredila na ravni mesta.

Pomaknite se na dno in kliknite **Save and return to course** (ali **Save and display**, da takoj odprete aktivnost).

Aktivnost se prikaže kot vrstica v razdelku z ikono FastComments. Študenti kliknejo vrstico, da odprejo nit komentarjev.

#### Vdelajte FastComments neposredno v urejevalnik

Za nit znotraj strani Page, poglavja Book, lekcije Lesson ali katerega koli drugega vira, ki uporablja urejevalnik Atto ali TinyMCE:

1. Odprite vir v načinu urejanja.
2. Postavite kazalec na mesto, kjer naj se prikaže nit.
3. V orodni vrstici urejevalnika kliknite gumb **LTI** / **External tool**. V Atto je označen z "Insert LTI Advantage content". V TinyMCE (privzeto v Moodle 4.3+) je v meniju **More** kot **External tools**.
4. Izberite **FastComments** s seznama orodij.
5. FastComments odpre izbirnik za deep-linking. Potrdite naslov niti in kliknite **Embed**.
6. Urejevalnik vstavi LTI nadomestni blok. Shrani vir.

Vsaka vdelana instanca je ločena nit, ključena na deep-link content item ID, tako da stran s tremi vdelavami FastComments ustvari tri neodvisne niti.

#### Omejite dostop in nastavitve skupin

Standardne nastavitve aktivnosti Moodla veljajo za aktivnosti FastComments:

- **Common module settings** > **Group mode**. Nastavitev na **Separate groups** ali **Visible groups** sama po sebi ne razdeli FastComments na niti za vsako skupino. Skupinski način Moodla samo filtrira dnevnik ocen in seznam članov. Če želite ločeno nit za vsako skupino, dodajte eno FastComments aktivnost za vsako skupino in uporabite **Restrict access**, da omejite obseg vsake.
- **Restrict access** > **Add restriction**. Podpira standardne pogoje Moodla: **Date**, **Grade**, **Group**, **Grouping**, **User profile** in gnezdene nize omejitev. Uporabite **Group**, da zaklenete FastComments aktivnost za eno skupino.
- **Activity completion**. Nastavite na **Students must view this activity to complete it**, če želite sledenje dokončanja. FastComments trenutno ne poroča o dogodku dokončanja nazaj v Moodle razen samega zagona.

#### Preslikava vlog

FastComments prebere zahtevani LTI `roles` claim, ki ga Moodle pošilja ob vsakem zagonu, in ga preslika takole:

- Moodle **Manager** ali **Site administrator** -> FastComments **admin**
- Moodle **Editing teacher** ali **Non-editing teacher** -> FastComments **moderator**
- Moodle **Student** -> FastComments **commenter**
- Moodle **Guest** -> samo za branje

Administratorji lahko izbrišejo kateri koli komentar, prepovejo uporabnike in urejajo nastavitve nití. Moderatorji lahko brišejo in odobravajo komentarje znotraj niti, v katero so se zagnali. Srednje po meri v Moodlu podedujejo preslikavo arhetipa, iz katerega so bili klonirani.

#### Kaj vidijo študenti

Študenti kliknejo FastComments aktivnost (ali se pomaknejo do vdelanega bloka znotraj Page ali Book). Moodle pošlje njihovo identiteto FastComments prek LTI zagona:

- Brez zaslona za prijavo. FastComments jih prijavi z uporabo Moodle računa.
- Njihovo prikazno ime, e-pošta in avatar prihajajo iz Moodla.
- Nit je omejena na `(Moodle site, course, resource link ID)`, tako da enaka aktivnost, podvojena v drugem predmetu, dobi novo nit.
- Vezane (threaded) odgovore, glasovanje in obvestila delujejo enako kot samostojna nit FastComments.

#### Zavarujte javni dostop (priporočeno)

Privzeto so podatki komentarjev FastComments javno berljivi. Kdor koli, ki zna uganiti URL niti ali API končno točko, si lahko ogleda komentarje, tudi zunaj Moodla. Za razprave v predmetih skoraj zagotovo želite omejiti ogled samo na vpisane študente.

Odprite svojo <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">stran za prilagajanje gradnika</a> in ustvarite pravilo z omogočeno možnostjo **Require SSO To View Comments**, nato nastavite varnostno raven na **Secure SSO**, da se niti lahko naložijo samo prek podpisanega LTI zagona.

Oglejte si [Zaščita niti komentarjev s Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) za celoten potek, vključno s tem, kako omejiti pravilo na eno domeno ali stran.

#### Težave v Moodlu

**FastComments manjka v izbirniku aktivnosti.** Skrbnik mesta je orodje registriral, vendar ni nastavil **Tool configuration usage** na **Show in activity chooser and as a preconfigured tool**. Popravite to v **Site administration** > **Plugins** > **Activity modules** > **External tool** > **Manage tools** > ikona zobnika na ploščici FastComments.

**Zagon ne uspe ali prikazuje praznega okvirja, ko je nastavljen na "Same window".** Sejne piškotke Moodla uporabljajo `SameSite=Lax` po privzetku, in nekateri brskalniki jih odstranijo pri med-sitnem POSTu, ki ga LTI 1.3 uporablja za vrnitev iz FastComments. Nastavite **Launch container** na **New window** pri aktivnosti. To je trdna zahteva za vdelan FastComments znotraj Page ali Book, saj pot zagona, vdelan v urejevalnik, vedno odpre novo okno.

**The `iss` claim is the Moodle site URL, not a tenant ID.** FastComments uporablja URL spletnega mesta Moodle (konfiguracijsko vrednost `wwwroot`) kot LTI izdajatelja. Če se vaša instanca Moodla premakne na novo domeno ali spremenite `wwwroot`, obstoječe niti FastComments ostanejo vezane na starega izdajatelja in se ne bodo ujemale z novimi zagoni. Ponovno registrirajte orodje za novi URL in po potrebi migrirajte niti prek upravljanja FastComments.

**Varnostna kopija in obnovitev aktivnosti.** Varnostno kopiranje predmeta in obnova v nov predmet ustvari nove resource link ID-je, zato obnovljene FastComments aktivnosti začnejo z praznimi nitmi. Izvirni predmet obdrži izvirne niti. To je namerno vedenje, ne napaka.

**Moodle 4.5 TinyMCE privzeto.** Moodle 4.5 je privzeto opremljen s TinyMCE kot privzetim urejevalnikom za nove namestitve. Gumb External tool se nahaja v meniju **More** (`...`) namesto v glavni orodni vrstici. Starejša mesta, ki so nadgrajena iz 4.1, obdržijo Atto, razen če je skrbnik spremenil privzeto.