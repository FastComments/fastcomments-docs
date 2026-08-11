---
[related-parameter-start name = 'showBadgesInTopBar'; type = 'boolean'; related-parameter-end]

Standardmäßig zeigt FastComments Benutzer‑Badges nur in deren Kommentaren innerhalb des Kommentar‑Threads an.

Wir können jedoch Benutzer‑Badges neben ihrem Namen über dem Kommentarformular anzeigen, indem wir diese Funktion auf der Seite zur Widget‑Anpassung aktivieren:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.show-badges-in-top-bar'; alt='Kontrollkästchen zum Anzeigen von Badges in der oberen Leiste auf der Widget-Anpassungsseite, das Badges neben dem Namen über dem Kommentarformular platziert'; title='Option zum Anzeigen von Badges in der oberen Leiste' app-screenshot-end]

Damit werden die Badges des Benutzers neben seinem Namen im oberen Leistenbereich angezeigt, wodurch seine Erfolge und sein Status beim Verfassen eines Kommentars stärker hervorgehoben werden.

Beachten Sie, dass diese Funktion in der UI zur Widget‑Anpassung aktiviert sein muss, um zu funktionieren. Sie können optional das Flag **showBadgesInTopBar** in Ihrer Code‑Konfiguration auf false setzen, um es selektiv zu deaktivieren, selbst wenn es auf Serverebene aktiviert ist:

[code-example-start config = {showBadgesInTopBar: false}; linesToHighlight = [6]; title = 'Disable Show Badges in Top Bar'; code-example-end]
---