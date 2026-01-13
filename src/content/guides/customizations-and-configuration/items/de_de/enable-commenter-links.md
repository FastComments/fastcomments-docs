[related-parameter-start name = 'enableCommenterLinks'; type = 'boolean'; related-parameter-end]

Standardmäßig fragt FastComments den Benutzer nur nach seinem Kommentar, seinem Benutzernamen und seiner E-Mail-Adresse.

In einigen Situationen möchten Sie jedoch, dass der Benutzer einen Link zu seinem eigenen Blog oder seiner Website hinterlässt.

Wir können das Anzeigen eines zusätzlichen Eingabefeldes zur Angabe der Website-URL des Benutzers aktivieren, indem wir das Flag **enableCommenterLinks** auf true setzen:

[code-example-start config = {enableCommenterLinks: true}; linesToHighlight = [6]; title = 'Enabling Commenter Links'; code-example-end]

Wenn diese URL angegeben wird, wird das Benutzerkonto des Nutzers aktualisiert und der Benutzername in allen bisherigen und zukünftigen Kommentaren mit dieser URL verlinkt.

Dies lässt sich ohne Code auf der Seite zur Anpassung des Widgets konfigurieren:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments', '.commenter-links']; selector = '.commenter-links'; title='Enabling Commenter Links' app-screenshot-end]

---