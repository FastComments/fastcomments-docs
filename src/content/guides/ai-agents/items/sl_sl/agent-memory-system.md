Spomin agenta je obseg na najemnika (tenant-scoped), **skupni** ključ-vrednost skladišče, iz katerega lahko vsak agent v vašem najemniku bere in vanj piše. Obstaja zato, da lahko agenti prenašajo kontekst med zagoni.

### Zakaj spomin obstaja

Kontekst LLM je vezan na posamezen zagon. Brez spomina agent, ki izda opozorilo uporabniku, nima načina, da bi naslednjič, ko vidi istega uporabnika, vedel za to opozorilo. Politika eskalacije platforme - "opozori pred izključitvijo" - je odvisna od tega, da agent najde prejšnje opozorilo. Spomin je tisto, kar to omogoča.

### Dve vrsti spomina

- **WARNING** - zapisan samodejno kot del toka [`warn_user`](#tool-warn-user). Agent ne zapisuje zapisov `WARNING` ročno; ti so stranski učinek opozorila uporabnika.
- **NOTE** - zapisan z [`save_memory`](#tools-overview). Splošni kontekst, ki ga agent želi, da bodoči agenti vedo.

Politika eskalacije pri odločanju, ali je izključitev upravičena, išče posebej zapise `WARNING`.

### Obseg na najemnika, skupno agentom

Vsi agenti v vašem najemniku si delijo **en skladišče spomina**. Zapisek, ki ga shrani Agent A, je viden klicem `search_memory` Agenta B. To je namerno - želite, da zapiski agenta za triažo obveščajo odločitve moderatorskega agenta.

`tenantId` nastavi izvajalnik iz najemnika samega agenta - nikoli iz argumentov LLM - tako da puščanja spomina med najemniki so po konstrukciji nemogoča.

### Kaj vsebuje zapis spomina

Vsak vnos spomina vsebuje:

- **Kateri agent ga je zapisal**, in kdaj.
- **O kom gre** - uporabnik, ki ga ta spomin opisuje. Agent tega ne more izmisliti; platforma ga samodejno izpolni iz tistega, kar je sprožilo agenta.
- **Skriti signal alternativnega računa** - platforma zasebno zapiše tudi prstni odtis IP izvornega komentarja, tako da bodoča iskanja po spominu lahko razkrijejo zapise o drugih računih, ki objavljajo z istega IP. Prstnega odtisa agent ali LLM nikoli ne vidi.
- **Sam zapisek** - do 2000 znakov prostega besedila.
- **Oznake** za iskanje - do 10 kratkih oznak.
- **Vrsto** - bodisi opozorilo bodisi splošen zapisek.
- **Neobvezno povezavo do komentarja** - če je spomin vezan na določen komentar.

### Vedenje iskanja

[`search_memory`](#tools-overview) vrne do 25 zapisov, razvrščenih od najnovejšega, samodejno obseganih na (uporabnika sprožilca) ALI (druge račune na IP sprožilca). Rezultati so omejeni tudi po skupno 8000 znakih vse vrnjene vsebine - starejši vnosi so odstranjeni, če je omejitev dosežena.

Agent ne podaja `userId` ali `targetIpHash`. Oboje nastavi izvajalnik.

### Trajnost

Spomin **nima TTL**. Zapisi ostanejo, dokler niso izrecno odstranjeni. Zapisi WARNING o uporabniku se namenoma nikoli ne samodejno izbrišejo - zgodovina eskalacije mora biti nedoločeno najdena, sicer je preverjanje "išči pred izključitvijo" platforme brez smisla.

Trije načini, kako se spomin odstrani:

- Moderator izbriše izvorni komentar - vsak spomin, vezan na ta komentar, se kaskadno izbriše.
- Uporabnik je izbrisan - vsi vnosi spomina o tem uporabniku se odstranijo v isti transakciji.
- Vaš najemnik je izbrisan.

Danes ni skrbniškega vmesnika za brisanje posameznih zapisov spomina.

### Spomin v poskusnem zagonu (dry-run)

Agenti v poskusnem zagonu **ne** zapisujejo spomina. To je po načrtu: hipotetične odločitve agenta v poskusnem zagonu ne smejo onesnažiti skupnega skladišča spomina. Branje prek `search_memory` deluje v poskusnem zagonu normalno - agent lahko vidi prave spomine od živih agentov - le dodajati jim ne more.

### Spomin v ponovitvah (replays)

Enako kot pri poskusnem zagonu: agenti ponovitev ne zapisujejo spomina. Ponovitve so samo predogled. Glejte [Testni zagoni (ponovitve)](#test-runs-replays).

### Povzetek omejitev

| Omejitev | Vrednost |
|---|---|
| Največja dolžina vsebine spomina | 2000 znakov |
| Največja dolžina oznake spomina | 64 znakov |
| Največje število oznak spomina | 10 |
| Največja dolžina poizvedbe spomina | 200 znakov |
| Omejitev rezultatov iskanja spomina | 25 zapisov |
| Skupna omejitev vsebine iskanja spomina | 8000 znakov |

### Glejte tudi

- [Orodje: save_memory](#tools-overview) za pisanje.
- [Orodje: search_memory](#tools-overview) za branje.
- [Orodje: warn_user](#tool-warn-user) - edino orodje, ki zapisuje spomin vrste WARNING.
- [Orodje: ban_user](#tool-ban-user) - sistemski poziv zahteva, da se `search_memory` pokliče pred tem.