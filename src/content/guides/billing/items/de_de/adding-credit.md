Aufladungen werden über die auf Ihrem Konto gespeicherte Zahlungsmethode abgerechnet. Wenn Sie noch keine Karte hinterlegt haben, zeigt die Credits-Seite `Add a payment method first` mit einer Schaltfläche `Go to billing` an. Hinterlegen Sie eine Karte auf der Seite [Billing Info](https://fastcomments.com/auth/my-account/billing-info) und kehren Sie dann zurück.

So fügen Sie Guthaben hinzu:

1. Öffnen Sie [My Account -> Billing Info](https://fastcomments.com/auth/my-account/billing-info) und klicken Sie auf `Manage Credits`.
2. Wählen Sie unter `Add credit` einen der voreingestellten Beträge oder wählen Sie `Other amount` und geben Sie einen eigenen Betrag ein.
3. Klicken Sie auf die Schaltfläche `Add`. Diese zeigt den gewählten Betrag an, z. B. `Add $25.00`.
4. Wenn Ihre Bank die Bestätigung der Zahlung verlangt, schließen Sie die Prüfung im sich öffnenden Fenster ab.

Sobald die Zahlung durchgegangen ist, zeigt die Seite `Credit added` mit Ihrem neuen Guthaben an, und die Aufladung erscheint unter `Past top-ups` als `Credited`. Klicken Sie auf `Add more`, um erneut aufzuladen.

### Beträge und Limits

Die Voreinstellungen sind `10`, `25`, `50` und `100` in Ihrer Abrechnungswährung. Ein benutzerdefinierter Betrag kann pro Aufladung zwischen `5` und `10,000` liegen, und die genauen Limits für Ihre Währung werden unter dem Formular angezeigt. Bei Währungen ohne Dezimalstellen, wie dem japanischen Yen, sind die Voreinstellungen und Limits hundertmal größer, sodass sie ungefähr denselben Geldwert darstellen.

Aufladungen erfolgen immer in der Abrechnungswährung Ihres Kontos, derselben Währung wie Ihre Rechnungen. Die Seite zeigt Ihnen an, welche Währung belastet wird.

### Bankbestätigung (3D Secure)

Manche Banken verlangen die Bestätigung einer Online-Zahlung, in der Regel über Ihre Banking-App oder einen Einmalcode. Falls Ihre Bank dies verlangt, öffnet sich nach dem Klick auf `Add` ein Bestätigungsfenster. Ihre Karte wird erst belastet und Ihr Guthaben erst aktualisiert, nachdem die Bank die Zahlung genehmigt hat.

Wenn Sie die Seite schließen, bevor Sie die Bestätigung abgeschlossen haben, wird die Aufladung als `Pending` erfasst. Wenn Sie die Credits-Seite das nächste Mal öffnen, prüfen wir den Zahlungsstatus bei Stripe und markieren die Aufladung entsprechend als `Credited` oder `Failed`. Wenn die Bank die Zahlung nicht genehmigt, zeigt die Seite `Your bank did not approve the payment. Try again, or use a different card.` an, und es wird nichts belastet.

### Abgelehnte Karten

Wenn Ihre Karte abgelehnt wird, zeigt die Seite `Your card was declined. Check your card on the billing page, or try another card.` an. Es wird nichts belastet, Ihr Guthaben ändert sich nicht, und der Versuch wird unter `Past top-ups` als `Failed` aufgeführt. Aktualisieren Sie die Karte auf der Seite [Billing Info](https://fastcomments.com/auth/my-account/billing-info) und versuchen Sie es erneut.

Zum Schutz Ihrer Karte kann ein Konto pro Stunde nur eine begrenzte Anzahl von Aufladeversuchen durchführen. Wenn Sie `Too many top-up attempts. Wait a while and try again.` sehen, warten Sie und versuchen Sie es später erneut.

Zahlungen werden von Stripe abgewickelt. FastComments speichert niemals Ihre Kartendaten.