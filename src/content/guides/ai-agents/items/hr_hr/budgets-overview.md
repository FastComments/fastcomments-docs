Svaki agent ima ograničenja potrošnje. Platforma prestaje slati zahtjeve agentu kada se ograničenje dosegne i nastavlja slati kada razdoblje istekne.

### Dva opsega, dva razdoblja

Ukupno postoje četiri ograničenja - dva opsega (po agentu, po najmoprimcu) u kombinaciji s dva razdoblja (dnevno, mjesečno).

| Scope | Period | Where you set it |
|---|---|---|
| Per-agent daily | UTC day | Obrazac za uređivanje agenta -> **Proračun** -> **Dnevni proračun** |
| Per-agent monthly | calendar month | Obrazac za uređivanje agenta -> **Proračun** -> **Mjesečni proračun** |
| Per-tenant daily | UTC day | Izvedeno iz plana (nema zasebnog korisničkog unosa) |
| Per-tenant monthly | calendar month | Izvedeno iz plana (nema zasebnog korisničkog unosa) |

Okidač se izvršava samo ako **sva četiri ograničenja** to dopuštaju. Prvo ograničenje koje se iscrpi ono je što odbacuje okidač.

### Valuta

Proračuni po agentu unose se u valuti vašeg računa.

### Što se događa kada se ograničenje dosegne

- Okidač se bilježi kao **odbačen** s [drop reason](#drop-reasons) poput `agentDaily` ili `tenantMonthly`.
- Broj odbačenih okidača pojavljuje se na [Analytics page](#analytics-page) pod "Triggers skipped (this month)".
- Ne pokreće se poziv LLM-a; nijedni tokeni se ne troše na sam odbačeni okidač.
- Status agenta ostaje nepromijenjen - samo ne može slati zahtjeve dok razdoblje ne istekne.

### Reset razdoblja

- **Dnevna** ograničenja se resetiraju u ponoć po UTC.
- **Mjesečna** ograničenja se resetiraju na početku svakog kalendarskog mjeseca, po UTC.

Nema prenosa neiskorištenog budžeta u sljedeće razdoblje.

### Tvrdo ograničenje vs meka upozorenja

Ograničenja su **tvrda**. Ne postoji način "prekorači za 10% uz upozorenje". Kad se ograničenje dosegne, slanje se zaustavlja.

"Meka" komponenta su e-mail obavijesti [Budget Alerts](#budget-alerts) - dobivate e-mail pri konfigurabilnim pragovima (zadano 80% i 100%) kako biste mogli povećati ograničenje prije nego promet počne padati.

### Gdje provjeriti trenutačnu potrošnju

- [Analytics page](#analytics-page) - potrošnja po agentu i na razini najmoprimca s oznakama ograničenja.
- Sekcija **Stats** u obrascu za uređivanje agenta.
- Pregled liste (broj čekajućih odobrenja i nedavni pokušaji vidljivi su na kartici agenta).

### Odabir proračuna

Nekoliko smjernica:

- **Novi agent** - odredite proračun. Promatrajte [Run History](#run-history) tjedan dana. Prilagodite na temelju promatranog troška po pokretanju × očekivanog volumena okidača.
- **Agent s velikim prometom** (npr. okidač za novi komentar na prometnom sajtu) - dnevno ograničenje je ono što hvata izvankontrolne petlje. Odaberite dnevno ograničenje koje je 2–3× veće od očekivane dnevne potrošnje tako da normalan prometni dan stane bez problema.
- **Agent za sažimanje ili s puno konteksta** - trošak po izvršenju je visok. Postavite strože dnevno ograničenje kako biste spriječili da loš dan uništi mjesečni budžet.

### Zaobilaženje proračuna za reprodukcije

[Testna pokretanja / reprodukcije](#test-runs-replays) podliježu svom **vlastitom** tvrdom ograničenju (postavljenom u obrascu za reprodukciju, odvojenom od dnevnih/mjesečnih ograničenja agenta), I ograničenjima agenta i najmoprimca. Ono koje se prvo dostigne zaustavlja reprodukciju.

### Vidi također

- [Budget Alerts](#budget-alerts) za e-mail obavijesti.
- [Cost Model](#cost-model) za način na koji platforma pretvara tokene u dolare.
- [Drop Reasons](#drop-reasons) za potpuni popis razloga zbog kojih okidač ne radi.