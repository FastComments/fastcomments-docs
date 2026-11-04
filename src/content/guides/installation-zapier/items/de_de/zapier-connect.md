## Konto verbinden

1. Fügen Sie in Zapier einen FastComments-Schritt zu einem Zap hinzu oder öffnen Sie die FastComments-App-Seite im Zapier App Directory.
2. Wählen Sie **Sign in to FastComments**. Zapier fragt zuerst nach Ihrer Region: Wählen Sie **United States**, es sei denn, Ihr
   Konto wurde in der EU-Region erstellt (`eu.fastcomments.com`).
3. Ein FastComments-Fenster öffnet sich. Melden Sie sich an, falls Sie nicht bereits angemeldet sind.
4. Prüfen Sie die Zustimmungsseite. Sie zeigt die Zapier-Anwendung, das Konto, mit dem sie verbunden wird, und die
   angeforderten Berechtigungen (Lesen und Schreiben). Wählen Sie **Approve**.
5. Zapier speichert die Verbindung und beschriftet sie mit Ihrem Website-Namen und Benutzernamen.

Die Verbindung verwendet OAuth. Es wird kein API-Schlüssel in Zapier kopiert, und das Token, das Zapier hält, funktioniert
nur für das Konto, das Sie genehmigt haben.

## Wer eine Verbindung herstellen kann

Die Person, die die Verbindung genehmigt, muss ein **API-Administrator** auf dem FastComments-Konto sein. Kontoinhaber
haben diese Berechtigung; andere Teammitglieder können sie auf der Benutzerseite erhalten. Jemand ohne diese Berechtigung
sieht eine Seite mit der Meldung „Sie haben keine Berechtigung" anstelle des Zustimmungsformulars.

## Die richtige Website verbinden

Die Zustimmungsseite verbindet das Konto, bei dem Sie aktuell angemeldet sind. Wenn Sie mehrere Konten verwalten,
wechseln Sie vor der Genehmigung über den Konto-Umschalter zum richtigen Konto oder verwenden Sie den Link
**switch account** auf der Zustimmungsseite. Das Verbindungslabel in Zapier zeigt den Website-Namen, sodass eine falsche
Auswahl leicht zu erkennen ist.

## Zugriff überprüfen und widerrufen

Jede Verbindung erscheint unter **Connected Apps** im FastComments-Dashboard, mit den Berechtigungen, die sie besitzt,
und wann sie zuletzt verwendet wurde. Wenn Sie sie dort widerrufen, wird Zapier sofort getrennt; jeder Zap, der diese
Verbindung verwendet, stoppt, bis sie wieder verbunden ist. Sie können die Verbindung auch von der Zapier-Seite unter
**My Apps** entfernen.