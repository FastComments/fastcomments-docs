Kada se okidač aktivira za agenta, ali **ne** rezultira pozivom LLM-a, platforma evidentira "drop" s razlogom. Dropovi se pojavljuju na [Analytics page](#analytics-page) pod "Triggers skipped (this month)".

### The full list of drop reasons

| Reason | What happened |
|---|---|
| `agentDaily` | The agent's daily budget cap was hit. |
| `agentMonthly` | The agent's monthly budget cap was hit. |
| `tenantDaily` | The tenant's daily budget cap was hit. |
| `tenantMonthly` | The tenant's monthly budget cap was hit. |
| `qps` | The agent's per-minute rate limit (rolling 60s window) was hit. |
| `concurrency` | The agent's max concurrent runs was already saturated. |

### What's not in this list

Okidač koji nikada ne dođe do puta dispatcha nije "dropan" s razlogom - jednostavno nije proslijeđen. To uključuje:

- Agent je **Disabled**.
- Komentar koji pokreće ne odgovara agentovom [URL/locale scope](#scope-url-locale).
- Radnja koja pokreće je izvedena od istog agenta (sprječavanje petlje).
- Najmoprimac (tenant) ima neispravno naplaćivanje.
- Agent nije uključen u plan najmoprimca.

Ovo su tihi preskoci, a ne dropovi. Ne pojavljuju se u grafikonu dropova na Analyticsu.

### Reading drops on Analytics

The [Analytics page](#analytics-page) shows:

- **Triggers skipped (this month)** - counts grouped by drop reason.
- **Agents at or near their cap** - per-agent breakdown of which agents are pushing the cap, with a count of dropped triggers in the current period.

### What to do when you see drops

- **`agentDaily` / `agentMonthly`** - agentov vlastiti limit je previsoko postavljen. Povećajte limit na obrascu za uređivanje ili suzite opseg agenta (URL/locale, uži okidači).
- **`tenantDaily` / `tenantMonthly`** - limit na razini računa je previsok. Povećajte ga u postavkama naplate najmoprimca ili rasporedite potrošnju na manje agenata.
- **`qps`** - promet udarava u ograničenje po minuti u pomičnom prozoru. Često znak da viralna nit širi okidače brže nego što ih agent može obraditi. Agentova polja `maxTriggersPerMinute` i `maxConcurrent` ograničavaju ovo; njihovo povećanje povećava propusnost ali i trošak pikova.
- **`concurrency`** - isti osnovni uzrok kao i `qps`, ali na broju istodobnih zadataka. Povećajte `maxConcurrent` ako trebate više paralelizma.

### Drops vs errors

Drop znači "okidač nikada nije pokrenut". **Greška** znači "okidač je pokrenut, ali je poziv prema LLM-u ili slanje alata zakazalo". Greške se prate odvojeno na stranici [Run History](#run-history) (status `Error`).

### Drops can also stop replays

Isti razlozi za drop zaustavljaju i tekuće [test runs / replays](#test-runs-replays). Replay se zaustavlja s Error statusom i porukom koja navodi koji je budžet dosegnut (na primjer, agentov dnevni budžet).

### Loop prevention is silent on purpose

Ne postoji razlog za drop "ovaj okidač je došao od drugog agenta i preskočen je radi sprječavanja petlje". Njegovo bilježenje bi zagušilo analitiku bez korisnog signala - prema dizajnu, fan-out agenata ne bi trebao rezultirati eksplozijom okidača. Ako sumnjate da se petlja suprimira tamo gdje ne bi trebala biti, provjerite [Dnevnici komentara](/guide-moderation.html#comment-logs) - `botId` na komentarima koje je kreirao bot je ono po čemu se provodi provjera petlje.