Die Laufhistorie ist das pro Agent geführte Protokoll jedes ausgelösten Triggers. Erreichbar von der Agentenliste über die **Runs**-Schaltfläche oder direkt unter `/auth/my-account/ai-agents/{agentId}/runs`.

### Was auf der Seite zu finden ist

Eine paginierte Tabelle mit einer Zeile pro Lauf:

| Spalte | Bedeutung |
|---|---|
| Datum | Wann der Trigger ausgelöst wurde (oder wann der verzögerte Trigger lief). |
| Status | **Gestartet**, **Erfolgreich** oder **Fehler**. Ein **Dry Run**-Badge wird daneben angezeigt, wenn der Lauf im Dry-Run-Modus war. |
| Kosten | Kosten pro Lauf in der Währung Ihres Tenants. Für laufende (Gestartet) Läufe leer. |
| Aktionen | Die Anzahl der Tool-Aufrufe im Lauf. |
| Details | Eine **Anzeigen**-Schaltfläche, die die [Detaillierte Laufansicht](#run-detail-view) öffnet. |

### Bedeutung der Status

- **Gestartet** – der Lauf ist in Bearbeitung oder wurde beendet, bevor er abgeschlossen war. Ein Lauf, der ungewöhnlich lange im Status „Gestartet“ verharrt, deutet meist auf ein Timeout eines LLM-Aufrufs hin.
- **Fehler** – der Lauf wurde beendet, schlug jedoch irgendwo fehl – z. B. gab ein LLM-Aufruf einen Fehler zurück, eine Tool-Weiterleitung scheiterte usw. Die Detailansicht enthält den spezifischen Fehler.
- **Erfolgreich** – der Lauf wurde ohne Fehler abgeschlossen. Der Agent kann null, eine oder mehrere Aktionen ausgeführt haben.

### Zustand bei leerer Historie

Wenn ein Agent keine Läufe hat, zeigt die Seite: "Noch keine Läufe für diesen Agenten. Aktivierte Läufe erscheinen hier, sobald ein Trigger ausgelöst wird; verwenden Sie Testlauf, um vorzuschauen, was dieser Agent mit vergangenen Kommentaren tun würde."

Der letzte Hinweis ist beabsichtigt – der [Testlauf-Workflow](#test-runs-replays) ist der empfohlene Weg, um die Laufhistorie bei einem frischen Agenten zu befüllen.

### Was nicht auf der Laufhistorie-Seite zu finden ist

- **Live-Trigger, die nie ausgelöst wurden** – ein Trigger, der aufgrund von Budget-, Scope- oder Ratenbegrenzungen verworfen wurde, erscheint nicht auf dieser Seite. Diese werden auf der [Analyse-Seite](#analytics-page) unter „Triggers skipped“ angezeigt.
- **Genehmigungen** – ausstehende Genehmigungen für Aktionen, die in diesem Lauf vorgenommen wurden, befinden sich im [Genehmigungs-Posteingang](#approval-workflow). Die Aktion erscheint in der Laufdetailansicht als **Ausstehende Genehmigung**.

### Aufbewahrung

Einzelne Laufaufzeichnungen werden 90 Tage aufbewahrt, danach ist der Lauf nicht mehr in der Historie vorhanden. Kosten- und Triggerzahlen werden weiterhin in langfristigen Analysezusammenfassungen aggregiert, sodass die [Analyse-Seite](#analytics-page) historische Gesamtdaten auch über dieses Zeitfenster hinaus anzeigt.

### Wiedergaben

Von Wiedergaben erzeugte Läufe sind standardmäßig in der Live-Läufe-Ansicht ausgeblendet. Die Seite [Testläufe (Wiedergaben)](#test-runs-replays) ist der Ort, an dem Sie diese sehen.

### Filterung über Agenten hinweg

Die Läufetabelle ist pro Agent. Es gibt keine agentenübergreifende Laufansicht – die [Analyse-Seite](#analytics-page) ist die agentenübergreifende Zusammenfassung. Wenn Sie Läufe über mehrere Agenten hinweg untersuchen müssen, sind die [Webhooks](#webhooks-overview) `trigger.succeeded` und `trigger.failed` Ereignisse diejenigen, die Sie an Ihr eigenes System weiterleiten würden.