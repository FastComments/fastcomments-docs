E-pošte s upozorenjima o proračunu šalju se kada potrošnja agenta prijeđe konfigurabilni postotak njegovog limita. Šalju se osobama koje plaćaju račun.

### Kako upozorenja rade

Svaki agent ima polje **Pragovi upozorenja** na obrascu za uređivanje. Zadano je to `80%` i `100%`. Možete označiti ili odznačiti pojedinačne pragove, te možete dodati druge postotke.

Kada potrošnja agenta u danom opsegu (dnevnom ili mjesečnom) prvi put u tom razdoblju prijeđe prag, platforma šalje jednu e-poštu po primatelju. Ponovno prelazak praga kasnije u istom razdoblju (npr. potrošnja je pala ispod 80% i vratila se iznad) **ne** pokreće ponovno slanje.

Ovo je po razdoblju: novi dnevni reset ponovno pokreće logiku prelaska pragova za taj dan.

### Upozorenja u opsegu najmoprimca (tenanta)

Tenant (račun) ima vlastite dnevne i mjesečne limite. Upozorenja u opsegu tenanta aktiviraju se na fiksnim pragovima (`80%` i `100%`). Ona nisu konfigurabilna po agentu jer se odnose na cijeli tenant.

### Primatelji

Upozorenja o proračunu šalju se:

- Svakom korisniku označenom kao **Super admin** na tenantu.
- Svakom korisniku označenom kao **Billing Admin** na tenantu.

To uključuje uniju te dvije uloge - korisnik s obje uloge dobiva jednu e-poštu.

### Zašto obje uloge

Super admini su obično operateri kojima treba znati da agent dostiže svoj limit. Administratori naplate vlasnici su računa i trebaju znati o skokovima troškova bez obzira na to whether upravljaju agentima iz dana u dan. Da bi stvarno uredili agenta (podigli limit, pauzirali ga), primatelj također treba ulogu **Customization Admin** - koja štiti stranicu za uređivanje agenta.

### Odjava po korisniku

Primatelji koji su se odjavili od administratorskih obavijesti na svom profilu se preskaču. To je isti prekidač za odjavu koji kontrolira ostale administratorske obavijesti.

Ako su **svi** primatelji odjavljeni, upozorenje se zapisuje u dnevnik (razina upozorenja) i e-pošta se ne šalje.

### Sadržaj e-pošte

E-pošta sadrži:

- **Prikazno ime agenta** i interno ime.
- **Opseg** koji je prekoračen (npr. "dnevni proračun agenta", "mjesečni proračun agenta", "dnevni proračun računa", "mjesečni proračun računa").
- **Postotak praga** koji je prekoračen.
- **Potrošnju** u valuti tenanta.
- **Limit** u valuti tenanta.
- **Prijavnu poveznicu s jednim klikom** koju je potpisao token, koja vodi primatelja izravno na:
  - Stranicu za uređivanje agenta, za upozorenja u opsegu agenta.
  - Stranicu popisa AI agenata, za upozorenja u opsegu tenanta.

Poveznica je unaprijed autentificirana, pa je primatelj jedan klik udaljen od podizanja limita ili onemogućavanja agenta.

### Kako se pragovi aktiviraju

Platforma prati koji su se pragovi već aktivirali u ovom razdoblju, odvojeno za agenta i za tenant. Dakle:

- Prelazak 80% pa 100% u istom razdoblju aktivira oba, redoslijedom.
- Prelazak izravno s 0% na 100% u jednom velikom skoku aktivira **najviši** prekoračeni prag (100%), a ne 80%, tako da se dostavlja najozbiljnije upozorenje.

### Kada prestajete primati upozorenja

Ako potrošnja agenta nikad ne dosegne sljedeći prag u ovom razdoblju, ne dobivate daljnje e-pošte u ovom razdoblju. Sljedeći dnevni reset (ili mjesečni reset) briše praćenje.

### Onemogućavanje upozorenja

Odznačite prag koji ne želite. Ako ne želite nikakva upozorenja za određenog agenta, odznačite sve postotke. Upozorenja u opsegu tenanta ne mogu se onemogućiti po agentu (vaze za cijeli tenant).

### Također pogledajte

- [Pregled proračuna](#budgets-overview).
- [Razlozi odbacivanja](#drop-reasons) - što se događa kada je limit u potpunosti dosegnut.
- [Model troškova](#cost-model) - što se mjeri.