Kada se okidač aktivira za agenta ali **ne** dovede do poziva LLM-a, platforma zabeleži "preskok" sa razlogom. Preskoci se pojavljuju na [Analytics page](#analytics-page) pod "Preskočeni okidači (ovog meseca)".

### Kompletna lista razloga za preskoke

| Reason | What happened |
|---|---|
| `agentDaily` | Dosegnut je dnevni limit budžeta agenta. |
| `agentMonthly` | Dosegnut je mesečni limit budžeta agenta. |
| `tenantDaily` | Dosegnut je dnevni limit budžeta naloga (tenant). |
| `tenantMonthly` | Dosegnut je mesečni limit budžeta naloga (tenant). |
| `qps` | Dosegnut je ograničenje stope po agentu po minuti (tureći prozor od 60s). |
| `concurrency` | Maksimalan broj paralelnih pokretanja agenta je već bio zasićen. |

### Šta nije na ovoj listi

Okidač koji nikada ne dođe do puta dispečovanja nije "preskočen" sa razlogom - on jednostavno nije dispečovan. To uključuje:

- Agent je **Onemogućen**.
- Komentar koji je izazvao okidanje ne odgovara agentovom [URL/locale scope](#scope-url-locale).
- Radnja koja je pokrenula okidač je izvršena od strane istog agenta (sprečavanje petlje).
- Tenant ima neispravno naplaćivanje.
- Agent nije u planu tenanta.

Ovo su tihi preskoci, ne preskoci sa razlogom. Ne pojavljuju se na grafikonu preskoka u Analitici.

### Čitanje preskoka u Analitici

[Analytics page](#analytics-page) prikazuje:

- **Preskočeni okidači (ovog meseca)** - brojači grupisani po razlozima preskoka.
- **Agentii koji su na ili blizu svog limita** - razbijanje po agentima koji pritiskaju limit, sa brojem preskočenih okidača u tekućem periodu.

### Šta uraditi kada vidite preskoke

- **`agentDaily` / `agentMonthly`** - agentov sopstveni limit je previsok. Povećajte limit na formi za uređivanje ili suzite opseg agenta (URL/locale, uži okidači).
- **`tenantDaily` / `tenantMonthly`** - limit na nivou naloga je previsok. Povećajte ga u podešavanjima naplate tenanta, ili raspodelite troškove na manje agenata.
- **`qps`** - saobraćaj dostiže limit po minuti u rolling-prozoru. Često znak da viralna nit širi okidače brže nego što agent može da ih izvrši. Polja agenta `maxTriggersPerMinute` i `maxConcurrent` ograničavaju ovo; njihovo povećanje povećava propusnost ali i troškove za pikove.
- **`concurrency`** - isti osnovni uzrok kao i `qps`, ali kod broja istovremeno u toku. Povećajte `maxConcurrent` ako vam treba više paralelizma.

### Preskoci naspram grešaka

Preskok znači "okidač nikada nije pokrenut". **Greška** znači "okidač je pokrenut ali je poziv LLM-a ili dispečovanje alata propalo". Greške se prate odvojeno na stranici [Run History](#run-history) (status `Error`).

### Preskoci takođe mogu zaustaviti reprodukcije

Isti razlozi preskoka zaustavljaju i tekuće [test runs / replays](#test-runs-replays). Reprodukcija se zaustavlja sa statusom Error i porukom koja navodi koji je budžet dosegnut (na primer, dnevni budžet agenta).

### Sprečavanje petlji je namerno tihi događaj

Ne postoji razlog preskoka za "ovaj okidač je došao od drugog agenta i preskočen je da bi se sprečila petlja". Beleženje toga bi zagušilo analitiku bez korisnog signala - po dizajnu, fan-out agenata nikada ne bi trebalo da rezultira eksplozijom okidača. Ako sumnjate da se petlja potiskuje tamo gde ne bi trebalo, proverite [Comment Logs](/guide-moderation.html#comment-logs) - `botId` na komentarima koje je napisao bot je ono na šta se proverava petlja.