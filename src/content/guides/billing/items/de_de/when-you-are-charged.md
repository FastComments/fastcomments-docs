Wann und wie viel Ihrer Karte belastet wird, hängt von der Art Ihres Tarifs ab.

### Feste Tarife

Wenn Sie ein `Creators`- oder `Pro`-Abonnement starten, wird der erste Monat oder das erste Jahr sofort im Browser abgerechnet, wenn Sie auf `Pay Now & Start Subscription` klicken. Wenn Ihre Bank Sie bittet, die Zahlung zu bestätigen, schließen Sie die Prüfung im sich öffnenden Fenster ab. Das Abonnement beginnt erst, sobald die Zahlung durchgegangen ist.

Danach verlängert sich das Abonnement am selben Tag jedes Monats bzw. jedes Jahres, wenn Sie `Pay Annually` gewählt haben, gezählt ab dem Tag Ihrer Anmeldung. Die Verlängerung wird automatisch der hinterlegten Karte belastet und die Rechnung an Ihre Rechnungs-E-Mail gesendet. Es ist keine Aktion Ihrerseits erforderlich.

Die jährliche Abrechnung ist 15% günstiger als die monatliche. Flex kann nicht jährlich abgerechnet werden.

### Flex

Flex wird nachträglich abgerechnet: Sie nutzen den Dienst während eines Kalendermonats, und die Rechnung für diesen Monat wird am letzten Tag des Monats (UTC) erstellt und sofort Ihrer Karte belastet. Das Abrechnungsdatum ist immer der letzte Tag des Monats und kann nicht geändert werden.

Die Rechnung deckt nur diesen Kalendermonat ab. Die Nutzung im Kalendermonat, in dem Sie sich angemeldet haben, wird nie abgerechnet, daher erhalten Sie Ihre erste Rechnung frühestens am Ende des Monats nach Ihrer Anmeldung.

Wenn Ihre Nutzung unter dem Flex-Mindestbetrag liegt, ergänzt eine Position mit der Bezeichnung `FastComments Flex. (minimum ... /mo)` die Rechnung auf den Mindestbetrag. Der aktuelle Mindestbetrag wird auf Ihrer Seite [Billing Info](https://fastcomments.com/auth/my-account/billing-info) und auf der [Preisseite](https://fastcomments.com/traffic-pricing) angezeigt.

Sie können jederzeit einsehen, wie die nächste Rechnung ausfallen wird. Der `Next Invoice Amount` befindet sich auf der Seite [Billing Info](https://fastcomments.com/auth/my-account/billing-info), und die vollständige Aufschlüsselung Zeile für Zeile finden Sie auf der Seite [Billing Analytics](https://fastcomments.com/auth/my-account/analytics/billing). Was gezählt wird, ist unter [How Flex Usage Is Measured](#how-flex-usage-is-measured) beschrieben.

### Das Speichern einer Karte löst keine Belastung aus

Sobald Ihre Abrechnung eingerichtet ist, wird das Speichern oder Aktualisieren einer Karte ohne Tarifwechsel nie zu einer Belastung führen, es sei denn, Sie haben eine unbezahlte Rechnung. Das Formular besagt `You won't be charged now unless you change plans.` und die Karte wird einfach bei Stripe für die nächste Rechnung hinterlegt. Nur wenn Sie noch keine gültige Abrechnung haben, sagt das Formular `Your card will be charged now.`, und selbst dann wird ein Flex-Konto erst am Ende des Monats belastet. Etwaiges vorhandenes [Guthaben](#credits) auf Ihrem Konto wird vor der Belastung der Karte mit jeder Rechnung verrechnet.

### Verwaltete Sub-Konten

Wenn Ihr Konto Sub-Konten verwaltet (z. B. eines pro Website oder Umgebung, oder White-Label-Mandanten), wird nur das übergeordnete Konto abgerechnet. Die Nutzung jedes Sub-Kontos erscheint als eigene Positionen auf der Flex-Rechnung des übergeordneten Kontos, mit dem Namen des Sub-Kontos als Präfix, und Sub-Konten teilen sich die Zahlungsmethode und den Abrechnungsstatus des übergeordneten Kontos.