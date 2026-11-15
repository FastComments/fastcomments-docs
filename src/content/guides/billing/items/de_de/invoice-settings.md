Die Seite [Invoice Settings](https://fastcomments.com/auth/my-account/invoice-settings), erreichbar über die Schaltfläche `Invoice Settings` auf Billing Info, steuert, was auf Ihren Rechnungen erscheint und wohin sie gesendet werden. Nur der `Account Owner` oder ein `Billing Admin` kann sie öffnen.

- `Company Name`: der Name Ihres Kontos, wie er im Dashboard, in E-Mails und Teameinladungen angezeigt wird. Dies ist nicht der Rechnungsname.
- `Billing Email`: wohin Rechnungen und Abrechnungsbenachrichtigungen gesendet werden. Stellen Sie dies auf ein Finanz- oder Buchhaltungspostfach ein, wenn der Kontoinhaber sie nicht erhalten soll. Es ändert nicht die Login-E-Mail eines Benutzers.
- `Invoice Currency`: die Währung, in der Ihre Rechnungen ausgestellt werden. Siehe [Currencies](#currencies).
- `Billing Address`: die auf Rechnungen gedruckten Felder `Name`, `Address`, `City`, `State / Province / Region`, `Postal Code` und `Country`. Dieselben Felder erscheinen im Billing-Info-Formular, wenn Sie eine Karte speichern; beide schreiben an denselben Ort.
- `VAT / Tax ID`: optional. Wird auf Ihren Rechnungen gedruckt.

Klicken Sie auf `Save Changes`. Die Seite bestätigt mit `Invoice settings updated successfully.` und die Änderungen gelten für die nächste Rechnung. Bestehende Rechnungen werden nicht geändert.

### VAT And Tax IDs

Wählen Sie Ihr Rechnungsland `Country`, bevor Sie die ID eingeben, da das Land bestimmt, um welche Art von ID es sich handelt: eine EU-USt-IdNr. für jeden EU-Mitgliedstaat, eine britische USt-IdNr. für das Vereinigte Königreich, eine ABN für Australien, eine GST/HST-Nummer für Kanada, eine EIN für die Vereinigten Staaten und so weiter für die anderen von Stripe unterstützten Länder.

Die ID wird beim Speichern validiert. Wenn sie für das ausgewählte Land nicht gültig ist, weist die Seite darauf hin und die ID wird nicht gespeichert, während Ihre anderen Einstellungen gespeichert werden. Wenn Ihr Land nicht zu den Ländern gehört, für die wir eine Steuer-ID erfassen können, weist die Seite ebenfalls darauf hin.

Wenn Sie eine Rechnungsadresse in der EU haben, werden Flex-Rechnungen mit 0% USt als Umkehrbelastungstransaktion ausgestellt, und Ihre USt-IdNr. erscheint nach dem Speichern darauf, sodass Ihr Steuerberater die USt auf Ihrer Seite behandeln kann.