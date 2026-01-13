[related-parameter-start name = 'readonly'; type = 'boolean'; related-parameter-end]

Das Kommentieren kann gesperrt werden, sodass durch Setzen des readonly-Flags auf true keine neuen Kommentare oder Stimmen abgegeben werden können.

Kommentare können dann außerdem nicht bearbeitet oder gelöscht werden.

[code-example-start config = {readonly: true}; linesToHighlight = [6]; title = 'Making The Comment Thread Readonly'; code-example-end]

Dies kann ohne Code auf der Seite zur Widget-Anpassung für eine gesamte Domain oder eine einzelne Seite angepasst werden:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.prevent-new-replies'; title='Making The Comment Thread Readonly' app-screenshot-end]

## Aktualisierung!

Seit November 2022 können Threads von Administratoren und Moderatoren über das Drei-Punkte-Menü oberhalb des Antwortbereichs **in Echtzeit** gesperrt oder entsperrt werden.

Dies verhindert neue Kommentare, erlaubt jedoch weiterhin Abstimmungen und ermöglicht es Benutzern, ihre Kommentare bei Bedarf zu löschen, während `readonly` diese Dinge nicht zulässt. 

Dies entspricht dem Feld `isClosed` in der `Page`-API.