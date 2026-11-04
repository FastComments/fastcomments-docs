**Orodja** agenta so dejanja, ki jih lahko izvede. Obrazec za urejanje agenta ima razdelek **Dovoljena klicanja orodij**, kjer označite orodja, ki jih ta agent sme uporabljati, ter razdelek **Potrditve**, kjer označite dejanja, ki morajo pred uveljavitvijo dobiti človeško odobritev.

Vsako orodje ima tri ravni:

- **Nedovoljeno** - agent ga ne vidi in ne more uporabiti.
- **Dovoljeno, brez odobritve** - agent ga uporabi neposredno. Zabeleženo v zgodovini zagonov.
- **Dovoljeno, z odobritvijo** - klic agenta je postavljen v čakalno vrsto za človeški pregled in se izvede šele, ko ga človek odobri.

Nedovoljena orodja so tiha: agent jih ne more zahtevati, platforma pa jih zavrne na mestu. Orodja z odobritvijo vedno gredo skozi [vhodni predal odobritev](#approval-workflow).

### Sled revizije pri vsakem dejanju

Vsako dejanje agenta je zabeleženo s kratko utemeljitvijo (1-2 stavka, ki pojasnjujeta razlog) in oceno zaupanja (0.0-1.0). Oboje je prikazano v [Pogledu podrobnosti zagona](#run-detail-view) in pri vsaki [odobritvi](#approval-workflow). Iskanje po spominu je edina izjema, ki je samo za branje: ni zabeleženo kot dejanje in je vedno na voljo ne glede na seznam dovoljenih orodij.

### Referenca orodij

#### Objavljanje komentarjev

Agentu omogoča objavo komentarja v svojem imenu. Komentar je javno prikazan pod prikaznim imenom agenta. Uporabljajo ga agenti pozdravljalnik in povzetkovalec. Razveljavi se - vsak moderator lahko odstrani slab komentar. Omogočite odobritev, če vaša skupnost zahteva, da je vsaka javno vidna sporočila pregledal človek.

#### Urejanje komentarja

Agentu omogoča prepis besedila komentarja v obsegu. Izvirno besedilo je ohranjeno v revizijskem dnevniku komentarja. Rezervirajte za ozke primere - odstranjevanje osebnih podatkov, ki jih je uporabnik razkril, ali popravek prejšnjega odgovora agenta samega. Ni namenjeno prepisovanju mnenj ali omiljanju tona. Glejte [Urejanje komentarja](#tool-edit-comment) za celotno stran.

#### Glasovanje o komentarjih

Agentu omogoča glasovanje za ali proti komentarju. Glas se prišteva k skupnemu številu glasov komentarja kot vsak drug glas. Večina skupnosti ne želi, da boti glasujejo; v nobeni začetni predlogi ni omogočeno. Če to dovolite, je glasovanje razveljavljivo.

#### Pripni / odpni komentar

Agentu omogoča pripeti komentar na vrh strani ali odpeti že pripetega. Platforma ne uveljavlja pravila enega pripetega komentarja na nit, zato je treba agenta za pripinanje navoditi, da najprej odpine prejšnji pripeti komentar. Da ugotovi, kaj je na isti strani že pripeto, lahko agent pokliče orodje samo za branje `get_pinned_comments` (glejte spodaj). Uporablja ga predloga Top Comment Pinner.

#### Zakleni / odkleni komentar

Agentu omogoča preprečevanje nadaljnjih odgovorov pod komentarjem ali njihovo obnovo. Zaklenjeni komentar ostane viden. Uporabno za ohlajevanje vročih niti, skupaj z odloženim odklepanjem. Da ugotovi, kaj je na isti strani trenutno zaklenjeno, lahko agent pokliče orodje samo za branje `get_locked_comments` (glejte spodaj).

#### Označi / odznači kot spam

Agentu omogoča označiti komentar kot spam (skrije ga pred bralci in ga posreduje klasifikatorju spama) ali odstraniti to oznako. Osnovno orodje za vsakega moderatorskega agenta. Razveljavi se.

#### Odobri / prekliči odobritev komentarja

Agentu omogoča prikaz zadržanega komentarja bralcem ali skriti že videnega. Najbolj uporabno na najemnikih, ki zadržijo nove komentarje za moderatorski pregled.

#### Označi komentar kot pregledan

Orodje za stanje čakalne vrste: označi komentar kot "moderator (ali agent) si je to ogledal." Ne spremeni vidnosti. Nizka tveganja; redko omejeno z odobritvijo.

#### Podeli značko

Agentu omogoča podeliti uporabniku značko, ki ste jo nastavili za svojega najemnika. Moderator jo lahko razveljavi. Ko je to orodje omogočeno, agent lahko vidi značke vašega najemnika in sam izbere pravo, zato vsmeho vodnikih skupnosti ali začetnem pozivu ni treba prilepiti identifikatorjev značk. Da usmerite, katera značka se podeli za katero vedenje, se v pozivu sklicujte na značke po njihovi **Prikazni oznaki**.

#### Pošlji e-pošto

Agentu omogoča poslati e-poštno sporočilo v obliki navadnega besedila avtorju komentarja v obsegu sprožilca. Agent nikoli ne vidi e-poštnega naslova prejemnika - izbere komentar, platforma pa dostavi na naslov, ki ga je komentator pustil ob objavi. Naslov pošiljatelja je vašega najemnika blagovna znamka pošiljatelja (z DKIM), ko se domena komentarja ujema z nastavljeno domeno, sicer pa privzeti naslov platforme. Uporabljajte zmerno - e-pošta je orodje z najvišjo stopnjo trenja, slaba e-poštna sporočila pa je težko razveljaviti.

#### Shrani / išči spomin agenta

Dva povezana orodja, ki bereta in pišeta v skupni zbirko opomb o uporabniku, za katerega se je sprožilec aktiviral. Spomin je v skupni rabi med vsemi agenti v vašem najemniku, zato opombe agenta za razvrščanje obveščajo odločitve moderatorskega agenta. Iskanje je samo za branje in vedno na voljo; shranjevanje je redko omejeno z odobritvijo. Glejte [Sistem spomina agenta](#agent-memory-system) za celotno zasnovo.

#### Pridobi pripete komentarje / Pridobi zaklenjene komentarje

Dve orodji za odkrivanje, ki sta samo za branje, in izpišeta pripete (ali zaklenjene) komentarje na isti strani (`urlId`), na kateri se je sprožilec aktiviral. Ne sprejemata argumentov - stran se prebere iz konteksta sprožilca, zato agent ne more preklopiti na druge strani. Uporabite ju, ko mora agent ukrepati glede komentarja, ki je že pripet ali zaklenjen - tipično kot prvi klic pred `unpin_comment` ali `unlock_comment`, ali pred pripetjem novega komentarja, da se obstoječi najprej odpihne.

Vsako orodje je v **Dovoljenih klicanjih orodij** ločeno omejeno (skrbnik označi `List pinned comments on the current page` ali `List locked comments on the current page`). Ne moreta biti omejena z odobritvijo - orodja samo za branje nimajo stranskih učinkov za odobriti. Njuno klicanje ni zabeleženo kot dejanje v zgodovini zagonov; prikaže se le posledični klic `unpin_comment` / `unlock_comment` / `pin_comment` (če pride do njega). Seznam je omejen na največ 20 najnovejših zadetkov na klic.

Pomembno za razumevanje: ko eno od teh orodij vrne commentId, se ta commentId doda v obseg agenta za ta zagon, tako da se nadaljnji klic `unpin_comment` / `unlock_comment` preveri proti varnostnemu preverjanju cilja orodja na platformi. Brez predhodnega klica orodja za odkrivanje agent ne more ukrepati glede komentarjev, ki niso že v neposrednem obsegu sprožilca. Torej agent za odpinjanje tipično dobi omogočeni oba orodji (npr. `get_pinned_comments` plus `unpin_comment`).

#### Opozori uporabnika

Pošlje zasebno opozorilo v DM uporabniku glede določenega komentarja in atomično zabeleži opozorilo v spomin agenta. Politika eskalacije platforme je zgrajena okoli tega orodja - najprej opozori, prepovej šele, če uporabnik ponovi prekršek. Glejte [Opozori uporabnika](#tool-warn-user) za celotno stran.

#### Prepovej uporabnika

Najbolj obsežno orodje, ki ga agent lahko pokliče. Prepove uporabnika s fiksno trajanjem, po možnosti kot senčno prepoved, po možnosti tudi prepove IP naslov in po možnosti tudi izbriše vse komentarje uporabnika. Dve destruktivni možnosti (IP, izbriši-vse) so omejene z dodatnimi privolitvami na obrazcu za urejanje. V regiji EU vse prepovedi zahtevajo človeško odobritev (glejte [Skladnost z EU DSA člen 17](#eu-dsa-compliance)). Glejte [Prepovej uporabnika](#tool-ban-user) za celotno stran.

### Podmožnosti orodja za prepoved

Orodje za prepoved razkriva dve destruktivni možnosti - izbriši-vse-komentarje in prepoved-po-IP - ki so modelu popolnoma skrite, dokler jih ne omogočite v razdelku **Možnosti prepovedi** na obrazcu za urejanje. Tudi če model halucinira parameter, platforma zavre vrednosti, ki jih niste omogočili. Glejte [Prepovej uporabnika](#tool-ban-user).