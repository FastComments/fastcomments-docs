E-poštna opozorila o proračunu se sprožijo, ko poraba agenta preseže nastavljivi odstotek njegovega limita. Pošljejo se osebam, ki so odgovorne za račun.

### Kako opozorila delujejo

Vsak agent ima na obrazcu za urejanje polje **Mejne vrednosti opozoril**. Privzeto so to `80%` in `100%`. Lahko označite ali odznačite posamezne meje in dodate druge odstotke.

Ko poraba agenta v določenem obsegu (dnevnem ali mesečnem) prvič v tem obdobju preseže mejo, platforma pošlje eno e-pošto na prejemnika. Ponovno preseganje iste meje kasneje v istem obdobju (npr. poraba je padla pod 80 % in se nato vrnila nad njo) ne povzroči ponovnega pošiljanja.

To velja za posamezno obdobje: dnevna ponastavitev ponovno zažene logiko prečkanja mej za ta dan.

### Opozorila na ravni najemnika

Najemnik (račun) ima svoje dnevne in mesečne omejitve. Opozorila na ravni najemnika se sprožijo pri fiksnih mejah (`80%` in `100%`). Te niso nastavljive za posameznega agenta, ker veljajo za celoten najemnik.

### Prejemniki

Opozorila o proračunu se pošljejo:

- Vsak uporabnik, označen kot **Super admin** na najemniku.
- Vsak uporabnik, označen kot **Billing Admin** na najemniku.

To vključuje združitev obeh vlog - uporabnik z obema vlogama prejme eno e-poštno sporočilo.

### Zakaj obe vlogi

Super admini so običajno operaterji, ki morajo vedeti, da agent dosega svoj limit. Billing admini upravljajo račun in morajo vedeti za nenadne povečane stroške, ne glede na to, ali dnevno upravljajo agente. Za dejansko urejanje agenta (povišanje limita, začasno zaustavitev) mora prejemnik imeti tudi vlogo **Customization Admin** - ta vloga omogoča dostop do strani za urejanje agenta.

### Posameznikova možnost izključitve

Prejemniki, ki so na svojem profilu odjavili obvestila skrbnikov, se preskočijo. To je isti preklop za odjavo, ki nadzoruje tudi druga obvestila skrbnikov.

Če so **vsi** prejemniki odjavljeni, se opozorilo zabeleži (nivo opozorila) in e-pošta ni poslana.

### Vsebina e-pošte

E-pošta vsebuje:

- Prikazno ime agenta in interno ime.
- Obseg, ki je bil presežen (npr. "dnevni proračun agenta", "mesečni proračun agenta", "dnevni proračun računa", "mesečni proračun računa").
- Preseženi odstotek meje.
- Poraba v valuti najemnika.
- Omejitev v valuti najemnika.
- Enoklikovno podpisano prijavno povezavo, ki preusmeri prejemnika neposredno na:
  - stran za urejanje agenta, za opozorila na ravni agenta.
  - stran s seznamom AI agentov, za opozorila na ravni najemnika.

Povezava je vnaprej overjena, zato je prejemnik le en klik oddaljen od povišanja limita ali onemogočitve agenta.

### Kako se sprožijo meje

Platforma spremlja, katere meje so bile v tem obdobju že sprožene, ločeno za agenta in za najemnika. Torej:

- Presežek 80% in nato 100% v istem obdobju sproži obe, v tem vrstnem redu.
- Če se pojavi skok od 0% neposredno na 100%, se sproži **najvišja** presežena meja (100%), ne 80%, tako da je dostavljeno najresnejše opozorilo.

### Kdaj prenehate prejemati opozorila

Če poraba agenta tega obdobja ne doseže naslednje meje, v tem obdobju ne prejmete več e-poštnih sporočil. Naslednja dnevna ponastavitev (ali mesečna ponastavitev) počisti spremljanje.

### Onemogočanje opozoril

Odstranite kljukico ob meji, ki je ne želite. Če ne želite nobenih opozoril za določenega agenta, odznačite vse odstotke. Opozoril v obsegu najemnika ni mogoče onemogočiti za posameznega agenta (veljajo za celoten najemnik).

### Glej tudi

- [Pregled proračunov](#budgets-overview).
- [Razlogi za zavrnitev](#drop-reasons) - kaj se zgodi, ko je limit popolnoma dosežen.
- [Model stroškov](#cost-model) - kaj se meri.

---