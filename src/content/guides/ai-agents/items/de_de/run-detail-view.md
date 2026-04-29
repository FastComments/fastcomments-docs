---
Wenn Sie in einer Zeile des [Ausführungsverlaufs](#run-history) auf **View** klicken, öffnet sich die Detailseite für den jeweiligen Durchlauf. Hier können Sie die Begründung des Agents lesen und seine Entscheidungen bewerten.

### Oben: Zusammenfassung des Durchlaufs

- **Agent** - welcher Agent ausgeführt wurde.
- **When** - Zeitstempel.
- **Status** - Gestartet / Erfolgreich / Fehler, plus das Abzeichen **Dry Run**, falls zutreffend.
- **Cost** - Kosten pro Durchlauf in der Währung Ihres Mandanten.
- **Cost per action** - Kosten geteilt durch die Anzahl der nicht ausstehenden Aktionen, nützlich, um ungewöhnlich teure Durchläufe zu erkennen.

### Durchgeführte Aktionen

Eine Liste, in Reihenfolge, aller Tool-Aufrufe, die der Durchlauf durchgeführt hat. Jeder Eintrag zeigt:

- **Action label** - "Kommentar geschrieben", "Kommentar als Spam markiert", "Benutzer gesperrt" und so weiter. Das Label wird aus dem Aktions-Typ-Enum abgeleitet.
- **Reference ID** - die betroffene Kommentar-, Benutzer- oder Badge-ID, als Monospace-Text dargestellt (kein Hyperlink).
- **Agent reasoning** - die Begründung, die der Agent mit dem Aufruf geliefert hat.
- **Confidence** - die vom Agenten selbst eingeschätzte Konfidenz, angezeigt als Prozentsatz.
- **Pending approval** badge - falls die Aktion in den [Genehmigungs-Posteingang](#approval-workflow) eingereiht ist, statt ausgeführt zu werden.

Wenn der Durchlauf keine Aktionen ausgeführt hat, lautet der Abschnitt: "During this run, no actions were taken."

### LLM-Transkript

Unterhalb der Aktionen das vollständige Transkript der Unterhaltung des Agents mit dem LLM:

- **System** - der System-Prompt (Plattform-Suffix + Ihr Initial-Prompt + Community-Richtlinien).
- **User** - die Kontextnachricht, die den Auslöser beschreibt.
- **Assistant** - die Antworten des Modells, einschließlich Tool-Aufrufen.
- **Tool** - das Tool-Ergebnis, das an das Modell zurückgespeist wurde (z. B. was `search_memory` zurückgegeben hat).

Lange Nachrichten sind einklappbar; klicken Sie auf **Expand** / **Collapse**, um sie anzuzeigen.

### Transkripte lesen

Das Transkript ist die wichtigste Seite zum Feinabstimmen. Wenn der Agent eine Entscheidung trifft, der Sie nicht zustimmen, lesen Sie es zurück, um zu sehen:

- Was das Modell **sah** (die Kontextnachricht des Users).
- Was das Modell **entschied** (die Assistant-Tool-Aufrufe).
- Was das Modell **in Betracht zog** (alle Tool-Ergebnisse - z. B. hat der Agent tatsächlich `search_memory` aufgerufen und hat es etwas gefunden, bevor es eine Sperre verhängte).

Wenn das Modell konsequent denselben Fehler macht, bearbeiten Sie den [Initial-Prompt](#personality-prompt) - oder verwenden Sie [Prompts verfeinern](#refining-prompts) aus einer abgelehnten Genehmigung.

### Aktionsreferenzen

Die Referenz-IDs werden als Monospace-Text angezeigt (keine Hyperlinks):

- Kommentare: die Kommentar-ID.
- Benutzer: die Benutzer-ID.
- Abzeichen: die Abzeichen-ID.

Sie können die ID kopieren, um den betroffenen Datensatz auf der entsprechenden Moderations-/Admin-Seite nachzuschlagen.

### Was beim Dry-Run fehlt

Dry-Run-Durchläufe zeigen die **gleichen** Aktionen, Begründungen und Konfidenzwerte. Der einzige Unterschied ist das Abzeichen **Dry Run** in der Statuszeile. Die Referenz-IDs für Kommentare / Benutzer / Abzeichen werden weiterhin angezeigt – der Agent hat sie nur nicht beeinflusst.

### Fehler

Bei Durchläufen im `Error`-Status zeigt die Detailseite die zugrunde liegende Fehlermeldung. Häufige Fehler:

- **Kein LLM-API-Schlüssel konfiguriert** - Fehlkonfiguration des Tenants oder der Plattform.
- **LLM-Aufruf-Timeout** - der LLM-Anbieter war langsam oder nicht verfügbar.
- **Tool-Dispatch-Fehler** - der Agent wählte ein Tool mit fehlerhaften Argumenten (z. B. eine Kommentar-ID, die nicht mehr existiert).
- **Budget während des Durchlaufs erschöpft** - das Limit des Agents wurde getroffen, während der Durchlauf lief. Der Durchlauf wurde gestoppt.

Fehler rollen teilweise ausgeführte Aktionen nicht zurück – alle Tool-Aufrufe, die vor dem Fehler abgeschlossen wurden, bleiben bestehen.
---