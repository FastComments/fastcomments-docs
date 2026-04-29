Budžetni emailovi za upozorenja šalju se kada potrošnja agenta pređe konfigurabilni procenat njegove kape. Oni idu osobama koje su vlasnici računa.

### Kako rade upozorenja

Svaki agent ima polje **Pragovi upozorenja** na formi za uređivanje. Po defaultu to su `80%` i `100%`. Možete označiti ili poništiti pojedinačne pragove, i možete dodati druge procente.

Kada potrošnja agenta u datom opsegu (dnevnom ili mjesečnom) prvi put u tom periodu pređe prag, platforma pošalje jedan email po primaocu. Ponovno prelazak pragau istom periodu kasnije (npr. potrošnja je pala ispod 80% pa ponovo prešla) **neće** ponovo poslati poruku.

Ovo je po periodu: novi dnevni reset ponovno pokreće logiku prekoračenja praga za taj dan.

### Upozorenja na nivou tenanta

Tenant (račun) ima svoje dnevne i mjesečne kape. Upozorenja na nivou tenanta aktiviraju se na fiksnim pragovima (`80%` i `100%`). Oni se ne mogu konfigurirati po agentu jer važe za cijelog tenanta.

### Primaoci

Budžetna upozorenja se šalju:

- Svakom korisniku označenom kao **Super admin** na tenantu.
- Svakom korisniku označenom kao **Billing Admin** na tenantu.

To uključuje uniju obje uloge - korisnik koji ima obje uloge dobije jedan email.

### Zašto obje uloge

Super admini su obično operateri kojima treba informacija da agent dostiže svoju kapu. Billing administratori su vlasnici računa i trebaju znati o skokovima troškova bez obzira upravljaju li agentima svakodnevno. Da bi zaista uredio agenta (povećao kapu, pauzirao ga), primalac također treba ulogu **Customization Admin** - koja ograničava pristup stranici za uređivanje agenta.

### Isključenje po korisniku

Primaoci koji su se odjavili od administrativnih obavijesti na svom profilu se preskaču. Ovo je isti prekidač odjave koji kontroliše druge administrativne obavijesti.

Ako su **svi** primaoci odjavili obavijesti, upozorenje se zabilježi (nivo upozorenja) i email se ne šalje.

### Sadržaj emaila

Email sadrži:

- Ime agenta za prikaz i interno ime.
- Opseg koji je prešao (npr. "agent daily budget", "agent monthly budget", "account daily budget", "account monthly budget").
- Procenat praga koji je pređen.
- Potrošnju u valuti tenanta.
- Limit u valuti tenanta.
- Jednoklikni potpisani link za prijavu koji primaoca vodi direktno na:
  - Stranicu za uređivanje agenta, za upozorenja u opsegu agenta.
  - Stranicu sa listom AI agenata, za upozorenja na nivou tenanta.

Link je pre-autentifikovan, tako da je primalac jednim klikom udaljen od povećanja kape ili onemogućavanja agenta.

### Kako se pragovi aktiviraju

Platforma prati koji su pragovi već aktivirani u ovom periodu, zasebno za agenta i za tenanta. Dakle:

- Prelazak `80%` pa zatim `100%` u istom periodu aktivira oba, redom.
- Ako se ide direktno sa 0% na 100% u jednom velikom skoku, aktivira se **najviši** pređeni prag (100%), a ne 80%, tako da se dostavlja najozbiljnije upozorenje.

### Kada prestanete dobijati upozorenja

Ako potrošnja agenta nikada ne dostigne sljedeći prag u ovom periodu, nećete dobijati dodatne emailove u tom periodu. Sljedeći dnevni reset (ili mjesečni reset) briše praćenje.

### Onemogućavanje upozorenja

Odznačite prag koji ne želite. Ako ne želite nikakva upozorenja za određenog agenta, odznačite sve procente. Upozorenja na nivou tenanta se ne mogu onemogućiti po agentu (važe za cijelog tenanta).

### Vidi također

- [Budgets Overview](#budgets-overview).
- [Drop Reasons](#drop-reasons) - šta se dešava kada je kapa potpuno dostignuta.
- [Cost Model](#cost-model) - šta se mjeri.