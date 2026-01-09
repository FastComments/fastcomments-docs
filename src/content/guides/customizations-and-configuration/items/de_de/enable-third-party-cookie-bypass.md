[related-parameter-start name = 'enableThirdPartyCookieBypass'; type = 'boolean'; related-parameter-end]

Für die Authentifizierung ist FastComments darauf angewiesen, dass Cookies von Drittanbietern in Ihrem Browser aktiviert sind. Ohne diese müssen Nutzer immer ihre E-Mail-Adresse angeben, um zu kommentieren (es sei denn, das E-Mail-Eingabefeld ist ausgeblendet), und ihre Kommentare werden standardmäßig immer als nicht verifiziert angezeigt.

Um dies zu umgehen, können Sie die Umgehung für Cookies von Drittanbietern aktivieren. 

Wenn diese Einstellung aktiviert ist, wird ein kleines Popup angezeigt, das eine Meldung anzeigt, dass der Nutzer angemeldet wird. Dieses Popup erscheint, sobald der Nutzer mit dem Kommentar-Widget interagiert; zum Beispiel, wenn er einen Kommentar abgibt.

Dies kann im Code erfolgen, indem die Flagge **enableThirdPartyCookieBypass** auf true gesetzt wird:

[code-example-start config = {enableThirdPartyCookieBypass: true}; linesToHighlight = [6]; title = 'Enabling Third-Party Cookie Bypass'; code-example-end]

Alternativ können Sie dies auch über die Widget-Anpassungsoberfläche unter `Enable Third-Party Cookie Popup` einrichten:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.enable-third-party-cookie-bypass'; clickSelectors = ['.enable-third-party-cookie-bypass']; title='Enabling Third-Party Cookie Bypass' app-screenshot-end]