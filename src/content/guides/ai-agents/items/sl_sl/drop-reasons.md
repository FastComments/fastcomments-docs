Ko se sprožilec (trigger) zažene za agenta, a **ne** povzroči klica LLM, platforma zabeleži "izpust" (drop) z razlogom. Izpusti so prikazani na [strani Analytics](#analytics-page) pod "Triggers skipped (this month)".

### Celoten seznam razlogov za izpust

| Razlog | Kaj se je zgodilo |
|---|---|
| `agentDaily` | Dosežen je dnevni proračunski strop agenta. |
| `agentMonthly` | Dosežen je mesečni proračunski strop agenta. |
| `tenantDaily` | Dosežen je dnevni proračunski strop najemnika (tenant). |
| `tenantMonthly` | Dosežen je mesečni proračunski strop najemnika (tenant). |
| `qps` | Dosežena je omejitev hitrosti agenta na minuto (drseče 60-sekundno okno). |
| `concurrency` | Največje število sočasnih izvedb agenta je že bilo zapolnjeno. |

### Kaj ni na tem seznamu

Sprožilec, ki nikoli ne doseže poti odpošiljanja (dispatch), ni "izpuščen" z razlogom - preprosto ni odpošlan. To vključuje:

- Agent je **Onemogočen** (Disabled).
- Komentar, ki je sprožil akcijo, ne ustreza [obsegu URL/jezikovne oznake](#scope-url-locale) agenta.
- Sprožilno akcijo je izvedel isti agent (preprečevanje zank).
- Najemnik ima neveljavno obračunavanje.
- Agent ni v paketu najemnika.

To so tihe preskočitve, ne izpusti. Ne pojavljajo se v grafu izpustov na Analytics.

### Branje izpustov na Analytics

[Stran Analytics](#analytics-page) prikazuje:

- **Triggers skipped (this month)** - števila, združena po razlogu izpusta.
- **Agents at or near their cap** - razčlenitev po agentih, kateri agenti omejujejo strop, s številom izpuščenih sprožilcev v trenutnem obdobju.

### Kaj storiti, ko opazite izpuste

- **`agentDaily` / `agentMonthly`** - lastni strop agenta je pretesen. Bodisi dvignite strop na obrazcu za urejanje bodisi zožite obseg agenta (URL/jezikovna oznaka, ožji sprožilci).
- **`tenantDaily` / `tenantMonthly`** - strop na ravni računa je pretesen. Dvignite ga v nastavitvah obračunavanja najemnika ali porazdelite porabo med manj agentov.
- **`qps`** - promet dosega omejitev na minuto z drsečim oknom. Pogosto znak viralne niti, ki sproža sprožilce hitreje, kot jih agent zmore izvesti. Polji agenta `maxTriggersPerMinute` in `maxConcurrent` to omejujeta; njuno povečanje poveča prepustnost, a tudi strošek sunkovitosti (burst).
- **`concurrency`** - ista korenita vzroka kot pri `qps`, a pri številu izvedb v teku. Povečajte `maxConcurrent`, če potrebujete več vzporednosti.

### Izpusti v primerjavi z napakami

Izpust pomeni "sprožilec se ni nikoli izvedel". **Napaka** pomeni "sprožilec se je izvedel, a klic LLM ali odpošiljanje orodja ni uspelo". Napake se beležijo ločeno na strani [Run History](#run-history) (status `Error`).

### Izpusti lahko ustavijo tudi ponovitve (replays)

Isti razlogi za izpust ustavijo [testne izvedbe / ponovitve](#test-runs-replays) v teku. Ponovitev se ustavi s statusom Error in sporočilom, ki poimenuje, kateri proračun je bil dosežen (na primer dnevni proračun agenta).

### Preprečevanje zank je namenoma tiho

Ni razloga za izpust za "ta sprožilec je prišel od drugega agenta in je bil preskočen, da se prepreči zanka". Beleženje tega bi zamašilo analitiko brez koristnega signala - po zasnovi razvejanje agentov ne bi nikoli smelo povzročiti eksplozije sprožilcev. Če sumite, da je zanka zatirana, čeprav ne bi smela biti, preverite [Comment Logs](/guide-moderation.html#comment-logs) - preverjanje zank se opira na `botId` pri komentarjih, ki jih je napisal bot.