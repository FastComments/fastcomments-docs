[related-parameter-start name = 'locale'; type = 'string'; related-parameter-end]

Standardmäßig rendert FastComments das Kommentar-Widget in der vom System und Browser des Benutzers ermittelten Locale.

Wenn ein Benutzer kommentiert oder sich anmeldet, aktualisieren wir seine zuletzt verwendete Locale und verwenden diese auch für das Versenden von E-Mails.

Dies wirkt sich darauf aus, wie das Kommentar-Widget für den Benutzer übersetzt wird. Eine Locale besteht aus der Sprache und Region des Benutzers, sodass das Konfigurieren der Locale in der Regel die Sprache ändert, die dem Benutzer angezeigt wird.

#### Über die Benutzeroberfläche

Dies kann über die UI zur Widget-Anpassung definiert werden. Siehe die Option „Locale / Language“:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.locale-override'; alt='Locale / Language Dropdown auf der Widget-Anpassungsseite, das verwendet wird, um die vom Besucher erkannte Locale zu überschreiben'; title='Ändern der Locale / Language' app-screenshot-end]

#### Über Code

Dies kann mit einer gewünschten Locale überschrieben werden.

[code-example-start config = {locale: 'ru_ru'}; linesToHighlight = [6]; title = 'Manuelle Definition der Benutzer-Locale'; code-example-end]

### Unterstützte Sprachen und Locale-Codes

[Hier finden Sie die vollständige Liste der unterstützten Sprachen und der entsprechenden Locale-Codes.](/guide-supported-languages.html#supported-languages)

### SSO-Hinweis

Wenn Sie SSO verwenden, möchten Sie möglicherweise die Locale des Benutzers im Benutzerobjekt übergeben, damit E-Mails und andere Dinge korrekt für ihn lokalisiert werden.

---