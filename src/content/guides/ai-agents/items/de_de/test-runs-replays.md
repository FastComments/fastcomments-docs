A **Testlauf** (auch **Replay** genannt) führt den Agenten gegen ein Fenster historischer Kommentare aus, **ohne echte Aktionen auszuführen**. Es ist der schnellste Weg, das Verhalten des Agenten vor dem Live-Betrieb zu prüfen.

Erreichbar von der Agenten-Übersichtsseite über die Schaltfläche **Testlauf** in der Zeile jedes Agenten.

### Was es macht

Die Plattform:

1. Wählt eine Stichprobe historischer Kommentare aus, die dem Scope des Agenten entsprechen, im von Ihnen gewählten Zeitfenster.
2. Führt für jeden Kommentar den Agenten vollständig durch, als wäre der Kommentar gerade erst gepostet worden - gleicher Kontext, gleicher LLM-Aufruf, gleiche Tool-Auswahl, gleiche Begründungen und Konfidenzwerte.
3. Zeichnet jeden Lauf als Dry-Run auf, taggt ihn so, dass er mit dem Replay gruppiert bleibt und von Live-Run-Ansichten ausgeschlossen ist.
4. **Vergleicht** das Urteil des Agenten mit dem, was tatsächlich mit dem Kommentar passiert ist – wurde er später genehmigt, als Spam markiert, gelöscht, vom Spam-Engine blockiert etc.

Das Ergebnis ist ein Diff pro Kommentar: "Der Replay-Agent hätte dies als Spam markiert, aber der Kommentar ist derzeit genehmigt und sauber."

### Konfiguration

Die Testlauf-Seite hat eine einzige Eingabe:

- **Tage historischer Kommentare zur Auswertung** – ein numerisches Feld `days` zwischen 1 und 90. Ältere Kommentare sind nicht berechtigt.

Stichprobengröße und Obergrenze sind **nicht in der UI sichtbar** – beides sind serverseitige Standardwerte, die pro Plan angewendet werden. Die Seite zeigt Informationsfelder an:

- **Übereinstimmende Kommentare im Fenster** – wie viele Kommentare in Betracht gezogen würden.
- **Bis zu N Kommentare aus diesem Fenster werden verarbeitet** – die effektive Stichprobengröße unter Berücksichtigung der serverseitigen Obergrenze.
- **Geschätzte Kosten** – in der Währung Ihres Tenants.

### Rate-Limit

Jeder Benutzer ist auf **10 Testläufe pro 24 Stunden** beschränkt (Rate-Limit via key `replay-create:${requestedBy}`). Die Schaltfläche zeigt einen Tooltip, wenn Sie das Limit erreicht haben ("You've reached 10 test runs in the last 24 hours.").

### Gleichzeitigkeit

Pro Agent kann nur ein Replay gleichzeitig aktiv sein. Wenn Sie einen zweiten Replay starten, während einer läuft, werden Sie zu dem laufenden Replay weitergeleitet.

### Ergebnisse lesen

Wenn das Replay abgeschlossen ist, zeigt die Ergebnisseite Tabs an:

- **Differenzen** (standardmäßig aktiv) – das Urteil des Replay-Agenten unterscheidet sich von der Realität. (Am interessantesten – "der Agent hätte diesen Kommentar als Spam markiert, aber der Kommentar wurde genehmigt und ist in Ordnung".)
- **Übereinstimmungen** – das Urteil des Replay-Agenten stimmt mit dem überein, was tatsächlich passiert ist. (Beruhigend – der Agent stimmt mit der Realität überein.)
- **Keine Aktion** – der Replay-Agent hat entschieden, nichts zu tun. (Manchmal die richtige Antwort; manchmal hat der Agent etwas übersehen.)
- **Alle** – jedes Ergebnis unabhängig von der Klassifikation.

Für jeden Kommentar in einem Tab:

- **Vorheriges Ergebnis** – die Klassifikation dessen, was tatsächlich passiert ist: **POSITIV**, **NEGATIV** oder **UNBESTIMMT**, mit **Belegen** ("Kommentar am {date} als gelöscht markiert", "Engine: bayes" und so weiter).
- **Replay-Agent würde** – die Aktion, die der Agent gewählt hat.
- **Warum** – die Begründung.
- **Konfidenz** – angezeigt als Prozentsatz.

### Warum Replays den Trockenlauf erzwingen

Ein Replay gegen einen Kommentar, der vor vier Monaten gelöscht wurde, sollte ihn nicht nachträglich löschen – er ist bereits gelöscht. Ein Replay gegen einen Kommentar, den der Agent jetzt genehmigen würde, sollte den aktuellen Zustand des Kommentars nicht ändern. Replay ist ein Vorschau-Werkzeug. Das Erzwingen des Dry-Runs macht es sicher, ein Replay gegen beliebige historische Zeitfenster auszuführen.

### Reproduzierbarkeit

Replays frieren die Konfiguration des Agenten zum Zeitpunkt des Replay-Starts ein. Nachfolgende Änderungen am Agenten beeinflussen die Ergebnisse des Replays nicht – die Ergebnisseite bleibt stabil als Aufzeichnung dessen, was genau diese Version des Agenten getan hätte.

### Wenn Budgets ein Replay stoppen

Replays unterliegen:

- Ihrer eigenen **harten Obergrenze** (im Replay-Formular festgelegt).
- Den täglichen und monatlichen **Budgetgrenzen** des Agenten.
- Den täglichen und monatlichen **Budgetgrenzen** des Tenants.

Die zuerst erreichte Grenze bricht das Replay mit einem spezifischen Fehlercode ab. Alle pro-Kommentar-Ergebnisse, die vor dem Abbruch erzeugt wurden, werden in [Ausführungsverlauf](#run-history) gespeichert.

### Wie Replays ausgeführt werden

Replays laufen im Hintergrund, nicht synchron. Nachdem Sie auf "Testlauf starten" geklickt haben, wird das Replay in die Warteschlange gestellt und ein Worker nimmt es auf. Ein langer Replay kann mehrere Minuten dauern. Die Ergebnisseite pollt und zeigt den Fortschritt (verarbeitete Anzahl, bisher ausgegebener Betrag) während des Laufs an.

Wenn ein Worker mitten im Replay abstürzt, requeue die Plattform das Replay automatisch, sodass es beim nächsten Durchlauf fortgesetzt wird. Ein kurzer Aussetzer lässt ein Replay niemals verwaist.

### Was ein Replay nicht macht

- **Beachtet keine [Trigger-Verzögerungen](#trigger-deferred-delay).** Replays laufen sofort, nicht 30 Minuten später.
- **Schreibt nicht in das Memory.** Replay-Agenten speichern keine Memory-Notizen, selbst wenn ihre Logik das normalerweise tun würde.
- **Feuern keine Webhooks aus.** Durch Replays erzeugte Trigger generieren keine `trigger.succeeded` / `trigger.failed` Webhook-Events.
- **Schließen nicht bereits wiedergegebene Kommentare aus.** Einen zweiten Replay gegen dasselbe Fenster auszuführen, deckt dieselben Kommentare ab.

### Siehe auch

- [Prompts verfeinern](#refining-prompts) – der iterative Bearbeitungs-Workflow, der gut mit Replays zusammenarbeitet.
- [Trockenlauf-Modus](#dry-run-mode) – das gleiche Konzept, gegen Live-Traffic.