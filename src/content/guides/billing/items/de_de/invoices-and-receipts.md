Jede Abbuchung – sei es eine Planverlängerung, ein Flex-Monat oder eine Planänderung – erzeugt eine Rechnung. Rechnungen dienen nach Bezahlung gleichzeitig als Quittung.

### Rechnungen finden

Öffnen Sie [Billing History](https://fastcomments.com/auth/my-account/billing-history) oder klicken Sie auf der Seite Billing Info auf `View Invoices`. Die Tabelle listet jede Rechnung mit Datum, Abrechnungszeitraum, Rechnungsnummer, Beschreibung, Gesamtbetrag, Status (`Paid`, `Unpaid`, `Draft` oder `Void`) und Fälligkeitsdatum auf, zehn pro Seite, neueste zuerst.

Für jede Rechnung:

- `View` öffnet die Rechnung auf der gehosteten Seite von Stripe, wo Sie sie auch herunterladen können.
- `PDF` lädt das PDF direkt herunter.
- `Pay Now` erscheint anstelle von `View` bei einer unbezahlten Rechnung und führt Sie zur Zahlungsseite von Stripe für diese Rechnung, wo Sie sie direkt bezahlen können.

Rechnungen werden bei Erstellung auch an Ihre Rechnungs-E-Mail gesendet. Damit sie stattdessen an ein Finance-Postfach statt an den Kontoinhaber gehen, legen Sie die `Billing Email` in den [Invoice Settings](#invoice-settings) fest.

### Was auf der Rechnung steht

Die Rechnung zeigt den Rechnungsnamen und die Adresse sowie die VAT / Tax ID aus den [Invoice Settings](#invoice-settings), die Positionen, angerechnetes vorausbezahltes [Guthaben](#credits) und den Betrag, der Ihrer Karte belastet wurde. Flex-Rechnungen listen eine Position pro Nutzungsart auf, siehe [How Flex Usage Is Measured](#how-flex-usage-is-measured). Für Rechnungsadressen in der Europäischen Union werden Flex-Rechnungen mit 0% MwSt. und dem Fußtext `Reverse Charge Transaction: Customer to pay VAT` ausgestellt.

Änderungen an den Rechnungseinstellungen gelten für Rechnungen, die nach dem Speichern erstellt werden. Eine bereits ausgestellte Rechnung kann nicht bearbeitet werden, daher legen Sie Ihre Firmendaten und die VAT / Tax ID vor dem nächsten Abrechnungsdatum fest.

### Konten, die anderswo abgerechnet werden

Wenn Ihre Abrechnung extern erfolgt, zum Beispiel per Rechnung in Absprache mit dem Support, werden Ihnen Rechnungen zur Zahlung zugesandt, anstatt eine Karte zu belasten, und sie erscheinen möglicherweise nicht in der Billing History. Über Shopify oder BigCommerce installierte Konten werden von dieser Plattform abgerechnet, und die Seite Billing Info verlinkt auf die dortige Plan-Seite. Unterkonten, die über ein Elternkonto abgerechnet werden, haben keine eigenen Rechnungen; siehe [When You Are Charged](#when-you-are-charged).