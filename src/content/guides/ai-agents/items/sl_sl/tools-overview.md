Agentova **orodja** so dejanja, ki jih lahko izvede. Oblika za urejanje agenta ima razdelek **Allowed tool calls**, kjer označite orodja, ki jih sme agent uporabljati, in razdelek **Approvals**, kjer označite dejanja, ki morajo pred uveljavitvijo dobiti človeško odobritev.

Obstajajo tri ravni za katerokoli orodje:

- **Disallowed** - agent ga ne more videti ali uporabiti.
- **Allowed, no approval** - agent ga uporablja neposredno. Zabeleženo v zgodovini zagona.
- **Allowed, with approval** - klic agenta se uvrsti v čakalnik za človekov pregled in se izvede šele, ko človek odobri.

Onemogočena orodja so tiha: agent jih ne more zahtevati in jih platforma popolnoma zavrne. Orodja, ki zahtevajo odobritev, vedno gredo skozi [approvals inbox](#approval-workflow).

### Revizijska sled pri vsakem dejanju

Vsako dejanje, ki ga agent izvede, je zabeleženo s kratko utemeljitvijo (1–2 stavka, ki pojasnita zakaj) in oceno zaupanja (0.0–1.0). Obe se prikažeta v [Run Detail View](#run-detail-view) in pri vsakem [approval](#approval-workflow). Iskanje v memoriji je edina izjema, ki je samo za branje: ni zabeleženo kot dejanje in je vedno na voljo ne glede na seznam dovoljenih.

### Referenca orodij

#### Objavljanje komentarjev

Dovoljuje agentu, da objavi komentar v svojem imenu. Komentar se javno prikaže pod prikaznim imenom agenta. Uporablja se pri agentih za pozdravljanje in povzemanje. Razveljavljivo - vsak moderator lahko odstrani neustrezen komentar. Če vaša skupnost zahteva, da vsako javno sporočilo pregleda človek, ga zaklenite za odobritvijo.

#### Urejanje komentarja

Dovoljuje agentu, da prepiše besedilo komentarja znotraj obsega. Izvirno besedilo ostane v revizijskem zapisu komentarja. Rezervirajte za ozka področja - cenzura osebnih podatkov, ki jih je uporabnik pomotoma razkril, ali popravek lastnega prejšnjega odgovora agenta. Ne za preoblikovanje mnenj ali omilitev tona. Celotno stran si oglejte pri [Edit comment](#tool-edit-comment).

#### Glasovanje o komentarjih

Dovoljuje agentu, da da glas za ali proti komentarju. Glas se šteje v skupno število glasov komentarja enako kot kateri koli drug glas. Večina skupnosti raje nima botov, ki glasujejo; v nobeni začetni predlogi to ni omogočeno. Če to dovolite, je glasovanje razveljavljivo.

#### Pripni / odpni komentar

Dovoljuje agentu, da pripne komentar na vrh strani ali odpniti že pripet komentar. Platforma ne uveljavlja pravila ene pripetosti na nit, zato bi morali agenta, namenjenega pripenjanju, navoditi, naj najprej odpne prejšnji pripeti komentar. Da odkrije, kaj je že pripeto na isti strani, lahko agent pokliče orodje samo za branje `get_pinned_comments` (glej spodaj). Uporablja se v predlogi Top Comment Pinner.

#### Zakleni / odkleni komentar

Dovoljuje agentu, da prepreči nadaljnje odgovore pod komentarjem ali obnovi odgovarjanje. Zaklenjen komentar ostane viden. Uporabno za ohlajanje razgrete debata, pogosto v kombinaciji z zamikom za odklep. Da odkrije, kaj je trenutno zaklenjeno na isti strani, lahko agent pokliče orodje samo za branje `get_locked_comments` (glej spodaj).

#### Oznaka / odstranitev oznake za spam

Dovoljuje agentu, da označi komentar kot spam (skrije ga pred bralci in poda kot vhod za klasifikator spama) ali odstrani to oznako. Temeljno orodje za vsakega moderatorja. Razveljavljivo.

#### Odobri / odstrani odobritev komentarja

Dovoljuje agentu, da prikaže zadržan komentar bralcem ali skrije že videnega. Najbolj uporabno pri najemnikih, ki zadržujejo nove komentarje za pregled moderatorja.

#### Označi komentar kot pregledan

Orodje za stanje v vrsti: označi komentar kot "moderator (ali agent) si ga je ogledal." Ne spremeni vidljivosti. Nizek vložek; redko pogojevano.

#### Podeli značko

Dovoljuje agentu, da uporabniku podeli značko, ki ste jo konfigurirali za vašega najemnika. Moderator jo lahko razveljavi. Ko je to orodje omogočeno, agent vidi značke vašega najemnika in sam izbere pravo, zato vam ni treba vnašati identifikatorjev značk v smernice skupnosti ali začetni poziv. Da usmerite katera značka se podeli za katero vedenje, v pozivu sklicujte značke po njihovem **Display Label**.

#### Pošlji e-pošto

Dovoljuje agentu, da pošlje navadno besedilo e-pošte avtorju komentarja v obsegu sprožilca. Agent nikoli ne vidi e-poštnega naslova prejemnika - izbere komentar in platforma dostavi na naslov, ki ga je avtor vnesel ob objavi. Od-naslov je v imenu vaše blagovne znamke (z DKIM), če domena komentarja ustreza konfigurirani domeni, sicer uporabi privzeto platformo. Uporabljajte varčno - e-pošta je orodje z največjo trenjem in slabe e-pošte je težko razveljaviti.

#### Shrani / poišči agentovo pomnilnik

Dve soparno orodji, ki bereta in zapisujeta skupni sklad zapiskov o uporabniku, za katerega je sprožilec deloval. Pomnilnik je deljen med vsemi agenti v vašem najemniku, zato zapiski triažnega agenta obveščajo odločitve moderatornega agenta. Iskanje je samo za branje in vedno na voljo; shranjevanje je redko pogojevano. Celoten dizajn si oglejte pri [Agent Memory System](#agent-memory-system).

#### Get pinned comments / Get locked comments

Dve orodji samo za branje, ki izpišeta pripete (ali zaklenjene) komentarje na isti strani (`urlId`), na kateri se je sprožilec aktiviral. Ne sprejemata argumentov - stran se bere iz konteksta sprožilca, zato se agent ne more preusmeriti na druge strani. Uporabite jih, kadar agent potrebuje ukrep nad komentarjem, ki je že pripet ali zaklenjen - običajno prvi klic pred `unpin_comment` ali `unlock_comment`, ali pred pripenjanjem novega komentarja, da se lahko najprej odpne obstoječega.

Vsako orodje je posamezno omogočeno v razdelku **Allowed tool calls** (administrator označi `List pinned comments on the current page` ali `List locked comments on the current page`). Ne morejo biti pogojevana z odobritvijo - orodja samo za branje nimajo stranskih učinkov, ki bi zahtevali odobritev. Klicanje le-teh ni zabeleženo kot dejanje v zgodovini zagona; le posledični klic `unpin_comment` / `unlock_comment` / `pin_comment` (če do njega pride) se pojavi. Seznam je omejen na zadnjih 20 zadetkov na klic.

Pomembno za razumevanje: ko eno od teh orodij vrne commentId, se ta commentId doda v obseg agenta za posamezen zagon, zato se naslednji klic `unpin_comment` / `unlock_comment` preveri glede na varnostno kontrolo ciljev orodij platforme. Brez prejšnjega klica na orodje za odkrivanje agent ne more ukrepati nad komentarji, ki niso že v neposrednem obsegu sprožilca. Zato agent, ki odpne pripete komentarje, običajno dobi omogočeni obe orodji (npr. `get_pinned_comments` skupaj z `unpin_comment`).

#### Opozori uporabnika

Pošlje zasebno sporočilo (DM) z opozorilom uporabniku o določenem komentarju in atomarno zabeleži opozorilo v agentovem pomnilniku. Politika stopnjevanja platforme je zgrajena okoli tega orodja - najprej opozori, prepove šele, če uporabnik ponovi kršitev. Celotna stran pri [Warn user](#tool-warn-user).

#### Prepove uporabnika

Najusodnejše orodje, ki ga lahko agent pokliče. Prepove uporabnika za določen čas, po potrebi kot shadow ban, po potrebi tudi prepove IP, po potrebi tudi izbriše vse uporabnikove komentarje. Dve uničujoči možnosti (IP, izbris-vseh) sta omogočeni le z dodatnima potrdiloma na obrazcu za urejanje. V regiji EU vse prepovedi zahtevajo človeško odobritev (glej [EU DSA Article 17 Compliance](#eu-dsa-compliance)). Celotna stran pri [Ban user](#tool-ban-user).

### Podmožnosti orodja Ban

Orodje Ban izpostavi dve uničujoči možnosti - delete-all-comments in ban-by-IP - ki sta modelu popolnoma skriti, dokler jih ne vklopite preko razdelka **Ban options** na obrazcu za urejanje. Tudi če model navidezno "izumi" parameter, platforma zavrne vrednosti, ki jih niste izbrali. Oglejte si [Ban user](#tool-ban-user).