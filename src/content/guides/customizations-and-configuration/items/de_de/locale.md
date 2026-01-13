[related-parameter-start name = 'locale'; type = 'string'; related-parameter-end]

Standardmäßig zeigt FastComments das Kommentar-Widget in der vom System und Browser des Benutzers bestimmten Locale an.

Wenn ein Benutzer kommentiert oder sich einloggt, aktualisieren wir seine zuletzt verwendete Locale und verwenden diese auch zum Versenden von E-Mails.

Dies beeinflusst, wie das Kommentier-Widget für den Benutzer übersetzt wird. Locale setzt sich aus der Sprache und der Region des Benutzers zusammen, daher ändert das Konfigurieren der Locale in der Regel die Sprache, die zur Anzeige von Text für den Benutzer verwendet wird.

#### Über die Benutzeroberfläche

Dies kann über die UI zur Anpassung des Widgets festgelegt werden. Siehe die Option "Locale / Language":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.locale-override'; title='Changing The Locale / Language' app-screenshot-end]

#### Über Code

Dies kann mit einer gewünschten Locale überschrieben werden.

[code-example-start config = {locale: 'ru_ru'}; linesToHighlight = [6]; title = 'Manually Defining the User\'s Locale'; code-example-end]

### Unterstützte Sprachen und Locale-Codes

[Hier finden Sie die vollständige Liste der unterstützten Sprachen und der entsprechenden Locale-Codes.]( /guide-supported-languages.html#supported-languages)

### SSO-Hinweis

Wenn Sie SSO verwenden, sollten Sie die Locale des Benutzers im Benutzerobjekt übergeben, damit E-Mails und andere Inhalte korrekt für ihn lokalisiert werden.

---