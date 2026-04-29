Ein Agent hat einen von drei Status:

### Disabled

Der Agent ist ausgeschaltet. Es werden keine Trigger verarbeitet und der Agent erscheint nicht im Dispatch-Pfad. Seine Laufhistorie, Analysen und sein Speicher bleiben erhalten - wenn Sie ihn später wieder aktivieren, sind die historischen Daten noch da.

Verwenden Sie `Disabled`, wenn:
- Sie einen Agenten aus dem Rotation nehmen möchten, ohne ihn zu verlieren.
- Ein Agent sich fehlverhält und Sie ihn sofort stoppen müssen, während Sie untersuchen.
- Sie Agenten saisonal hinein- und herausrotieren (z. B. ein nur während der Feiertage eingesetzter Begrüßer).

### Dry Run - default for new agents

Der Agent läuft End-to-End - er verarbeitet Trigger, ruft das LLM auf, wählt Tool-Aufrufe aus, berechnet Begründungen und Vertrauen - aber **keine echte Aktion wird ausgeführt**. Jeder Lauf wird mit dem **Dry Run**-Badge in [Run History](#run-history) aufgezeichnet.

Verwenden Sie `Dry Run`, wenn:
- Ein neuer Agent gerade aus der Verpackung ist. Jede Starter-Vorlage landet im dry-run.
- Sie das Prompt bearbeitet oder die Triggermenge geändert haben und sehen möchten, wie sich die Änderung auswirkt, bevor Sie sie übernehmen.
- Sie einen [test run / replay](#test-runs-replays) durchführen (Replays erzwingen dry-run unabhängig vom Agentenstatus).

Die Plattform berechnet Tokens für dry-run Läufe - der LLM-Aufruf findet weiterhin statt, nur die Nebenwirkungen werden übersprungen. Budgetgrenzen gelten auch für dry-run. Siehe [Budgets Overview](#budgets-overview).

### Enabled

Der Agent führt echte Aktionen aus. Tool-Aufrufe werden ausgeführt - oder zur [approval](#approval-workflow) in die Warteschlange gestellt, wenn die Aktion genehmigungspflichtig ist.

Verwenden Sie `Enabled`, nachdem die dry-run-Ausgabe korrekt aussieht.

### Switching status

Sie können auf dem Bearbeitungsformular zwischen beliebigen zwei Status wechseln. Ein Wechsel von Dry Run zu Enabled führt nicht dazu, dass die Dry-Run-Aktionen rückwirkend erneut ausgeführt werden - diese bleiben als Dry-Run-Historie erhalten. Neue Trigger ab diesem Zeitpunkt laufen live.

Ein Wechsel von Enabled zu Disabled während eines Laufs bricht einen laufenden Durchlauf **nicht** ab. Der aktuell ausgeführte Trigger wird fertiggestellt (mit allem, was er bereits gestartet hat); der nächste Trigger wird verworfen, weil der Agent jetzt Disabled ist.

### Status during billing problems

Wenn die Abrechnung Ihres Tenants ungültig wird, werden alle Agenten effektiv pausiert, unabhängig vom gespeicherten Status - Trigger werden mit `BILLING_INVALID` verworfen, bis die Abrechnung wiederhergestellt ist. Das gespeicherte Statusfeld wird nicht geändert; der Dispatcher weigert sich einfach auszuführen. Siehe [Plans and Eligibility](#plans-and-eligibility).