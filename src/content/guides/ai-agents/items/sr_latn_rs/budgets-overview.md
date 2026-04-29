Svaki agent ima ograničenja potrošnje. Platforma prestaje da dispatch-uje agenta kada se dostigne ograničenje i nastavlja kada počne novi period.

### Dva opsega, dva perioda

Postoje ukupno četiri ograničenja - dva opsega (po agentu, po tenant-u) u kombinaciji sa dva perioda (dnevni, mesečni).

| Scope | Period | Where you set it |
|---|---|---|
| Per-agent daily | UTC day | Forma za uređivanje agenta -> **Budžet** -> **Dnevni budžet** |
| Per-agent monthly | calendar month | Forma za uređivanje agenta -> **Budžet** -> **Mesečni budžet** |
| Per-tenant daily | UTC day | Izvedeno iz plana (nema zasebnog unosa vidljivog korisniku) |
| Per-tenant monthly | calendar month | Izvedeno iz plana (nema zasebnog unosa vidljivog korisniku) |

Okidač se izvršava samo ako **sva četiri ograničenja** to dozvole. Prvo ograničenje koje se iscrpi je ono koje prekida okidač.

### Valuta

Budžeti po agentu se unose u valuti vašeg naloga.

### Šta se dešava kada se dostigne ograničenje

- Okidač se beleži kao **odbačen** sa [razlogom odbacivanja](#drop-reasons) kao što su `agentDaily` ili `tenantMonthly`.
- Broj odbacenih pojavljuje se na [Analytics page](#analytics-page) pod "Triggers skipped (this month)".
- Ne pravi se LLM poziv; nijedan token se ne troši na sam odbaceni okidač.
- Status agenta ostaje nepromenjen - jednostavno ne može da dispatch-uje dok ne počne novi period.

### Reset perioda

- **Dnevna** ograničenja se resetuju u ponoć po UTC.
- **Mesečna** ograničenja se resetuju početkom svakog kalendarskog meseca, po UTC.

Ne postoji prenos neiskorišćenog budžeta u naredni period.

### Stroga ograničenja naspram blagih upozorenja

Ograničenja su **stroga**. Ne postoji režim "prekorači za 10% uz upozorenje". Kada se ograničenje dostigne, dispatch se zaustavlja.

"Blagi" deo su [Budget Alerts](#budget-alerts) e-poruke - dobijate mejl na podesivim pragovima (podrazumevano 80% i 100%) tako da možete povisiti ograničenje pre nego što saobraćaj počne da opada.

### Gde pratiti trenutnu potrošnju

- [Analytics page](#analytics-page) - potrošnja budžeta po agentu i za ceo tenant sa markerima ograničenja.
- Sekcija **Stats** u formi za uređivanje agenta.
- Lista (broj zahteva za odobrenje na čekanju i nedavni izvršeni zapisi je na kartici agenta).

### Odabir budžeta

Nekoliko praktičnih pravila:

- **Nov agent** - odredite budžet. Pratite [Run History](#run-history) nedelju dana. Prilagodite na osnovu uočenog troška po izvršavanju × očekivanog obima okidača.
- **Agent sa velikim obimom** (npr. okidač za novi komentar na prometnom sajtu) - dnevno ograničenje je ono što zaustavlja beskonačnu petlju. Izaberite dnevno ograničenje koje je 2–3× vaše očekivane dnevne potrošnje tako da se normalan prometni dan uklopi bez problema.
- **Agent za sumiranje ili koji intenzivno koristi kontekst** - trošak po izvršavanju je visok. Postavite strože dnevno ograničenje da sprečite da loš dan obori mesečni budžet.

### Zaobilaženje budžeta za ponovna izvršavanja

[Test runs / replays](#test-runs-replays) podležu svom sopstvenom strogom ograničenju (podešenom na formi za replay, odvojenom od dnevnih/mesečnih ograničenja agenta), I ograničenjima agenta i tenant-a. Koji god bude prvi dostignut, zaustavlja replay.

### Pogledajte i

- [Budget Alerts](#budget-alerts) za e-mail obaveštenja.
- [Cost Model](#cost-model) za način na koji platforma konvertuje tokene u dolare.
- [Drop Reasons](#drop-reasons) za kompletan spisak razloga zbog kojih okidač nije pokrenut.