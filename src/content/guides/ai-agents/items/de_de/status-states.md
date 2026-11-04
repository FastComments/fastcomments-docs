Ein Agent hat einen von drei Statusen:

### Disabled

Der Agent ist ausgeschaltet. Es werden keine Trigger verarbeitet und der Agent erscheint nicht im Dispatch-Pfad. Sein Run-Verlauf, Analysen und Speicher bleiben erhalten - wenn Sie ihn später wieder aktivieren, sind die historischen Daten noch da.

Verwenden Sie `Disabled`, wenn:
- Sie einen Agenten aus der Rotation nehmen möchten, ohne ihn zu verlieren.
- Ein Agent sich fehlerhaft verhält und Sie ihn sofort stoppen müssen, während Sie der Sache nachgehen.
- Sie Agenten saisonal ein- und auswechseln (z. B. einen nur für Feiertage aktiven Begrüßer).

### Dry Run - Standard für neue Agenten

Der Agent läuft vollständig durch - er verarbeitet Trigger, ruft das LLM auf, wählt Tool-Aufrufe aus, berechnet Begründungen und Konfidenz - aber **es wird keine echte Aktion ausgeführt**. Jeder Lauf wird mit dem **Dry Run**-Badge in [Run History](#run-history) aufgezeichnet.

Verwenden Sie `Dry Run`, wenn:
- Ein neuer Agent gerade erst eingerichtet wurde. Jede Starter-Vorlage landet im Dry-Run.
- Sie den Prompt bearbeitet oder die Trigger-Menge geändert haben und sehen möchten, wie sich die Änderung auswirkt, bevor Sie sie verbindlich machen.
- Sie einen [Testlauf / Replay](#test-runs-replays) ausführen (Replays erzwingen den Dry-Run unabhängig vom Agentenstatus).

Die Plattform berechnet Token für Dry-Run-Läufe - der LLM-Aufruf findet weiterhin statt, nur die Nebeneffekte werden übersprungen. Budgetgrenzen gelten auch für den Dry-Run. Siehe [Budgets Overview](#budgets-overview).

### Enabled

Der Agent führt echte Aktionen aus. Tool-Aufrufe werden ausgeführt - oder zur [Genehmigung](#approval-workflow) in die Warteschlange gestellt, wenn die Aktion gesperrt ist.

Verwenden Sie `Enabled`, wenn die Dry-Run-Ausgabe korrekt aussieht.

### Wechseln des Status

Sie können im Bearbeitungsformular zwischen beliebigen zwei Statusen wechseln. Ein Wechsel von Dry Run zu Enabled führt die Dry-Run-Aktionen nicht rückwirkend erneut aus - diese bleiben als Dry-Run-Verlauf bestehen. Neue Trigger ab diesem Zeitpunkt laufen live.

Ein Wechsel von Enabled zu Disabled während eines Laufs bricht einen laufenden Vorgang **nicht** ab. Der aktuell ausgeführte Trigger wird abgeschlossen (mit allem, was er bereits gestartet hat); der nächste Trigger wird verworfen, da der Agent jetzt Disabled ist.

### Status bei Abrechnungsproblemen

Wenn die Abrechnung Ihres Tenants ungültig wird, werden alle Agenten unabhängig vom gespeicherten Status effektiv pausiert - Trigger werden mit `BILLING_INVALID` verworfen, bis die Abrechnung wiederhergestellt ist. Das gespeicherte Statusfeld wird nicht geändert; der Dispatcher verweigert einfach die Ausführung. Siehe [Plans and Eligibility](#plans-and-eligibility).