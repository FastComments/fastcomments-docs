Standardmäßig zeigt FastComments den Namen des Benutzers so an, wie er ihn eingegeben hat, oder wie er uns über SSO übermittelt wurde.

Es kann jedoch wünschenswert sein, den Namen des Benutzers zu maskieren oder anders anzuzeigen. Zum Beispiel, wenn der Name des Benutzers Allen Rex ist, möchten Sie vielleicht nur "Allen R." anzeigen.

Dies kann ohne Code in der Widget-Anpassungsoberfläche, unter der Einstellung `Commenter Name Format`, durchgeführt werden:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.commenter-name-format select'; selector = '.commenter-name-format'; title='Change Name Format' app-screenshot-end]

Die verfügbaren Formate sind:

- Großschreibung (zeigt Beispielbenutzer als Beispielbenutzer an)
- Letzter Anfangsbuchstabe (zeigt Beispielbenutzer als Beispiel B. an)
- Alle Initialen (zeigt Beispielbenutzer als B. B. an)
- "Anonym" anzeigen

Die Änderung wirkt sofort. Benutzer sehen ihren vollständigen Benutzernamen weiterhin oben im Kommentarbereich für sich selbst, aber in ihren Kommentaren wird der modifizierte Benutzername angezeigt.

Benutzernamen werden serverseitig maskiert, um die Nutzer zu schützen.

---