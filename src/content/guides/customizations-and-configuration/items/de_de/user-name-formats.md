Standardmäßig zeigt FastComments den Namen des Benutzers so an, wie er ihn eingegeben hat, oder wie er uns über SSO übermittelt wurde.

Es kann jedoch wünschenswert sein, den Namen des Benutzers zu verschleiern oder auf andere Weise anzuzeigen. Wenn der Name des Benutzers beispielsweise Allen Rex lautet, möchten Sie vielleicht nur „Allen R." anzeigen.

Dies kann ohne Code in der Widget-Anpassungs-UI unter der Einstellung `Commenter Name Format` erfolgen:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.commenter-name-format select'; selector = '.commenter-name-format'; alt='Geöffnetes Dropdown Commenter Name Format mit Optionen wie Capitalize, Last Initial und All Initials'; title='Namensformat ändern' app-screenshot-end]

Die verfügbaren Formate sind:

- Capitalize (zeigt den Beispielbenutzer als Example User an)
- Last Initial (zeigt Example User als Example U. an)
- All Initials (zeigt Example User als E. U. an)
- „Anonymous" anzeigen

Die Änderung wirkt sich sofort aus. Benutzer sehen weiterhin ihren vollständigen Benutzernamen für sich selbst oben im Kommentarbereich, aber ihre Kommentare zeigen den geänderten Benutzernamen.

Benutzernamen werden serverseitig verschleiert, um Benutzer zu schützen.