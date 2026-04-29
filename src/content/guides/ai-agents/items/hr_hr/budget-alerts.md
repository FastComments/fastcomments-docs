E-mailovi upozorenja o proračunu šalju se kada potrošnja agenta prijeđe konfigurabilni postotak njegovog ograničenja. Šalju se osobama koje su odgovorne za račun.

### Kako rade upozorenja

Svaki agent ima polje **Alert thresholds** na obrascu za uređivanje. Zadano je `80%` i `100%`. Možete označiti ili poništiti pojedinačne pragove, i možete dodati druge postotke.

Kada potrošnja agenta u određenom opsegu (dnevnom ili mjesečnom) prvi put u tom razdoblju prijeđe prag, platforma pošalje jedan e-mail po primatelju. Ponovno prijeći prag kasnije u istom razdoblju (npr. potrošnja je pala ispod `80%` pa se ponovno vratila iznad) NE znači ponovno slanje.

Ovo je po razdoblju: novi dnevni reset ponovno pokreće logiku detektiranja prijeđenih pragova za taj dan.

### Tenant-scope alerts

Tenant (account) ima svoje dnevne i mjesečne limite. Upozorenja u opsegu tenant-a aktiviraju se na fiksnim pragovima (`80%` i `100%`). Oni se ne mogu konfigurirati po agentu jer se primjenjuju na cijeli tenant.

### Recipients

Upozorenja o proračunu šalju se:

- Svakom korisniku označenom kao **Super admin** na tenant-u.
- Svakom korisniku označenom kao **Billing Admin** na tenant-u.

To uključuje uniju dviju uloga - korisnik s obje uloge dobiva jedan e-mail.

### Zašto obje uloge

Super admini su obično operateri koji trebaju znati da agent dostiže svoj limit. Billing administratori upravljaju računom i trebaju znati o skokovima troškova bez obzira upravljaju li agentima iz dana u dan. Da bi zapravo uredio agenta (povećao limit, pauzirao ga), primatelj također treba ulogu **Customization Admin** - koja kontrolira pristup stranici za uređivanje agenta.

### Per-user opt-out

Primatelji koji su se odjavili od administratorskih obavijesti u svom profilu se preskaču. Ovo je isti prekidač odjave koji kontrolira i druge administratorske obavijesti.

Ako su **svi** primatelji odjavljeni, upozorenje se evidentira (na razini upozorenja) i ne šalje se e-mail.

### Email content

E-mail sadrži:

- Prikazno ime agenta i interno ime.
- Opseg koji je prijeđen (npr. "agent daily budget", "agent monthly budget", "account daily budget", "account monthly budget").
- Postotak praga koji je prijeđen.
- Iskorištenje u valuti tenant-a.
- Limit u valuti tenant-a.
- Link za prijavu s jednim klikom i potpisom koji vodi primatelja izravno na:
  - Stranicu za uređivanje agenta, za upozorenja u opsegu agenta.
  - Stranicu s popisom AI agenata, za upozorenja u opsegu tenant-a.

Link je prethodno autentificiran, tako da je primatelj udaljen samo jedan klik od povećanja limita ili onemogućavanja agenta.

### Kako se pragovi aktiviraju

Platforma prati koji su pragovi već aktivirani u tom razdoblju, zasebno za agenta i za tenant. Dakle:

- Prijeći `80%` pa potom `100%` u istom razdoblju aktivira oba, redom.
- Ići izravno s `0%` na `100%` u jednom velikom skoku aktivira **najviši** prijeđeni prag (`100%`), a ne `80%`, tako da se isporučuje najozbiljnije upozorenje.

### Kada prestanete primati upozorenja

Ako potrošnja agenta tijekom tog razdoblja nikada ne dosegne sljedeći prag, ne dobivate daljnje e-mailove u tom razdoblju. Sljedeći dnevni reset (ili mjesečni reset) briše praćenje.

### Isključivanje upozorenja

Poništite kvačicu kod praga koji ne želite. Ako ne želite nikakva upozorenja za određenog agenta, poništite sve postotke. Upozorenja u opsegu tenant-a ne mogu se onemogućiti po agentu (primjenjuju se na cijeli tenant).

### Vidi također

- [Budgets Overview](#budgets-overview).
- [Drop Reasons](#drop-reasons) - što se događa kada je limit u potpunosti dosegnut.
- [Cost Model](#cost-model) - što se mjeri.