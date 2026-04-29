Standardmäßig läuft ein Agent über Ihren gesamten Tenant — jede Seite, jede Locale. Die Abschnitte **Scope** und **Locales** im Bearbeitungsformular ermöglichen es, das einzuschränken.

### Auf bestimmte Seiten beschränken

Das Feld **Restrict to specific pages** akzeptiert ein URL-Muster pro Zeile, in url-pattern glob syntax. Der Agent läuft nur bei Kommentaren, deren Seiten-URL mindestens einem der Muster entspricht. Beispiele:

- `/news/*` - jede Seite unter `/news`.
- `/forums/*` - jede Seite unter `/forums`.
- `/blog/2026/*` - jede Seite unter `/blog/2026`.
- (mehrere Zeilen zusammen) - der Agent läuft, wenn **ein** Muster passt.

Maximum: 50 Muster pro Agent. Muster müssen gültige url-pattern globs sein - das Formular lehnt fehlerhafte mit einer spezifischen Fehlermeldung ab.

Wenn das Feld **blank** ist, läuft der Agent auf jeder Seite im Tenant.

Wenn das Feld **non-blank** ist, schlägt der Agent geschlossen fehl: Jeder Trigger, dessen Kommentar keine `urlId` hat (z. B. tenant-level events ohne Seitenkontext), wird übersprungen. Das ist beabsichtigt - "scoped to /news/*" sollte nicht stillschweigend zu "everything" werden.

### Auf bestimmte Locales beschränken

Der Dual-List-Picker **Restrict to specific locales** nimmt FastComments-Locale-IDs an (`en_us`, `zh_cn`, `de_de` usw.). Der Agent läuft nur bei Kommentaren, deren erkannte Locale in der ausgewählten Liste enthalten ist.

Die erkannte Locale stammt aus dem `locale`-Feld des Kommentars, das vom Kommentar-Widget beim Absenden anhand der Seiten-Locale gesetzt wird.

Wenn **keine Locales** ausgewählt sind, läuft der Agent für jede Locale.

Wenn **eine oder mehrere Locales** ausgewählt sind, schlägt der Agent geschlossen fehl: Trigger ohne Kommentar oder Trigger bei Kommentaren ohne `locale`-Feld werden übersprungen.

### Kombinierte Einschränkung

URL- und Locale-Filter werden mit UND verknüpft. Ein Trigger löst den Agenten nur aus, wenn **beide** Filter dies zulassen.

Nützliche Muster:
- `/news/*` URL pattern + `en_us` locale - nur der englische News-Bereich.
- Kein URL-Filter + mehrere Locales - tenantweit, aber nur für die Sprachen, für die das Prompt dieses Agenten verfasst wurde.

### Warum einen Agenten einschränken

- **Kosten.** Einschränkung reduziert die Anzahl der Trigger, die der Agent verarbeiten muss, und damit die Token-Ausgaben.
- **Korrektheit.** Ein auf Fachartikel abgestimmtes Summarisierungs-Prompt kann auf Produktseiten schlechte Ergebnisse liefern. Einschränkung ist ein schärferes Werkzeug als das Auffordern des Prompts, nicht-technische Seiten auf Englisch zu überspringen.
- **Locale-spezifisches Verhalten.** Ein Begrüßer, der nur auf Deutsch schreibt, sollte nur bei Kommentaren mit deutscher Locale laufen. Kombinieren Sie den `de_de`-Locale-Bereich mit einem deutschsprachigen Ton im [initial prompt](#personality-prompt).

### Was die Einschränkung *nicht* bewirkt

- Es ändert nicht die **agent slot count** (siehe [Plans and Eligibility](#plans-and-eligibility)) - ein eingegrenzter Agent belegt weiterhin einen Slot.
- Es ändert nicht die [Budget caps](#budgets-overview) - die täglichen und monatlichen Limits pro Agent gelten für alle übereinstimmenden Trigger.
- Es wirkt sich nicht rückwirkend auf vergangene Ausführungen aus - die Ausführungshistorie zeigt alles, was der Agent getan hat, selbst wenn Sie ihn später enger einschränken.