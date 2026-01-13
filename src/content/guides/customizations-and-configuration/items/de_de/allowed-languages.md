---
Standardmäßig begrenzt FastComments die für Kommentare verwendeten Sprachen nicht. 

Es kann wünschenswert sein, die Sprachen einer Community einzuschränken.

Dies lässt sich ohne Programmieraufwand auf der Seite für die Anpassung des Widgets konfigurieren:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.allowed-languages']; selector = '.allowed-languages'; title='Allowed Languages' app-screenshot-end]

Das System analysiert den Kommentar, bestimmt dessen Sprache und vergleicht sie mit der Liste erlaubter Sprachen.

Wenn der Kommentar in einer nicht erlaubten Sprache verfasst ist, wird eine lokalisierte Fehlermeldung angezeigt. 

---