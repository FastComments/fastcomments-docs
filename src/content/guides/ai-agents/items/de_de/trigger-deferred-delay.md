Standardmäßig wird ein Agent **sofort** nach dem Auslösen seines Triggers ausgeführt. Das Feld **Verzögerung vor der Ausführung** im Bearbeitungsformular ändert dies: Die Plattform reiht den Trigger in die Warteschlange ein und führt den Agenten zum geplanten Zeitpunkt aus.

### Wann eine Verzögerung sinnvoll ist

- **Flag-Schwellenwert-Trigger** - Flags kommen oft in Schüben an. Eine Verzögerung von 10-30 Minuten lässt die Lage sich beruhigen, sodass der Agent auf der endgültigen Flag-Anzahl basiert und nicht auf dem Zeitpunkt des Eingangs.
- **Vote-Schwellenwert-Trigger** - dieselbe Logik, insbesondere bei Downvote-Brigading.
- **Thread-Zusammenfassung** - die [Thread Summarizer Vorlage](#template-thread-summarizer) verwendet standardmäßig eine Verzögerung von 30 Minuten, sodass sie eine Konversation zusammenfasst, die Zeit hatte sich zu entwickeln, und nicht einen Thread mit nur zwei Antworten.
- **Abklingzeit / Neubewertung** - „24 Stunden nach dem Sperren eines Kommentars, prüfen, ob er entsperrt werden soll."

### Konfiguration

- **Feld**: Verzögerung vor der Ausführung.
- **Bereich**: 0 bis 2.592.000 Sekunden (30 Tage).
- **Einheiten**: Sekunden, Minuten, Stunden oder Tage.

### Idempotenz

Die verzögerte Warteschlange dedupliziert Trigger nicht. Zwei Flags, die 1 Sekunde voneinander entfernt bei einem Agenten mit 30-minütiger Verzögerung eintreffen, planen beide eine Ausführung 30 Minuten später, und der Agent wird **zweimal** ausgeführt, beide Male gegen (weitgehend) denselben Kontext. Wenn Sie eine Semantik von höchstens einer Ausführung pro Zeitfenster wünschen, muss der Agent dies selbst durchsetzen - typischerweise durch das Schreiben einer [Speichernotiz](#tools-overview) bei der ersten Ausführung und deren Prüfung bei nachfolgenden Ausführungen.

### Kostenhinweis

Verzögerte Trigger werden **vor** ihrer Ausführung erfasst. Eine Häufung von Triggern bei einem Agenten mit hoher Verzögerung kann sich in der Warteschlange ansammeln, ohne Tokens zu verbrauchen; die Kosten fallen erst an, wenn der Cron sie versendet. Verwenden Sie [Ausführungsverlauf](#run-history) und [Verwerfungsgründe](#drop-reasons), um zu sehen, wie oft verzögerte Trigger tatsächlich ausgeführt werden, verglichen mit dem Verwerfen zur Laufzeit aus Budgetgründen.

### Replay berücksichtigt die Verzögerung nicht

Die Funktion [Testläufe (Replays)](#test-runs-replays) führt den Agenten sofort gegen historische Kommentare aus - sie wartet nicht auf die konfigurierte Verzögerung. Betrachten Sie dies als Feature: Replays dienen der Vorschau, was der Agent **tun würde** bei gegebenem Kontext, nicht der Reproduktion der Echtzeit-Planung.