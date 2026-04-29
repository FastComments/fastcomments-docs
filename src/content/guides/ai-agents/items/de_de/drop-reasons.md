When a trigger fires for an agent but does **not** result in an LLM call, the platform records a "drop" with a reason. Drops appear in the [Analytics-Seite](#analytics-page) unter "Triggers skipped (this month)".

### The full list of drop reasons

| Reason | What happened |
|---|---|
| `agentDaily` | Das tägliche Budgetlimit des Agents wurde erreicht. |
| `agentMonthly` | Das monatliche Budgetlimit des Agents wurde erreicht. |
| `tenantDaily` | Das tägliche Budgetlimit des Mandanten wurde erreicht. |
| `tenantMonthly` | Das monatliche Budgetlimit des Mandanten wurde erreicht. |
| `qps` | Das per-Minute Rate-Limit des Agents (rollierendes 60‑Sekunden‑Fenster) wurde erreicht. |
| `concurrency` | Die maximale Anzahl gleichzeitiger Ausführungen des Agents war bereits ausgelastet. |

### What's not in this list

Ein Trigger, der nie den Dispatch-Pfad erreicht, wird nicht mit einem Grund "gedroppt" — er wird einfach nicht dispatched. Dazu gehört:

- Der Agent ist **Deaktiviert**.
- Der auslösende Kommentar stimmt nicht mit dem [URL-/Locale-Bereich](#scope-url-locale) des Agents überein.
- Die auslösende Aktion wurde vom selben Agenten vorgenommen (Schleifenverhinderung).
- Der Mandant hat ungültige Abrechnung.
- Der Agent ist nicht im Plan des Mandanten.

Dies sind stille Übersprünge, keine Drops. Sie erscheinen nicht im Drops-Diagramm auf der Analytics-Seite.

### Drops auf der Analytics-Seite lesen

Die [Analytics-Seite](#analytics-page) zeigt:

- **Triggers skipped (this month)** - Zählungen gruppiert nach Abbruchgrund.
- **Agents at or near their cap** - Aufschlüsselung pro Agent, welche Agents das Limit erreichen, mit einer Anzahl gedropter Trigger im aktuellen Zeitraum.

### Was zu tun ist, wenn Sie Abbrüche sehen

- **`agentDaily` / `agentMonthly`** - Das eigene Limit des Agents ist zu restriktiv. Erhöhen Sie das Limit im Bearbeitungsformular oder schränken Sie den Agenten ein (URL/Locale, engere Trigger).
- **`tenantDaily` / `tenantMonthly`** - Das kontoübergreifende Limit ist zu restriktiv. Erhöhen Sie es in den Abrechnungseinstellungen des Mandanten oder verteilen Sie die Ausgaben auf weniger Agents.
- **`qps`** - Der Traffic trifft das pro-Minute rollierende Fenster-Limit. Oft ein Zeichen dafür, dass ein viraler Thread Trigger schneller auslöst, als der Agent sie abarbeiten kann. Die Felder `maxTriggersPerMinute` und `maxConcurrent` des Agents begrenzen dies; eine Erhöhung steigert den Durchsatz, erhöht aber auch die Kosten bei Spitzenbelastung.
- **`concurrency`** - Gleiche Ursache wie bei `qps`, aber bezogen auf die Anzahl der laufenden Instanzen. Erhöhen Sie `maxConcurrent`, wenn Sie mehr Parallelität benötigen.

### Drops vs errors

Ein Drop bedeutet, "der Trigger lief nie". Ein **Fehler** bedeutet, "der Trigger lief, aber der LLM-Aufruf oder das Tool-Dispatch ist fehlgeschlagen". Fehler werden separat auf der [Run-History-Seite](#run-history) verfolgt (Status `Error`).

### Drops können auch Replays stoppen

Die gleichen Abbruchgründe stoppen laufende [Testläufe / Wiedergaben](#test-runs-replays). Die Wiedergabe endet mit einem Fehlerstatus und einer Nachricht, die angibt, welches Budget erreicht wurde (zum Beispiel das tägliche Budget des Agents).

### Schleifenverhinderung ist absichtlich still

Es gibt keinen Abbruchgrund für "dieser Trigger kam von einem anderen Agenten und wurde zur Verhinderung einer Schleife übersprungen". Das zu protokollieren würde die Analytics ohne nützlichen Hinweis verunreinigen — per Design sollte Fan-Out von Agents niemals zu einer Explosion von Triggern führen. Wenn Sie vermuten, dass eine Schleife unterdrückt wird, obwohl sie das nicht sollte, prüfen Sie die [Kommentarprotokolle](/guide-moderation.html#comment-logs) — die `botId` bei bot-verfassten Kommentaren ist der Wert, auf den die Schleifenprüfung prüft.