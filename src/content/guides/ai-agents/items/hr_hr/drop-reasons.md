Kada se okidač aktivira za agenta, ali **ne** rezultira LLM pozivom, platforma bilježi "ispad" (drop) s razlogom. Ispadi se pojavljuju na [Analytics stranici](#analytics-page) pod "Triggers skipped (this month)".

### Potpuni popis razloga ispadanja

| Razlog | Što se dogodilo |
|---|---|
| `agentDaily` | Dostignut je dnevni limit proračuna agenta. |
| `agentMonthly` | Dostignut je mjesečni limit proračuna agenta. |
| `tenantDaily` | Dostignut je dnevni limit proračuna tenanta. |
| `tenantMonthly` | Dostignut je mjesečni limit proračuna tenanta. |
| `qps` | Dostignuto je ograničenje brzine agenta po minuti (klizni prozor od 60 s). |
| `concurrency` | Maksimalan broj istovremenih izvođenja agenta je već zasićen. |

### Što nije na ovom popisu

Okidač koji nikada ne dođe do faze otpreme nije "ispao" s razlogom - jednostavno se ne otprema. To uključuje:

- Agent je **Disabled**.
- Okidački komentar ne odgovara [URL/lokalnom opsegu](#scope-url-locale) agenta.
- Okidačku radnju izvršio je isti agent (prevencija petlje).
- Tenant ima nevažeću naplatu.
- Agent nije u planu tenanta.

To su tiha preskakanja, ne ispadi. Ne pojavljuju se u grafikonu ispadanja na Analyticsu.

### Čitanje ispadanja na Analyticsu

[Analytics stranica](#analytics-page) prikazuje:

- **Triggers skipped (this month)** - brojanja grupirana po razlogu ispadanja.
- **Agents at or near their cap** - analiza po agentu koji agenti pritišću limit, s brojem ispaljenih okidača u trenutnom razdoblju.

### Što učiniti kada vidite ispade

- **`agentDaily` / `agentMonthly`** - vlastiti limit agenta je preusan. Ili podignite limit na obrascu za uređivanje ili suzite opseg agenta (URL/lokalni, uži okidači).
- **`tenantDaily` / `tenantMonthly`** - limit na razini računa je preusan. Podignite ga u postavkama naplate tenanta ili raspodijelite potrošnju na manje agenata.
- **`qps`** - promet pogađa ograničenje po minuti s kliznim prozorom. Često znak viralne rasprave koja generira okidače brže nego što ih agent može izvršiti. Polja agenta `maxTriggersPerMinute` i `maxConcurrent` ograničavaju ovo; njihovo povećanje povećava propusnost, ali i trošak naglog povećanja.
- **`concurrency`** - isti uzrok kao `qps`, ali na razini broja aktivnih izvođenja. Podignite `maxConcurrent` ako trebate više paralelizma.

### Ispadi vs pogreške

Ispad znači "okidač se nikada nije izvršio". **Pogreška** znači "okidač se izvršio, ali LLM poziv ili otprema alata nije uspjela". Pogreške se pratiti odvojeno na stranici [Run History](#run-history) (status `Error`).

### Ispadi također mogu zaustaviti ponovna izvođenja

Isti razlozi ispadanja zaustavljaju [test runs / replays](#test-runs-replays) u tijeku. Ponovno izvođenje se zaustavlja sa statusom Error i porukom koja navodi koji je proračun pogođen (na primjer, dnevni proračun agenta).

### Prevencija petlje je namjerno tiha

Ne postoji razlog ispadanja za "ovaj okidač je došao od drugog agenta i preskočen je kako bi se spriječila petlja". Bilježenje toga bi zatrpalo analitiku bez korisnog signala - po dizajnu, širenje agenata nikada ne bi trebalo rezultirati eksplozijom okidača. Ako sumnjate da se petlja potiskuje tamo gdje ne bi trebala biti, provjerite [Comment Logs](/guide-moderation.html#comment-logs) - `botId` na komentarima koje je autorizirao bot je ono na čemu se temelji provjera petlje.