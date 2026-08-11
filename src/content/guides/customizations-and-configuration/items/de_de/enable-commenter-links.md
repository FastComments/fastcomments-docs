[related-parameter-start name = 'enableCommenterLinks'; type = 'boolean'; related-parameter-end]

Standardmäßig fragt FastComments den Benutzer nur nach seinem Kommentar, seinem Benutzernamen und seiner E‑Mail-Adresse.

In einigen Situationen möchten Sie jedoch, dass der Benutzer einen Link zu seinem eigenen Blog oder seiner eigenen Website hinterlässt.

Wir können das Anzeigen eines zusätzlichen Eingabefeldes für die Website-URL des Benutzers aktivieren, indem wir das **enableCommenterLinks**‑Flag auf true setzen:

[code-example-start config = {enableCommenterLinks: true}; linesToHighlight = [6]; title = 'Aktivieren von Kommentator-Links'; code-example-end]

Wenn diese URL angegeben wird, wird das Konto des Benutzers aktualisiert und sein Benutzername in allen vergangenen und zukünftigen Kommentaren wird auf diese URL verlinken.

Dies kann ohne Code auf der Widget-Anpassungsseite angepasst werden:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments', '.commenter-links']; selector = '.commenter-links'; alt='Widget-Anpassungsseite mit aktiviertem Kontrollkästchen für Kommentator-Links, um ein Feld für die Website-URL zum Kommentarformular hinzuzufügen'; title='Aktivieren von Kommentator-Links' app-screenshot-end]