---
FastComments ist eine lokalisierte Plattform. Alle unsere Widgets, E-Mails und Benachrichtigungen sind lokalisiert.

Lokalisiert bedeutet, dass wir je nach Standort und bevorzugter Sprache des Benutzers eine andere Sprache und Formatierung anzeigen. Wir ermitteln dies anhand der Informationen, die der Browser des Benutzers uns zur Verfügung stellt.

Wir können den Text in der E-Mail anpassen, indem wir auf die `Translations`-Registerkarte gehen, eine `Locale` auswählen und den Text bearbeiten. Text, der vom Standard abweicht, ist in der UI hervorgehoben. Sie können zwischen Locales wechseln und am Ende speichern, ohne Änderungen zu verlieren.

Lokalisierten Text greift man über das `TEXT`-Objekt ab, zum Beispiel: `<%= TEXT.INTRO %>`.

### SSO-Hinweis

Bei SSO-Integrationen: Wenn `locale` nicht angegeben ist, wird es jedes Mal aktualisiert, wenn der Benutzer mit einer anderen Locale auf das Kommentar-Widget zugreift. Das bedeutet, dass die Sprachpräferenz des Benutzers automatisch aktualisiert wird und zukünftige E-Mails in dieser Locale gesendet werden.

Dies kann auch manuell gesetzt werden, indem `locale` im SSO-Payload übergeben wird.

---