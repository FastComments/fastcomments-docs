Orodja agenta so dejanja, ki jih lahko izvede. Obrazec za urejanje agenta ima razdelek **Dovoljeni klici orodij**, kjer označite orodja, ki jih agent sme uporabljati, in razdelek **Odobritve**, kjer označite dejanja, ki morajo biti potrjena s strani človeka, preden začnejo veljati.

Za vsako orodje obstajajo tri ravni:

- **Prepovedano** - agent ga ne more videti ali uporabljati.
- **Dovoljeno, brez odobritve** - agent ga uporabi neposredno. Zapisano v zgodovini izvajanja.
- **Dovoljeno, z odobritvijo** - klic agenta se postavi v čakalno vrsto za pregled s strani človeka in se izvede le, ko ga človek odobri.

Prepovedana orodja so tiha: agent jih ne more zahtevati in jih platforma takoj zavrne. Orodja, ki zahtevajo odobritev, vedno gredo skozi [predal za odobritve](#approval-workflow).

### Revizijska sled za vsako dejanje

Vsako dejanje agenta je zabeleženo s kratkim utemeljitvijo (1–2 stavka, ki pojasnjujeta zakaj) in oceno zaupanja (0.0–1.0). Obe se prikažeta v [Pogledu podrobnosti izvajanja](#run-detail-view) in pri vsaki [odobritvi](#approval-workflow). Iskanje v pomnilniku je edina izjema v načinu samo‑za‑branje: ni zabeleženo kot dejanje in je vedno na voljo ne glede na dovolitveni seznam.

### Referenca orodij

#### Objavljanje komentarjev

Omogoča agentu objavo komentarja v svojem imenu. Komentar se javno prikaže pod prikaznim imenom agenta. Uporablja ga pozdravni in povzetkovni agent. Obnovljivo - kateri koli moderator lahko odstrani neprimeren komentar. Običajno dovoljeno brez odobritve; nastavite odobritev, če vaša skupnost zahteva, da so vsa javna sporočila pregledana s strani človeka.

#### Glasovanje o komentarjih

Omogoča agentu, da na komentar odda glas za ali proti. Glas se všteje v skupno število glasov za komentar kot vsak drug glas. Večina skupnosti raje nima botov, ki glasujejo; v nobeni začetni predlogi ni omogočeno. Če to dovolite, je glasovanje obnovljivo.

#### Pripni / odstrani pripenjenost komentarja

Omogoča agentu, da komentar pripne na vrh strani ali odpnese že pripet komentar. Platforma ne uveljavlja pravila en pripet komentar na nit, zato mora biti agentu za pripenjanje naročeno, naj najprej odpnese prej pripet komentar. Uporablja se v predlogi Top Comment Pinner. Obnovljivo; običajno dovoljeno brez odobritve.

#### Zakleni / odkleni komentar

Omogoča agentu, da prepreči nadaljnje odgovore na komentar ali jih obnovi. Zaklenjeni komentar ostane viden. Uporabno za umiritev vročih nitk, v kombinaciji z odloženim odklepanjem. Obnovljivo, vendar vidno vaši skupnosti; razmislite o zahtevi odobritve v skupnostih z visokimi vložki.

#### Označi / odstrani spam

Omogoča agentu, da komentar označi kot spam (skrije ga pred bralci in ga pošlje v spam razvrščevalnik) ali počisti to oznako. Temeljno orodje za vsakega moderacijskega agenta. Obnovljivo. Močno priporočamo, da v prvih tednih, ko gradite zaupanje v agenta, zahtevate odobritev.

#### Odobri / prekliči odobritev komentarja

Omogoča agentu, da zadržan komentar prikaže bralcem ali skrije že viden komentar. Najbolj uporabno pri tenants, ki zadržujejo nove komentarje za pregled moderatorja. Preklic odobritve vidnega komentarja je tvegano - razmislite o zahtevi odobritve.

#### Označi komentar kot pregledan

Orodje stanja čakalne vrste: označi komentar kot "moderator (ali agent) si je to ogledal." Ne spreminja vidnosti. Nizko tveganje; redko se zahteva odobritev.

#### Podeli značko

Omogoča agentu, da uporabniku podeli značko iz konfiguracije značk vašega tenant-a. Moderator lahko to razveljavi. Redko se zahteva odobritev. Agent mora poznati ID značke, zato vključite ustrezne ID-je v vaše [smernice skupnosti](#community-guidelines) ali [začetni poziv](#personality-prompt).

#### Pošlji e-pošto

Omogoča agentu pošiljanje navadne besedilne e-pošte z naslova `noreply@fastcomments.com` na naslov, ki ga izbere. Uporabljajte varčno - e-pošta je orodje z največjo trenjivostjo in napačnih sporočil je težko razveljaviti. Močno razmislite o zahtevi odobritve in usmerite potrditvena e-poštna sporočila tistemu, ki ima v lasti predal, na katerega bo agent pošiljal.

#### Shrani / išči pomnilnik agenta

Dve povezani orodji, ki bereta in zapisujeta skupni nabor zapiskov o uporabniku, za katerega je sprožilec deloval. Pomnilnik je deljen med vsemi agenti v vašem tenant-u, zato zapiski triažnega agenta obveščajo odločitve moderacijskega agenta. Iskanje je samo za branje in je vedno na voljo; shranjevanje redko zahteva odobritev. Glejte [Sistem pomnilnika agenta](#agent-memory-system) za celoten načrt.

#### Opozori uporabnika

Pošlje uporabniku zasebno DM opozorilo glede določenega komentarja in hkrati zabeleži opozorilo v pomnilnik agenta. Politika eskalacije platforme je zgrajena okoli tega orodja - najprej opozorite, banajte le, če uporabnik ponovno stori prekršek. Manj pogosto zahteva odobritev kot `ban_user`, vendar razmislite o zahtevi odobritve v prvih tednih delovanja agenta. Glejte [Opozori uporabnika](#tool-warn-user) za celotno stran.

#### Prepovej uporabnika

Najbolj posledično orodje, ki ga lahko agent pokliče. Prepove uporabnika za določen čas, po želji kot shadow ban, po želji tudi prepove IP, po želji tudi izbriše vse uporabnikove komentarje. Dve uničujoči možnosti (IP, delete-all-comments) sta modelu popolnoma skriti, dokler ju ne vključite prek razdelka **Možnosti prepovedi** na obrazcu za urejanje. Tudi če model halucinira parameter, platforma zavrne vrednosti, v katere niste privolili. Glejte [Prepovej uporabnika](#tool-ban-user) za celotno stran.

### Podopcije orodja za prepoved

Orodje za prepoved ponuja dve uničujoči možnosti - delete-all-comments in ban-by-IP - ki sta modelu popolnoma skriti, dokler ju ne vključite prek razdelka **Možnosti prepovedi** na obrazcu za urejanje. Tudi če model halucinira parameter, platforma zavrne vrednosti, v katere niste privolili. Glejte [Prepovej uporabnika](#tool-ban-user).

V območju EU vse prepovedi zahtevajo človeško odobritev (glejte [Skladnost z 17. členom EU DSA](#eu-dsa-compliance)). Močno razmislite o zahtevi odobritve povsod.