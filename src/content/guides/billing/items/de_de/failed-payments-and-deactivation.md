Eine Zahlung kann fehlschlagen, weil eine Karte abgelaufen ist, ersetzt wurde, kein Guthaben vorhanden ist oder die Bank die Abbuchung blockiert hat. FastComments gibt Ihnen Zeit, das Problem zu beheben, bevor etwas nicht mehr funktioniert, und informiert die Personen, die es beheben können.

### Was passiert

1. **Die Abbuchung schlägt fehl.** Die Rechnung bleibt in der [Billing History](https://fastcomments.com/auth/my-account/billing-history) mit dem Status `Unpaid` und einem `Pay Now`-Button stehen. Bei einem festen Tarif kann Stripe die Abbuchung wiederholen. Bei Flex wiederholt FastComments jede Nacht alle unbezahlten Rechnungen, die älter als die Kulanzfrist sind.
2. **Eine Warnung wird gesendet.** Jede Nacht prüft FastComments jedes Konto. Wenn eine Flex-Rechnung nach Ablauf der Kulanzfrist weiterhin unbezahlt ist und der Wiederholungsversuch fehlschlägt, oder wenn ein Abo mit festem Tarif beendet wurde, weil es nicht eingezogen werden konnte, erhält jeder Benutzer mit der Berechtigung `Billing Admin` die E-Mail `Your FastComments Tenant Will Be Deactivated Soon!` mit dem Datum, und das Konto wird zur Deaktivierung eine Kulanzfrist später eingeplant. Das Widget funktioniert in der Zwischenzeit weiterhin.
3. **Deaktivierung.** Wenn sich bis zum eingeplanten Datum nichts geändert hat, wird das Konto deaktiviert: Kommentare werden auf Ihrer Website ausgeblendet und Besucher sehen `This FastComments account's billing information is more than sixty days out of date. Comments hidden until the issue is resolved.` Billing-Administratoren werden erneut per E-Mail benachrichtigt. Ihre Daten bleiben erhalten.

Die Kulanzfrist beträgt eine Woche für die meisten Konten und zwei Wochen für Konten, die älter als ein Jahr sind. Bei Flex gilt sie zweimal, einmal vor der Warnung und einmal zwischen der Warnung und der Deaktivierung, sodass zwischen einer fehlgeschlagenen Abbuchung und dem Ausblenden der Kommentare mindestens zwei Wochen liegen.

### Behebung

Hinterlegen Sie eine funktionierende Karte auf der Seite [Billing Info](https://fastcomments.com/auth/my-account/billing-info). Das allein reaktiviert das Konto sofort und bricht jede geplante Deaktivierung ab; die unbezahlte Rechnung wird dann mit der neuen Karte eingezogen – bei einem festen Tarif sofort und bei Flex während der nächtlichen Prüfung. Sie können die Rechnung auch direkt über `Pay Now` in der Billing History bezahlen.

Wenn die hinterlegte Karte in Ordnung ist und der Fehler nur vorübergehend war, müssen Sie nichts unternehmen: Die nächtliche Prüfung erkennt die bezahlte Rechnung oder das aktive Abo, bricht die Deaktivierung ab und sendet den Billing-Administratoren eine Bestätigungs-E-Mail. Ein bereits deaktiviertes Konto wird auf dieselbe Weise reaktiviert.

### Mehrere Rechnungen gleichzeitig

War Ihre Karte einige Monate lang ungültig und wurde dann korrigiert, wird jeder unbezahlte Monat separat eingezogen, sodass Sie möglicherweise mehrere Abbuchungen am selben Tag sehen. Prüfen Sie den Abrechnungszeitraum jeder Rechnung in der Billing History: Sie betreffen unterschiedliche Monate und sind keine Duplikate.

### Sehen Sie nach der Zahlung immer noch eine Rechnungswarnung?

Die Warnung gehört zu einem bestimmten Konto. Wenn Sie bezahlt haben, aber eine Website weiterhin `billing information is out of date` anzeigt, verwendet das Widget auf dieser Website höchstwahrscheinlich ein anderes Konto als das, auf dem Sie bezahlt haben. Dies passiert, wenn eine Website zweimal eingerichtet wurde oder wenn es ein Konto auf `fastcomments.com` und ein weiteres auf der EU-Bereitstellung unter `eu.fastcomments.com` gibt, die separat abgerechnet werden. Vergleichen Sie die `tenantId` im Code Ihrer Seite mit der unter `Settings -> API/SSO`, oder [kontaktieren Sie den Support](https://fastcomments.com/auth/my-account/help), und wir klären das.