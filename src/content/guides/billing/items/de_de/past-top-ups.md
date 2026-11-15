Die Tabelle `Past top-ups` am unteren Rand der Credits-Seite listet Ihre letzten Top-ups auf, die neuesten zuerst, mit `Date`, `Amount` und `Status` jedes Top-ups. Vor Ihrem ersten Top-up zeigt sie `No top-ups yet.`

Jedes Top-up hat einen von drei Statuswerten:

- `Credited`: Die Zahlung war erfolgreich und der Betrag wurde Ihrem Guthaben hinzugefügt.
- `Pending`: Die Zahlung wurde gestartet, aber noch nicht abgeschlossen, meist weil eine Bankbestätigung nicht abgeschlossen wurde. Ausstehende Top-ups werden beim nächsten Öffnen der Credits-Seite mit Stripe abgeglichen und wechseln zu `Credited` oder `Failed`.
- `Failed`: Die Karte wurde abgelehnt oder die Bank hat die Zahlung nicht genehmigt. Es wurde nichts abgebucht und Ihr Guthaben hat sich nicht geändert.

Die Tabelle erfasst Top-ups, die von dieser Seite aus getätigt wurden. Rechnungen, die mit Guthaben bezahlt wurden, werden bei den Rechnungen selbst angezeigt, unter `View Invoices` auf der Seite [Billing Info](https://fastcomments.com/auth/my-account/billing-info).