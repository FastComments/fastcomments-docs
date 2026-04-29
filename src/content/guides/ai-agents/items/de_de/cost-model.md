Agentenkosten sind **tokenbasiert**. Jeder LLM-Aufruf gibt eine Token-Anzahl zurück, die Plattform konvertiert diese mit dem modellabhängigen Preis pro Token in USD-Cents, und die Cents werden gegen die Budgets des Agents und des Tenants abgerechnet.

### Was berechnet wird

- **Alle LLM-Aufrufe**, einschließlich des Aufrufs, der null Tool-Aktionen erzeugt ("der Agent hat beschlossen, nichts zu tun"). Die Inferenz wird auch dann bezahlt, wenn keine Aktion resultiert.
- **Dry-Run-Aufrufe**. Dry-Run bedeutet "nicht handeln, aber trotzdem das LLM aufrufen" - der LLM-Aufruf kostet gleich viel. Siehe [Dry-Run-Modus](#dry-run-mode).
- **Replay-Aufrufe**. Replays sind Dry-Run-Durchläufe gegen historische Kommentare. Sie kosten Tokens. Siehe [Testläufe (Replays)](#test-runs-replays).

### Was nicht berechnet wird

- **Trigger, die niemals einen LLM-Aufruf erzeugen.** Fälle, die vor dem LLM verworfen werden (über Budget, durch Ratenbegrenzung, Scope-Mismatch, ungültige Abrechnung, Schleifenvermeidung) kosten null Tokens. Siehe [Abbruchgründe](#drop-reasons).
- **Tool-Dispatch.** Das Aufrufen von `pin_comment` oder eines anderen Tools verursacht an sich keine Token-Kosten – nur der LLM-Round-Trip.
- **`search_memory`.** Es ist schreibgeschützt und erzeugt keinen eigenen LLM-Round-Trip.

### Kosten pro Lauf

Ein einzelner Agent-Lauf kann das LLM mehrfach aufrufen – jedes Tool-Call-Ergebnis wird wieder in das Modell eingespeist, damit es entweder ein weiteres Tool aufruft oder beendet. Daher ist `tokensUsed` bei einem Lauf die Summe aller LLM-Round-Trips in diesem Lauf.

Die größten Kostenquellen pro Lauf:

- **Lange [Initial-Prompts](#personality-prompt) und [Community-Richtlinien](#community-guidelines)** – sie gehen in jeden Lauf hinein.
- **[Kontextoptionen](#context-options)** – Thread-Kontext, Benutzerhistorie, Seitenmetadaten. Jede Komponente fügt Tokens hinzu.
- **Der Kommentartext selbst** – lange Kommentare kosten mehr.
- **Mehrere Tool-Aufrufe in einem Lauf** – das Ergebnis jeder Tool-Nachricht wird zurück an das Modell gesendet.
- **Memory-Lesevorgänge** – `search_memory` gibt bis zu 25 Datensätze zurück (begrenzt auf insgesamt 8000 Zeichen Inhalt). Die meisten dieser Bytes fließen in das nächste Prompt ein.

**Max Tokens Per Trigger** (default 20,000) begrenzt die **Antwort**größe pro LLM-Aufruf. Sie begrenzt nicht die Eingabegröße.

### Token-zu-Cents-Konvertierung

Die Plattform wendet einen einzigen paketbezogenen Satz pro Tenant an (`flexLLMCostCents` pro `flexLLMUnit` Tokens). Die Kosten pro Token sind paketweit, nicht modellabhängig – beide verfügbaren Modelle ([GLM 5.1 and GPT-OSS Turbo](#choosing-a-model)) werden auf einem gegebenen Paket zum gleichen Satz berechnet. Die [Run-Detail-Ansicht](#run-detail-view) zeigt die Kosten pro Lauf in Ihrer Währung an, sobald ein Lauf abgeschlossen ist.

### Wo die Kosten erfasst werden

Jeder Lauf protokolliert seine rohe Token-Anzahl und die Kosten pro Lauf. Tages- und Monatsgesamtwerte werden in der [Analytics-Seite](#analytics-page) zusammengefasst.

### Wie man die Kosten liest

- **Kosten pro Lauf**: [Run-Detail-Ansicht](#run-detail-view) -> `Cost` field.
- **Tägliche / monatliche Gesamtheit**: [Analytics-Seite](#analytics-page) -> Budgetnutzung und Diagramme der täglichen Kosten.
- **Kosten pro Aktion**: ebenfalls in der Run-Detail-Ansicht, nützlich zur Optimierung, wenn die Tool-Schleife eines Agents ungewöhnlich lange ist.

### Siehe auch

- [Modellauswahl](#choosing-a-model) - der größere Hebel bei den Kosten.
- [Kontextoptionen](#context-options) - woher zusätzliche Kosten stammen.
- [Budget-Übersicht](#budgets-overview) - harte Obergrenzen, die unkontrollierte Kosten verhindern.