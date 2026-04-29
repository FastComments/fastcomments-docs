Agentova **orodja** so dejanja, ki jih lahko izvede. Obrazec za urejanje agenta ima razdelek **Dovoljeni klici orodij**, kjer označite orodja, ki jih ta agent lahko uporablja, in razdelek **Odobritve**, kjer označite dejanja, ki morajo biti potrjena s strani človeka, preden stopijo v veljavo.

Za vsako orodje obstajajo tri stopnje:

- **Onemogočeno** - agent ga ne vidi in ga ne more uporabljati.
- **Dovoljeno, brez odobritve** - agent ga uporablja neposredno. Zabeleženo v zgodovini zagona.
- **Dovoljeno, z odobritvijo** - agentov klic se uvrsti v čakalnik za človeški pregled in se izvede šele po odobritvi.

Onemogočena orodja so nevidna: agent jih ne more zahtevati in platforma jih odločno zavrne. Orodja, ki zahtevajo odobritev, vedno gredo skozi [mapo za odobritve](#approval-workflow).

### Revizijska sled pri vsakem dejanju

Vsako dejanje, ki ga agent izvede, je zabeleženo s kratko utemeljitvijo (1–2 stavka, zakaj) in oceno zaupanja (0.0–1.0). Obe se prikažeta v [Pogledu podrobnosti zagona](#run-detail-view) in ob vsaki [odobritvi](#approval-workflow). Iskanje v spominu je edina izjema samo za branje: ni zabeleženo kot dejanje in je vedno na voljo ne glede na seznam dovoljenih.

### Referenca orodij

#### Objavljanje komentarjev

Dovoli agentu, da objavi komentar v svojem imenu. Komentar se javno prikaže pod prikaznim imenom agenta. Uporabljajo ga agenti za pozdravljanje in povzemanje. Vračljivo - vsak moderator lahko odstrani neprimeren komentar. Običajno dovoljeno brez odobritve; omejite ga z odobritvijo, če vaša skupnost zahteva, da mora vsako javno sporočilo pregledati človek.

#### Urejanje komentarja

Dovoli agentu, da prepiše besedilo komentarja, ki je v obsegu. Izvirno besedilo je shranjeno v revizijskem dnevniku komentarja. Rezervirajte za ozke primere - odstranjevanje osebno identificirajočih informacij (PII), ki jih je uporabnik razkril, ali popravke prejšnjega odgovora agenta. Ni za preoblikovanje mnenj ali omilitev tona. **Močno razmislite o omejitvi z odobritvijo.** Oglejte si [Uredi komentar](#tool-edit-comment) za celotno stran.

#### Glasovanje o komentarjih

Dovoli agentu, da komentarju da glas za ali proti. Glas se šteje v skupno število glasov komentarja enako kot vsak drug glas. Večina skupnosti raje nima botov, ki glasujejo; v nobeni začetni predlogi ni omogočeno. Če to dovolite, je glasovanje vračljivo.

#### Pripni / odstrani pripenjanje komentarja

Dovoli agentu, da pripne komentar na vrh strani ali odstrani pripenjanje že pripetega komentarja. Platforma ne uveljavlja pravila en pripetek na nit, zato bi moral agent, ki pripenja, najprej odstraniti prejšnji pripet komentar. Uporablja ga predloga Top Comment Pinner. Vračljivo; običajno dovoljeno brez odobritve.

#### Zakleni / odkleni komentar

Dovoli agentu, da prepreči nadaljnje odgovore pod komentarjem ali obnovi odgovore. Zaklenjen komentar ostane viden. Uporabno za umirjanje razgretih nití, v kombinaciji z odloženim odklepanjem. Vračljivo, vendar vidno vaši skupnosti; razmislite o omejitvi z odobritvijo v primerih z visokimi vložki.

#### Označi / odznači kot spam

Dovoli agentu, da označi komentar kot neželeno pošto (skrivanje pred bralci in hranjenje klasifikatorja spama) ali odstrani to zastavico. Osnovno orodje za katerega koli moderacijskega agenta. Vračljivo. Močno razmislite o omejitvi z odobritvijo v prvih tednih, ko gradite zaupanje v agenta.

#### Odobri / odvzemi odobritev komentarja

Dovoli agentu, da z bralci prikaže zadržan komentar ali skrije že videnega. Najbolj uporabno pri najemnikih, ki zadržujejo nove komentarje za moderatorjev pregled. Visoka tveganja pri odvzemu odobritve videnega komentarja - razmislite o omejitvi z odobritvijo.

#### Označi komentar kot pregledan

Orodje stanja v čakalni vrsti: označi komentar kot "moderator (ali agent) je to pregledal." Ne spreminja vidnosti. Nizka tveganja; redko omejeno.

#### Podeli značko

Dovoli agentu, da uporabniku podeli značko, ki ste jo konfigurirali za svoj najemnik. Moderator lahko razveljavi. Redko omejeno. Ko je to orodje omogočeno, agent vidi značke vašega najemnika in sam izbere pravo, zato vam ni treba lepiti identifikatorjev značk v smernice skupnosti ali začetni poziv. Če želite usmerjati, katera značka se podeli za določeno vedenje, v pozivu sklicujte se na značke po njihovem **prikaznem oznaki**.

#### Pošlji e-pošto

Dovoli agentu, da pošlje navadno besedilno e-pošto avtorju komentarja v obsegu sprožilca. Agent nikoli ne vidi prejemnikovega e-poštnega naslova - izbere komentar, platforma pa dostavi na naslov, ki ga je tisti komentator pustil ob objavi. Naslov pošiljatelja je blagovna znamka vašega najemnika (z DKIM), kadar domena komentarja ustreza konfigurirani domeni, sicer je uporabljen privzeti naslov platforme. Uporabljajte varčno - e-pošta ima največje trenje in slabe e-pošte je težko razveljaviti. Močno razmislite o omejitvi z odobritvijo in usmerjanju e-pošt o odobritvah na osebo, ki ima v lasti prejemni predal, kamor bo agent na koncu pošiljal.

#### Shrani / poišči spomin agenta

Dve povezani orodji, ki bereta in zapisujeta skupni bazen opomb o uporabniku, za katerega se sprožilec aktivira. Spomin je deljen med vsemi agenti v vašem najemniku, zato zapiski triažnega agenta vplivajo na odločitve moderacijskega agenta. Iskanje je samo za branje in vedno na voljo; shranjevanje je redko omejeno. Oglejte si [Sistem spomina agenta](#agent-memory-system) za celoten dizajn.

#### Opozori uporabnika

Pošlje zasebno opozorilo v DM uporabniku glede določenega komentarja in atomarno zabeleži opozorilo v spominu agenta. Politika eskalacije platforme temelji na tem orodju - najprej opozori, prepovej le, če uporabnik znova stopi v prekršek. Redkeje omejeno kot `ban_user`, vendar razmislite o omejitvi v prvih tednih življenja agenta. Oglejte si [Opozori uporabnika](#tool-warn-user) za celotno stran.

#### Prepovej uporabnika

Najbolj posledično orodje, ki ga agent lahko pokliče. Prepove uporabnika za določeno obdobje, po želji kot shadow ban, po želji tudi prepove IP, po želji tudi izbriše vse uporabnikove komentarje. Dve destruktivni možnosti (IP, delete-all-comments) sta omejeni za dodatno izrecno vključitev na obrazcu za urejanje. Tudi če model zmoti parameter, platforma zavrne vrednosti, za katere niste dali soglasja. Oglejte si [Prepovej uporabnika](#tool-ban-user) za celotno stran.

### Podmožnosti orodja za prepoved

Orodje Ban razkriva dve destruktivni možnosti - delete-all-comments in ban-by-IP - ki sta modelu popolnoma skriti, dokler ju ne vključite prek razdelka **Ban options** na obrazcu za urejanje. Tudi če model izmisljuje parameter, platforma zavrne vrednosti, za katere niste dali soglasja. Oglejte si [Prepovej uporabnika](#tool-ban-user).