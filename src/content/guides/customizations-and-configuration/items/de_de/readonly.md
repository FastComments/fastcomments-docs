[related-parameter-start name = 'readonly'; type = 'boolean'; related-parameter-end]

Das Kommentieren kann gesperrt werden, sodass keine neuen Kommentare oder Stimmen hinterlassen werden können, indem das readonly-Flag auf true gesetzt wird.

Kommentare können außerdem nicht bearbeitet oder gelöscht werden.

[code-example-start config = {readonly: true}; linesToHighlight = [6]; title = 'Den Kommentar-Thread schreibgeschützt machen'; code-example-end]

Dies kann ohne Code angepasst werden, auf der Widget-Anpassungsseite, für eine gesamte Domain oder Seite:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.prevent-new-replies'; alt='Einstellung zum Verhindern neuer Antworten auf der Widget-Anpassungsseite, die einen Thread für eine Domain oder Seite sperrt'; title='Den Kommentar-Thread schreibgeschützt machen' app-screenshot-end]

## Aktualisierung!

Ab November 2022 können Threads von Administratoren und Moderatoren **live** über das Drei-Punkte-Menü über dem Antwortbereich gesperrt oder entsperrt werden.

Damit werden neue Kommentare verhindert, während das Abstimmen weiterhin möglich ist und Benutzer ihre Kommentare bei Bedarf löschen können, während `readonly` dies nicht zulässt. 

Dies entspricht dem Feld `isClosed` in der `Page`-API.