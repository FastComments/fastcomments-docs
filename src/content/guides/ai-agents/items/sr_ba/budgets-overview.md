Svaki agent ima limite potrošnje. Platforma prestaje dispatch-ovati agenta kada se limit dostigne i nastavlja kada period istekne.

### Dva opsega, dva perioda

Postoje četiri limita ukupno - dva opsega (po agentu, po tenantu) u kombinaciji sa dva perioda (dnevni, mjesečni).

| Scope | Period | Where you set it |
|---|---|---|
| Dnevni (po agentu) | UTC dan | Forma za uređivanje agenta -> **Budžet** -> **Dnevni budžet** |
| Mjesečni (po agentu) | kalendarski mjesec | Forma za uređivanje agenta -> **Budžet** -> **Mjesečni budžet** |
| Dnevni (po tenantu) | UTC dan | Izvedeno iz plana (nema odvojenog korisničkog unosa) |
| Mjesečni (po tenantu) | kalendarski mjesec | Izvedeno iz plana (nema odvojenog korisničkog unosa) |

Trigger se izvršava samo ako mu **sva četiri limita** dozvoljavaju. Prvi limit koji se potroši je onaj koji zaustavlja trigger.

### Valuta

Budžeti po agentu unose se u valuti vašeg računa.

### Šta se dešava kada se limit dostigne

- Trigger se evidentira kao **dropped** sa [drop reason](#drop-reasons) kao što su `agentDaily` ili `tenantMonthly`.
- Broj onemogućenih pojavljuje se na [Analytics page](#analytics-page) pod "Triggers skipped (this month)".
- Ne pravi se poziv prema LLM-u; na sam onemogućeni trigger se ne troše tokeni.
- Status agenta ostaje nepromijenjen - samo što ne može dispatch-ovati dok period ne istekne.

### Prelazak perioda

- **Dnevni** limiti se resetuju u ponoć po UTC.
- **Mjesečni** limiti se resetuju na početku svakog kalendarskog mjeseca, po UTC.

Ne postoji prenos neiskorištenog budžeta u naredni period.

### Strogi limit vs. meka upozorenja

Limiti su **strogi**. Ne postoji režim "preći za 10% uz upozorenje". Kada se limit dostigne, dispatch prestaje.

"Meki" dio su email obavijesti [Budget Alerts](#budget-alerts) - dobivate email na podesivim pragovima (zadano 80% i 100%) kako biste mogli podići limit prije nego što saobraćaj počne opadati.

### Gdje pročitati trenutnu upotrebu

- [Analytics page](#analytics-page) - upotreba budžeta po agentu i za cijeli tenant sa oznakama limita.
- Sekcija **Stats** u formi za uređivanje agenta.
- Prikaz liste (broj čekajućih odobrenja i nedavne izvršavanja je na kartici agenta).

### Odabir budžeta

Nekoliko pravila za orijentaciju:

- **Novi agent** - odredite budžet. Pratite [Run History](#run-history) sedmicu dana. Prilagodite na osnovu opaženog troška po izvršavanju × očekivanog volumena okidača.
- **Agent sa velikim prometom** (npr. okidač za novi komentar na prometnom sajtu) - dnevni limit je ono što hvata nekontrolisani loop. Odaberite dnevni limit koji je 2-3× veći od očekivane dnevne potrošnje tako da normalan prometni dan stane unutar njega.
- **Agent za sumiranje ili koji troši mnogo konteksta** - trošak po izvršavanju je visok. Postavite stroži dnevni limit da spriječite da loš dan 'probije' mjesečni budžet.

### Zaobilaženje budžeta za replay-e

[Test runs / replays](#test-runs-replays) podliježu svom **vlastitom** strogiom limitu (podešenom na formi za replay, odvojeno od dnevnih/mjesečnih limita agenta), I limitima agenta i tenant-a. Koji god bude dosegnut prvi zaustavlja replay.

### Vidi također

- [Budget Alerts](#budget-alerts) za email notifikacije.
- [Cost Model](#cost-model) za način na koji platforma konvertuje tokene u dolare.
- [Drop Reasons](#drop-reasons) za kompletan spisak razloga zbog kojih trigger ne radi.