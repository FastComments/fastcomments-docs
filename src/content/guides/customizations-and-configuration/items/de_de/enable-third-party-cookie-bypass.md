[related-parameter-start name = 'enableThirdPartyCookieBypass'; type = 'boolean'; related-parameter-end]

Für die Authentifizierung ist FastComments darauf angewiesen, dass Drittanbieter‑Cookies in Ihrem Browser aktiviert sind. Ohne diese müssen Benutzer immer ihre E‑Mail-Adresse eingeben, um zu kommentieren (es sei denn, das E‑Mail‑Eingabefeld ist ausgeblendet), und ihre Kommentare werden standardmäßig immer als nicht verifiziert angezeigt.

Um dies zu umgehen, können Sie die Drittanbieter‑Cookie‑Umgehung aktivieren. 

Wenn diese Einstellung aktiviert ist, wird ein kleines Popup angezeigt, das eine Meldung zeigt, dass der Benutzer eingeloggt wird. Dieses Popup erscheint jedes Mal, wenn der Benutzer mit dem Kommentar‑Widget interagiert; zum Beispiel, wenn er einen Kommentar hinterlässt.

Wir können dies im Code tun, indem wir das Flag **enableThirdPartyCookieBypass** auf true setzen:

[code-example-start config = {enableThirdPartyCookieBypass: true}; linesToHighlight = [6]; title = 'Enabling Third-Party Cookie Bypass'; code-example-end]

Wir können dies auch über die Widget‑Anpassungs‑UI einrichten, unter `Enable Third-Party Cookie Popup`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.enable-third-party-cookie-bypass'; clickSelectors = ['.enable-third-party-cookie-bypass']; alt='Widget‑Anpassungsseite mit dem aktivierten Kontrollkästchen „Enable Third-Party Cookie Popup“'; title='Aktivieren der Drittanbieter‑Cookie‑Umgehung' app-screenshot-end]