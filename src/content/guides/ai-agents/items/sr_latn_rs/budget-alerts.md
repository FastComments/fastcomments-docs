Emailovi sa upozorenjima o budžetu se šalju kada potrošnja agenta pređe konfigurabilan procenat njegovog limita. Šalju se osobama koje plaćaju račun.

### Kako rade upozorenja

Svaki agent ima polje **Alert thresholds** na formi za izmenu. Podrazumevano je to `80%` i `100%`. Možete čekirati ili odčekirati pojedinačne pragove, i možete dodati druge procente.

Kada potrošnja agenta u datom opsegu (dnevnom ili mesečnom) prvi put u tom periodu pređe prag, platforma šalje jedan email po primaocu. Ponovno prelaska praga kasnije u istom periodu (npr. potrošnja je pala ispod 80% pa se vratila iznad) **ne** pokreće ponovno slanje.

Ovo važi po periodu: novi dnevni reset ponovo pokreće logiku prelaska pragova za taj dan.

### Upozorenja na nivou tenanta

Tenant (nalog) ima svoje dnevne i mesečne limite. Upozorenja na nivou tenanta aktiviraju se na fiksnim pragovima (`80%` i `100%`). Ona nisu konfigurabilna po agentu jer se odnose na ceo tenant.

### Primaoci

Upozorenja o budžetu se šalju:

- Svakom korisniku označenom kao **Super admin** na tenantu.
- Svakom korisniku označenom kao **Billing Admin** na tenantu.

To uključuje uniju dve uloge - korisnik sa obe uloge dobija jedan email.

### Zašto obe uloge

Super admini su obično operateri kojima treba da znaju da agent dostiže svoj limit. Billing admini su vlasnici fakture i treba da znaju za skokove troškova bez obzira na to da li svakodnevno upravljaju agentima. Da bi zaista izmenili agenta (podigli limit, pauzirali ga), primalac takođe treba da ima ulogu **Customization Admin** - koja štiti stranicu za izmenu agenta.

### Odjava po korisniku

Primaoci koji su se odjavili sa admin notifikacija na svom profilu se preskaču. To je isti prekidač za odjavu koji kontroliše i druge admin notifikacije.

Ako su **svi** primaoci odjavljeni, upozorenje se zapisuje u log (nivo upozorenja) i email se ne šalje.

### Sadržaj emaila

Email sadrži:

- **Prikazano ime agenta** i interno ime.
- **Opseg** koji je pređen (npr. "agent daily budget", "agent monthly budget", "account daily budget", "account monthly budget").
- **Procenat praga** koji je pređen.
- **Potrošnju** u valuti tenanta.
- **Limit** u valuti tenanta.
- **Jednoklikni potpisani link za prijavu** koji primaoca vodi direktno na:
  - Stranicu za izmenu agenta, za upozorenja na nivou agenta.
  - Stranicu sa listom AI agenata, za upozorenja na nivou tenanta.

Link je unapred autentifikovan, tako da je primalac jedan klik udaljen od podizanja limita ili onemogućavanja agenta.

### Kako se pragovi aktiviraju

Platforma prati koji su pragovi već aktivirani u ovom periodu, odvojeno za agenta i za tenant. Dakle:

- Prelazak 80% pa zatim 100% u istom periodu aktivira oba, redom.
- Prelazak direktno sa 0% na 100% u jednom velikom skoku aktivira **najviši** pređeni prag (100%), a ne 80%, tako da se dostavlja najozbiljnije upozorenje.

### Kada prestajete da dobijate upozorenja

Ako potrošnja agenta nikada ne dostigne sledeći prag u ovom periodu, ne dobijate dalje emailove u ovom periodu. Sledeći dnevni reset (ili mesečni reset) briše praćenje.

### Onemogućavanje upozorenja

Odčekirajte prag koji ne želite. Ako ne želite nijedno upozorenje za konkretnog agenta, odčekirajte sve procente. Upozorenja na nivou tenanta ne mogu se onemogućiti po agentu (važe za ceo tenant).

### Vidi takođe

- [Budgets Overview](#budgets-overview).
- [Drop Reasons](#drop-reasons) - šta se dešava kada se limit u potpunosti dostigne.
- [Cost Model](#cost-model) - šta se meri.