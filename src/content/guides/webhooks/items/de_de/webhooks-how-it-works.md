---
Alle Änderungen am Comment-Objekt im System lösen ein Ereignis aus, das in eine Warteschlange gelangt.

Das initiale Webhook-Ereignis wird normalerweise innerhalb von sechs Sekunden nach dem Auftreten der Ereignisquelle gesendet.

Sie können diese Warteschlange im Webhooks-Admin überwachen für den Fall, dass Ihre API ausfällt.

Wenn eine Anfrage an Ihre API fehlschlägt, stellen wir sie nach einem Zeitplan erneut in die Warteschlange.

Dieser Zeitplan ist `1 Minute * the retry count`. Wenn der Aufruf einmal fehlschlägt, wird er in
einer Minute erneut versucht. Wenn er zweimal fehlschlägt, wird er anschließend zwei Minuten warten, und so weiter. Dies dient dazu, Ihre
API nicht zu überlasten, falls sie aufgrund von Lastproblemen ausfällt.

Webhooks können über die [Protokollseite](https://fastcomments.com/auth/my-account/manage-data/webhooks/logs) abgebrochen werden.

---