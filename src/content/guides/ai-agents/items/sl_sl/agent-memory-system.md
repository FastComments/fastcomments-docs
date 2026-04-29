Agent memory is a tenant-scoped, **shared** key-value pool that every agent in your tenant can read from and write to. It exists so agents can carry context across runs.

### Zakaj pomnilnik obstaja

LLM kontekst je za posamezen zagon. Brez pomnilnika agent, ki uporabniku izreče opozorilo, ob naslednjem srečanju z istim uporabnikom ne more vedeti za to opozorilo. Politika eskalacije platforme - "opozori pred prepovedjo" - je odvisna od tega, da agent najde prejšnje opozorilo. Pomnilnik omogoča, da to deluje.

### Dve vrsti pomnilnika

- **WARNING** - zapisuje se samodejno kot del toka [`warn_user`](#tool-warn-user). Agent ne piše zapisov `WARNING` ročno; ti so stranski učinek opozarjanja uporabnika.
- **NOTE** - zapisuje se z [`save_memory`](#tools-overview). Splošen kontekst, ki ga agent želi, da ga bodo vedeli prihodnji agenti.

Politika eskalacije posebej išče zapise vrste `WARNING`, ko odloča, ali je prepoved utemeljena.

### Omejeno na najemnika, deljeno med agenti

Vsi agenti v vašem najemniku delijo **en sklop pomnilnika**. Opomba, ki jo shrani Agent A, je vidna klicem `search_memory` Agenta B. To je namenoma - želite, da opombe triažnega agenta obveščajo odločitve moderirnega agenta.

`tenantId` nastavi izvrševalec iz lastnega najemnika agenta - nikoli iz LLM argumentov - zato so puščanja pomnilnika med najemniki po zasnovi nemogoča.

### Kaj vsebuje zapis pomnilnika

Vsak zapis pomnilnika vsebuje:

- **Kateri agent ga je zapisal**, in kdaj.
- **O kom je** - uporabnik, ki ga ta pomnilnik opisuje. Agenta tega ne more izmišljati; platforma to avtomatsko izpolni iz sprožilca, ki je aktiviral agenta.
- **Skrit signal alt-racuna** - platforma prav tako (zasebno) zabeleži IP prstni odtis izvirnega komentarja, tako da lahko prihodnja iskanja pomnilnika prikažejo opombe o drugih računih, ki objavljajo z istega IP. Prstni odtis agentu ali LLM ni nikoli prikazan.
- **Sama opomba** - do 2000 znakov prostega besedila.
- **Oznake** za pridobivanje - do 10 kratkih oznak.
- **Vrsta** - bodisi opozorilo ali splošna opomba.
- **Neobvezna povezava do komentarja** - če je pomnilnik vezan na konkreten komentar.

### Vedenje iskanja

[`search_memory`](#tools-overview) vrne do 25 zapisov, razvrščenih od najnovejših navzdol, samodejno omejenih na (sprožilčevega uporabnika) ALI (druge račune na IP sprožilca). Rezultati so tudi omejeni na skupno 8000 znakov po vsebini vseh vrnjenih zapisov - starejši vnosi se zavržejo, če je omejitev dosežena.

Agent ne posreduje `userId` ali `targetIpHash`. Obe vrednosti nastavi izvrševalec.

### Trajnost

Pomnilnik nima **TTL**. Zapisi obstajajo dokler niso izrecno odstranjeni. Zapisi WARNING o uporabniku se namenoma nikoli samodejno ne brišejo - zgodovina eskalacij mora biti najdljiva za nedoločen čas, sicer je platformina preverba "iskanje pred prepovedjo" brez pomena.

Trije načini, kako se pomnilnik odstrani:

- Moderator izbriše osnovni komentar - vsak pomnilnik, vezan na ta komentar, se posledično izbriše.
- Uporabnik je izbrisan - vsi zapisi pomnilnika o tem uporabniku se odstranijo v isti transakciji.
- Vaš najemnik je izbrisan.

Danes ne obstaja skrbniški uporabniški vmesnik za brisanje posameznih zapisov pomnilnika.

### Pomnilnik v poskusnem izvajanju

Agenti v poskusnem izvajanju **ne** pišejo pomnilnika. To je zasnovano namenoma: hipotetične odločitve agenta v poskusnem izvajanju ne bi smele onesnažiti skupnega sklada pomnilnika. Branje nazaj preko `search_memory` v poskusnem izvajanju deluje normalno - agent lahko vidi resnične spomine od živih agentov - le dodajati jih ne more.

### Pomnilnik pri ponovitvah

Enako kot pri poskusnem izvajanju: agenti v ponovitvah ne pišejo pomnilnika. Ponovitve so samo pregled. Glejte [Test Runs (Replays)](#test-runs-replays).

### Povzetek omejitev

| Limit | Value |
|---|---|
| Memory content max length | 2000 chars |
| Memory tag max length | 64 chars |
| Memory tags max count | 10 |
| Memory query max length | 200 chars |
| Memory search result limit | 25 records |
| Memory search total content cap | 8000 chars |

### Oglejte si tudi

- [Tool: save_memory](#tools-overview) za zapisovanje.
- [Tool: search_memory](#tools-overview) za branje.
- [Tool: warn_user](#tool-warn-user) - edino orodje, ki zapiše pomnilnik vrste WARNING.
- [Tool: ban_user](#tool-ban-user) - sistemski prompt zahteva, da se pred tem pokliče `search_memory`.