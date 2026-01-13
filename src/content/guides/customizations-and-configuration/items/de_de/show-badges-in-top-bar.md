[related-parameter-start name = 'showBadgesInTopBar'; type = 'boolean'; related-parameter-end]

Standardmäßig zeigt FastComments Benutzerauszeichnungen nur an ihren Kommentaren innerhalb des Kommentar-Threads an.

Allerdings können wir Benutzerauszeichnungen neben ihrem Namen oberhalb des Kommentarfelds anzeigen, indem wir diese Funktion auf der Seite zur Anpassung des Widgets aktivieren:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.show-badges-in-top-bar'; title='Show Badges in Top Bar Option' app-screenshot-end]

Dadurch werden die Abzeichen des Benutzers neben seinem Namen im oberen Bereich angezeigt, wodurch seine Erfolge und sein Status beim Verfassen eines Kommentars stärker hervorgehoben werden.

Hinweis: Diese Funktion muss in der UI zur Anpassung des Widgets aktiviert sein, damit sie funktioniert. Optional können Sie das Flag **showBadgesInTopBar** in Ihrer Code-Konfiguration auf false setzen, um es selektiv zu deaktivieren, selbst wenn es auf Serverebene eingeschaltet ist:

[code-example-start config = {showBadgesInTopBar: false}; linesToHighlight = [6]; title = 'Disable Show Badges in Top Bar'; code-example-end]