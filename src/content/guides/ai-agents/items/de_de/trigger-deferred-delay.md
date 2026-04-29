Standardmäßig läuft ein Agent **sofort** nachdem sein Trigger ausgelöst wird. Das Feld **Delay before running** im Bearbeitungsformular ändert das: die Plattform stellt den Trigger in die Warteschlange und führt den Agenten zum geplanten Zeitpunkt aus.

### When to use a delay

- **Flag-Schwellen-Trigger** - Flags kommen oft in Schüben an. Eine Verzögerung von 10–30 Minuten lässt das Bild sich beruhigen, sodass der Agent auf die endgültige Anzahl von Flags reagiert statt auf den Moment des Eintreffens.
- **Vote-Schwellen-Trigger** - gleiche Logik, insbesondere bei Downvote-Brigaden.
- **Thread summarization** - die [Thread Summarizer template](#template-thread-summarizer) verwendet standardmäßig eine 30-minütige Verzögerung, sodass sie eine Unterhaltung zusammenfasst, die Zeit zum Entwickeln hatte, und nicht einen Thread mit nur zwei Antworten.
- **Cool-down / re-evaluation** - "24 hours after a comment is locked, consider whether to unlock it."

### Konfiguration

- **Feld**: Delay before running.
- **Bereich**: 0 to 2,592,000 seconds (30 days).
- **Einheiten**: Sekunden, Minuten, Stunden oder Tage.

### Idempotenz

Die verzögerte Warteschlange de-dupliziert Trigger nicht. Zwei Flags, die im Abstand von 1 Sekunde bei einem Agenten mit 30-Minuten-Verzögerung eintreffen, planen beide eine Ausführung 30 Minuten später, und der Agent wird **zweimal** ausgeführt, jeweils gegen (weitgehend) denselben Kontext. Wenn Sie eine Höchstens-eine-Ausführung-pro-Fenster-Semantik wünschen, muss der Agent das durchsetzen – typischerweise indem er beim ersten Lauf eine [memory note](#tools-overview) schreibt und bei nachfolgenden Läufen darauf prüft.

### Kostenhinweis

Verzögerte Trigger werden **aufgezeichnet, bevor** sie ausgeführt werden. Ein Schub von Triggern bei einem Agenten mit hoher Verzögerung kann sich in der Warteschlange ansammeln, ohne Tokens zu verbrauchen; die Kosten fallen erst an, wenn der Cron sie abarbeitet. Verwenden Sie [Run History](#run-history) und [Drop Reasons](#drop-reasons), um zu sehen, wie oft verzögerte Trigger tatsächlich ausgeführt werden im Vergleich dazu, wie oft sie zur Laufzeit aus Budgetgründen verworfen werden.

### Replay does not honor delay

Die Funktion [Test Runs (Replays)](#test-runs-replays) führt den Agenten sofort gegen historische Kommentare aus — sie wartet nicht auf die konfigurierte Verzögerung. Betrachten Sie das als Feature: Replays dienen dazu, eine Vorschau dessen zu erhalten, was der Agent **tun würde** angesichts des Kontexts, nicht dazu, die Echtzeit-Planung zu reproduzieren.