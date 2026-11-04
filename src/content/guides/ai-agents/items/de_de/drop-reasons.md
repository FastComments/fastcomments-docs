Wenn ein Trigger für einen Agenten auslöst, aber **keinen** LLM-Aufruf zur Folge hat, zeichnet die Plattform einen „Drop" mit einem Grund auf. Drops erscheinen auf der [Analytics-Seite](#analytics-page) unter „Triggers skipped (this month)".

### Die vollständige Liste der Drop-Gründe

| Grund | Was passiert ist |
|---|---|
| `agentDaily` | Das tägliche Budgetlimit des Agenten wurde erreicht. |
| `agentMonthly` | Das monatliche Budgetlimit des Agenten wurde erreicht. |
| `tenantDaily` | Das tägliche Budgetlimit des Tenants wurde erreicht. |
| `tenantMonthly` | Das monatliche Budgetlimit des Tenants wurde erreicht. |
| `qps` | Das Ratenlimit des Agenten pro Minute (rollierendes 60-Sekunden-Fenster) wurde erreicht. |
| `concurrency` | Die maximal gleichzeitigen Ausführungen des Agenten waren bereits ausgelastet. |

### Was nicht in dieser Liste steht

Ein Trigger, der den Dispatch-Pfad nie erreicht, wird nicht als „Drop" mit einem Grund gewertet – er wird einfach nicht ausgelöst. Das umfasst:

- Der Agent ist **Disabled**.
- Der auslösende Kommentar entspricht nicht dem [URL/locale scope](#scope-url-locale) des Agenten.
- Die auslösende Aktion wurde vom selben Agenten durchgeführt (Schleifenvermeidung).
- Der Tenant hat eine ungültige Abrechnung.
- Der Agent ist nicht im Plan des Tenants enthalten.

Dies sind stille Übersprünge, keine Drops. Sie erscheinen nicht im Drops-Diagramm auf Analytics.

### Drops auf Analytics lesen

Die [Analytics-Seite](#analytics-page) zeigt:

- **Triggers skipped (this month)** – Zählungen gruppiert nach Drop-Grund.
- **Agents at or near their cap** – Aufschlüsselung pro Agent, welche Agenten das Limit belasten, mit einer Anzahl verworfener Trigger im aktuellen Zeitraum.

### Was tun, wenn Sie Drops sehen?

- **`agentDaily` / `agentMonthly`** – das eigene Limit des Agenten ist zu knapp. Erhöhen Sie entweder das Limit im Bearbeitungsformular oder grenzen Sie den Agenten ein (URL/locale, engere Trigger).
- **`tenantDaily` / `tenantMonthly`** – das Limit auf Kontoebene ist zu knapp. Erhöhen Sie es in den Tenant-Abrechnungseinstellungen oder verteilen Sie die Ausgaben auf weniger Agenten.
- **`qps`** – der Traffic stößt an das rollierende Minutenlimit. Oft ein Zeichen dafür, dass ein viraler Thread Trigger schneller erzeugt, als der Agent sie ausführen kann. Die Felder `maxTriggersPerMinute` und `maxConcurrent` des Agenten begrenzen dies; eine Erhöhung steigert den Durchsatz, aber auch die Burst-Kosten.
- **`concurrency`** – gleiche Ursache wie `qps`, aber bei der Anzahl der laufenden Ausführungen. Erhöhen Sie `maxConcurrent`, wenn Sie mehr Parallelität benötigen.

### Drops vs. Fehler

Ein Drop bedeutet „der Trigger lief nie". Ein **Fehler** bedeutet „der Trigger lief, aber der LLM-Aufruf oder das Tool-Dispatch schlug fehl". Fehler werden separat auf der [Run History](#run-history)-Seite erfasst (Status `Error`).

### Drops können auch Replays stoppen

Dieselben Drop-Gründe stoppen laufende [Testläufe / Replays](#test-runs-replays). Das Replay stoppt mit dem Status Error und einer Meldung, die angibt, welches Budget erreicht wurde (z. B. das tägliche Budget des Agenten).

### Schleifenvermeidung ist absichtlich still

Es gibt keinen Drop-Grund für „dieser Trigger kam von einem anderen Agenten und wurde zur Vermeidung einer Schleife übersprungen". Das Protokollieren würde die Analytics ohne nützliches Signal überladen – by Design sollte das Fan-out von Agenten niemals zu einer Trigger-Explosion führen. Wenn Sie vermuten, dass eine Schleife unterdrückt wird, obwohl sie es nicht sollte, prüfen Sie die [Comment Logs](/guide-moderation.html#comment-logs) – die `botId` auf von Bots verfassten Kommentaren ist der Schlüssel für die Schleifenprüfung.