Ko agent postavi odobritev v čakalno vrsto, platforma obvesti pregledovalce po e-pošti. Na obrazcu za urejanje to nadzorujeta dve nastavitvi: **koga** obvestiti in **kolikokrat**.

### Kdo: način obveščanja

Dva načina:

- **Vsi skrbniki in moderatorji** (privzeto) - vsak lastnik računa, super skrbnik in administrator moderiranja komentarjev na najemniku je možen pregledovalec.
- **Specifični uporabniki** - ročno izberite seznam iz dvopolnega izbora na obrazcu za urejanje.

V vsakem primeru mora imeti možen pregledovalec račun na najemniku in veljaven e-poštni naslov, da prejme obvestila.

### Kolikokrat: pogostost na uporabnika

Vsak možen pregledovalec v svojem **osebnem profilu** nastavi svojo osebno pogostost obveščanja za odobritve od agentov:

- **Takoj** (privzeto) - en e-poštni naslov za vsako čakajočo odobritev, poslan takoj, ko je odobritev ustvarjena.
- **Vsako uro** - ena povzetna e-pošta na uro, ki povzema vse odobritve, umeščene v tisti uri.
- **Dnevno** - ena povzetna e-pošta na 24 ur.
- **Onemogočeno** - brez e-poštnih sporočil. Uporabnik lahko še vedno pregleda odobritve prek uporabniškega vmesnika mape »Prejeto«; le ne prejema obvestil.

Uporabnik spremeni to nastavitev v svojem profilu, ne na obrazcu za urejanje agenta. To je namerno - en najemnik ima lahko deset agentov, in moderatoru ni treba nastavljati želene pogostosti na vsakem agentu posebej.

### Cron opravila, ki poganjajo povzetke

- **`hourly-agent-approval-digest`** - preverja vsako uro, združuje odobritve, umeščene od zadnjega povzetka posameznega uporabnika, in pošlje eno e-pošto na uporabnika.
- **`daily-agent-approval-digest`** - isto, dnevno.
- **`agent-approval-reaper`** - odstranjuje odobritve, ki so starejše od 90 dni ne glede na stanje.

Upravlja se po prejemniku: uporabnik z urnim razporedom obdeluje urni cron in ga dnevni preskoči (in obratno). Uporabnike s takojšnjo pogostostjo obvesti pot kode za ustvarjanje odobritve, ne cron opravila.

### Stanje deduplikacije

Platforma sledi, katerim uporabnikom je bila o vsaki odobritvi že poslana e-pošta. Ko je uporabnik obveščen (takoj ali v povzetku), mu za isto odobritev ponovno ne pošljejo e-pošte - tudi če med ciklom spremeni pogostost iz takojšnje v dnevno.

### Odobritev iz e-pošte

Vsako obvestilno sporočilo vsebuje enoklikno podpisano povezavo za prijavo, ki pregledovalca neposredno pripelje na stran s podrobnostmi odobritve, že avtenticiranega. Od tam lahko odobrijo, zavrnejo ali odprejo potek [Refine Prompts](#refining-prompts).

### Kaj, če ni skrbnikov

Če je `notifyMode` `All admins and moderators`, ampak najemnik nima super skrbnikov, administratorjev moderiranja komentarjev ali lastnikov računov z veljavnimi e-poštnimi naslovi, platforma zabeleži opozorilo in odobritev je še vedno umeščena v čakalno vrsto - le nihče ni obveščen. Sedela bo v mapi Prejeto, dokler se je kdo ne loti.

Če je `notifyMode` `Specific users`, a niste izbrali nobenih uporabnikov, je izid enak.

### Kaj, če so obvestila o obračunu onemogočena

[Budget Alerts](#budget-alerts) - e-pošta povezana s proračunom - gre na skrbnike obračuna **ne glede na posameznikovo nastavitev pogostosti obveščanja**. To je namerno: prekoračitve proračuna vplivajo na stroške in odgovorna oseba za obračune mora vedeti.

Odobritvena obvestila upoštevajo le nastavitve pogostosti agent-odobritev na uporabnika. Ne preverjajo širše odjave od obvestil skrbnikov - uporabnik, ki se je odjavil od obvestil skrbnikov, bo še vedno prejel e-pošto o odobritvah, če je na seznamu pregledovalcev, razen če ima svojo pogostost agent-odobritev nastavljeno na **Onemogočeno**.

### Oglejte si tudi

- [Approval Workflow](#approval-workflow) za celoten življenjski cikel odobritve.
- [Refining Prompts](#refining-prompts) za potek »nenehno odobravam isto napako«.