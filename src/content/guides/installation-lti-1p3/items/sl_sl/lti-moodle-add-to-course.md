Ta vodnik opisuje, kako dodati FastComments v Moodle 4.x predmet, potem ko je skrbnik strani registriral orodje in ga nastavil tako, da se prikaže v izbiri dejavnosti. Če FastComments še ni registriran, si najprej oglejte vodič za registracijo v Moodlu.

#### Odprite predmet v načinu urejanja

1. Prijavite se v Moodle kot urejevalni učitelj (Editing Teacher) ali z višjimi pravicami za predmet.
2. Odprite predmet.
3. Vklopite **Način urejanja** z gumbom v zgornjem desnem kotu glave predmeta.

Moodle 4.x je nadomestil staro spustno polje »Add an activity or resource«, ki ga je uporabljal 3.x, z izbirnikom dejavnosti na celozaslonskem dialogu. Moodle 4.5 ohranja isti izbirnik, a doda vrstico s priljubljenimi/zvezdicami na vrhu, tako da je pripenjanje FastComments-a enkrat naredi hitrejše dostopanje v kasnejših delih.

#### Dodajte dejavnost FastComments

1. Pomaknite se do odseka predmeta (teme ali tedna), kamor spada razprava.
2. Kliknite **Add an activity or resource** na dnu tega odseka.
3. V dialogu izbirnika izberite **FastComments**. Če ga ne vidite, preskočite na razdelek o pogostih težavah spodaj.

Odpre se obrazec za nastavitve dejavnosti. Polja, ki so pomembna:

- **Activity name** (obvezno). Prikazano na strani predmeta in v dnevniku ocen. Primer: `Week 3 Discussion`.
- **Activity description**. Neobvezno uvodno besedilo, prikazano nad nitjo komentarjev.
- **Show description on course page**. Označite, če želite, da je opis viden brez klika v dejavnost.
- **Preconfigured tool**. Nastavljeno na `FastComments` (samodejno izbrano ob zagonu iz izbirnika). Ne spreminjajte.
- **Launch container**. Nastavite na **Novo okno**. Poglejte razdelek o pogostih težavah, zakaj »Isto okno« v nekaterih Moodle nameščanjih povzroči napake.
- **Tool URL**, **Public key**, **Shared secret**, **Custom parameters**. Pustite prazno. Dinamična registracija je to uredila na ravni strani.

Pomaknite se na dno in kliknite **Save and return to course** (ali **Save and display**, če želite takoj odpreti dejavnost).

Dejavnost se prikaže kot vrstica v odseku z ikono FastComments. Študenti kliknejo vrstico za odprtje niti komentarjev.

#### Vdelajte FastComments neposredno v urejevalnik

Za nit znotraj strani (Page), poglavja v Knjigi (Book chapter), lekcije (Lesson) ali katerekoli druge vsebine, ki uporablja urejevalnik Atto ali TinyMCE:

1. Odprite vsebino v načinu urejanja.
2. Postavite kazalec na mesto, kjer naj se prikaže nit.
3. V orodni vrstici urejevalnika kliknite gumb **LTI** / **External tool**. V Atto je označen kot »Insert LTI Advantage content«. V TinyMCE (privzeto v Moodle 4.3+) je pod menijem **More** kot **External tools**.
4. Izberite **FastComments** s seznama orodij.
5. FastComments odpre izbirnik za deep-linking. Potrdite naslov niti in kliknite **Embed**.
6. Urejevalnik vstavi LTI nadomestni blok. Shrani vsebino.

Vsaka vdelana instanca je ločena nit, določena z ID-jem vsebine deep-link, zato stran s tremi vdelavami FastComments dobi tri neodvisne niti.

#### Omejite dostop in nastavitve skupin

Standardne nastavitve dejavnosti v Moodlu veljajo za FastComments dejavnosti:

- **Common module settings** > **Group mode**. Nastavitev na **Separate groups** ali **Visible groups** sama po sebi ne razdeli FastComments v niti za posamezne skupine. Način skupin v Moodlu le filtrira dnevnik ocen in seznam članov. Če želite ločeno nit za vsako skupino, dodajte eno FastComments dejavnost za skupino in uporabite **Restrict access**, da omejite vsako posebej.
- **Restrict access** > **Add restriction**. Podpira standardne Moodle pogoje: **Date**, **Grade**, **Group**, **Grouping**, **User profile** in gnezdene sklope omejitev. Uporabite **Group**, da zaklenete FastComments dejavnost za eno skupino.
- **Activity completion**. Nastavite na **Students must view this activity to complete it**, če želite spremljanje dokončanja. FastComments trenutno ne poroča Moodle-u o dogodku dokončanja razen zagona.

#### Preslikava vlog

FastComments prebere LTI trditev roles, ki jo Moodle pošlje ob vsakem zagonu, in jo preslika tako:

- Moodle **Manager** ali **Site administrator** -> FastComments **admin**
- Moodle **Editing teacher** ali **Non-editing teacher** -> FastComments **moderator**
- Moodle **Student** -> FastComments **commenter**
- Moodle **Guest** -> samo za branje

Administratorji lahko izbrišejo katerikoli komentar, prepovejo uporabnike in urejajo nastavitve niti. Moderatorji lahko brišejo in potrjujejo komentarje znotraj niti, v katero so se zagnali. Prilagojene Moodle vloge podedujejo preslikavo arhetipa, iz katerega so bile klonirane.

#### Kaj vidijo študenti

Študenti kliknejo FastComments dejavnost (ali se pomaknejo do vdelanega bloka znotraj Strani ali Knjige). Moodle jim pošlje identiteto v FastComments preko LTI zagona:

- Brez zaslona za prijavo. FastComments jih prijavi z njihovim Moodle računom.
- Njihovo prikazno ime, e-pošta in avatar prihajajo iz Moodla.
- Nit je omejena na (Moodle site, course, resource link ID), tako da ista dejavnost, podvojena v drugem predmetu, dobi novo nit.
- Večstopenjske odgovore, glasovanje in obvestila delujejo enako kot pri samostojni FastComments niti.

#### Moodle pasti

**FastComments manjka v izbiri dejavnosti.** Skrbnik strani je registriral orodje, vendar ni nastavil **Tool configuration usage** na **Show in activity chooser and as a preconfigured tool**. To popravite pod **Site administration** > **Plugins** > **Activity modules** > **External tool** > **Manage tools** > ikona zobnika na ploščici FastComments.

**Zagon ne uspe ali prikaže praznega okvira, ko je nastavljen na "Isto okno".** Sejne piškotke Moodla uporabljajo `SameSite=Lax` privzeto, in nekateri brskalniki jih odstranijo pri med-spletni POST zahtevi, ki jo LTI 1.3 uporablja za vrnitev iz FastComments. Nastavite **Launch container** na **Novo okno** pri dejavnosti. To je stroga zahteva za vdelane FastComments znotraj Strani ali Knjige, saj pot do zagona, vgrajena v urejevalnik, vedno odpre novo okno.

**Trditev `iss` je URL Moodle strani, ne ID najemnika.** FastComments uporablja URL Moodle strani (konfiguracijsko vrednost `wwwroot`) kot LTI izdajatelja. Če se vaš Moodle premakne na novo domeno ali spremenite `wwwroot`, obstoječe FastComments niti ostanejo vezane na starega izdajatelja in se ne ujemajo z novimi zagoni. Ponovno registrirajte orodje za nov URL in po potrebi migrirajte niti preko FastComments skrbniške strani.

**Varnostna kopija in obnovitev dejavnosti.** Varnostno kopiranje predmeta in njegova obnova v nov predmet ustvari nove ID-je resource link, tako da obnovljene FastComments dejavnosti začnejo z praznimi nitmi. Izvirni predmet obdrži izvirne niti. To je pričakovano vedenje, ne napaka.

**Moodle 4.5 TinyMCE kot privzeto.** Moodle 4.5 vsebuje TinyMCE kot privzeti urejevalnik za nova namestitev. Gumb External tool je pod menijem **More** (`...`) namesto v glavni orodni vrstici. Starejše strani, ki so nadgrajene z različice 4.1, obdržijo Atto, razen če skrbnik ni spremenil privzetega.