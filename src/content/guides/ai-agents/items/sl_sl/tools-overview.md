Agentova **orodja** so dejanja, ki jih lahko izvede. Obrazec za urejanje agenta ima razdelek **Dovoljeni klici orodij**, kjer označite orodja, ki jih sme ta agent uporabljati, in razdelek **Odobritve**, kjer označite dejanja, ki morajo biti pred izvedbo potrjena s človekovo odobritvijo.

Za vsako orodje obstajajo tri ravni:

- **Prepovedano** - agent ga ne more videti niti uporabljati.
- **Dovoljeno, brez odobritve** - agent ga uporablja neposredno. Zapisano v zgodovini zagona.
- **Dovoljeno, z odobritvijo** - klic agenta se uvrsti v čakalno vrsto za človeški pregled in se izvede šele, ko ga človek odobri.

Prepovedana orodja so tiha: agent jih ne more zahtevati in platforma jih takoj zavrne. Orodja, ki zahtevajo odobritev, vedno gredo skozi [mapo odobritev](#approval-workflow).

### Revizijska sled pri vsakem dejanju

Vsako dejanje, ki ga agent izvede, je zabeleženo s kratkim utemeljenjem (1–2 stavka, ki pojasnjujeta, zakaj) in oceno zaupanja (0.0–1.0). Obe se prikažeta v [Pogledu podrobnosti zagona](#run-detail-view) in pri vsaki [odobritvi](#approval-workflow). Iskanje v spominu je edina izjema le za branje: ni zabeleženo kot dejanje in je vedno na voljo ne glede na dovolilni seznam.

### Referenca orodij

#### Objavljanje komentarjev

Dovoli agentu, da objavi komentar v svojem imenu. Komentar se javno prikaže pod imenom agenta. Uporabljajo ga agenti za pozdrav in agenti za povzemanje. Povratno—kakor koli moderator lahko odstrani slab komentar. Običajno dovoljeno brez odobritve; omejite ga z odobritvijo, če vaša skupnost zahteva, da so vsa javna sporočila pregledana s strani človeka.

#### Urejanje komentarja

Dovoli agentu, da prepiše besedilo komentarja v obsegu dovoljenega. Izvirno besedilo se ohrani v revizijskem zapisu komentarja. Rezervirajte za ozke primere—brisanje PII, ki ga je uporabnik razkril, ali popravilo agentovega lastnega prejšnjega odgovora. Ne za prepisovanje mnenj ali omiljanje tona. **Močno razmislite o zahtevi za odobritev.** Oglejte si [Uredi komentar](#tool-edit-comment) za celotno stran.

#### Glasovanje o komentarjih

Dovoli agentu, da glasuje za ali proti komentarju. Glas se šteje v skupno število glasov komentarja enako kot kateri koli drug glas. Večina skupnosti raje nima botov, ki glasujejo; v nobeni začetni predlogi ni omogočeno. Če to dovolite, je glasovanje povratno.

#### Pripni / odstrani pripenjanje komentarja

Dovoli agentu, da pripne komentar na vrh strani ali odstrani pripenjanje že pripečenega komentarja. Platforma ne uveljavlja pravila en pripet komentar na nit, zato je priporočljivo, da agent, ki pripenja, najprej odstrani prej pripeti komentar. Uporablja se v predlogi Top Comment Pinner. Povratno; običajno dovoljeno brez odobritve.

#### Zakleni / odkleni komentar

Dovoli agentu, da prepreči nadaljnje odgovore pod komentarjem ali povrne možnosti odgovarjanja. Zaklenjen komentar ostane viden. Uporabno za umirjanje razgrete razprave, v kombinaciji z odloženim odklepanjem. Povratno, a vidno vaši skupnosti; razmislite o zahtevi za odobritev v skupnostih z visokim vložkom.

#### Označi / odstrani označbo neželene pošte

Dovoli agentu, da označi komentar kot neželeno pošto (skrije ga pred bralci in ga pošlje klasifikatorju neželene pošte) ali počisti to oznako. Osnovno orodje za vsakega moderatornega agenta. Povratno. Močno razmislite o zahtevi za odobritev v prvih tednih, ko gradite zaupanje v agenta.

#### Odobri / prekliči odobritev komentarja

Dovoli agentu, da prikaže zadržan komentar bralcem ali skrije že videnega. Najbolj uporabno pri tenantih, ki zadržijo nove komentarje za pregled moderatorjev. Visok vložek pri preklicu odobritve že vidnega komentarja—razmislite o zahtevi za odobritev.

#### Označi komentar kot pregledan

Orodje za stanje vrste: označi komentar kot "moderator (ali agent) si je to ogledal." Ne spreminja vidnosti. Nizek vložek; redko omejeno z odobritvijo.

#### Podeli značko

Dovoli agentu, da uporabniku podeli značko iz konfiguracije značk vašega tenanta. Moderator lahko razveljavi. Redko omejeno z odobritvijo. Agent mora poznati ID značke, zato vključite ustrezne ID-je v vaše [smernice skupnosti](#community-guidelines) ali v [začetni poziv](#personality-prompt).

#### Pošlji e-pošto

Dovoli agentu, da pošlje navadno besedilno e-pošto z `noreply@fastcomments.com` na naslov po lastni izbiri. Uporabljajte previdno—e-pošta ima največje trenje in slabe e-pošte je težko razveljaviti. Močno razmislite o zahtevi za odobritev in usmerite odobritvena sporočila k lastniku poštnega predala, na katerega bo agent pošiljal.

#### Shrani / poišči spomin agenta

Dve povezani orodji, ki bereta in pišeta v skupni zbirki zapiskov o uporabniku, za katerega je sprožilo sprožilo. Spomin je deljen med vsemi agenti v vašem tenant-u, tako da opombe triažnega agenta vplivajo na odločitve moderatornega agenta. Iskanje je samo za branje in je vedno na voljo; shranjevanje je redko omejeno. Oglejte si [Sistem spomina agenta](#agent-memory-system) za celoten načrt.

#### Opozori uporabnika

Pošlje zasebno opozorilo v DM uporabniku glede določenega komentarja in atomarno zabeleži opozorilo v spomin agenta. Politika eskalacije platforme je zgrajena okoli tega orodja—najprej opozori, prepovej le, če uporabnik ponovno prestopi. Manj pogosto omejeno kot `ban_user`, vendar razmislite o omejitvi v prvih tednih delovanja agenta. Oglejte si [Opozori uporabnika](#tool-warn-user) za celotno stran.

#### Prepoved uporabnika

Najbolj posledično orodje, ki ga agent lahko pokliče. Prepove uporabnika za določeno obdobje, po želji kot shadow ban, po želji tudi prepove IP, po želji tudi izbriše vse uporabnikove komentarje. Dve uničevalni možnosti (IP, delete-all) sta skriti modelu, dokler ju ne omogočite preko dodatnih privolitev v razdelku **Možnosti prepovedi** na obrazcu za urejanje. V regiji EU vse prepovedi zahtevajo človeško odobritev (glejte [Skladnost z EU DSA - členom 17](#eu-dsa-compliance)). Močno razmislite o zahtevi za odobritev povsod. Oglejte si [Prepoved uporabnika](#tool-ban-user) za celotno stran.

### Pod-opcije orodja Ban

Orodje Ban razkriva dve uničevalni možnosti - delete-all-comments and ban-by-IP - ki sta modelu popolnoma skriti, dokler ju ne omogočite preko razdelka **Možnosti prepovedi** na obrazcu za urejanje. Tudi če model halucinira parameter, platforma zavrne vrednosti, za katere niste dali privolitve. Oglejte si [Prepoved uporabnika](#tool-ban-user).