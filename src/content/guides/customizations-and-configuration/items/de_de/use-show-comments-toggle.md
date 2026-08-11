[related-parameter-start name = 'useShowCommentsToggle'; type = 'boolean'; related-parameter-end]

Standardmäßig rendert FastComments das Kommentar‑Eingabefeld und den Kommentar‑Thread gleichzeitig. Um etwas vertikalen Platz zu sparen, blendet es außerdem alle anderen erforderlichen Felder aus, bis mit dem Widget interagiert wird.

Das Kommentar‑Widget kann jedoch hinter einer Schaltfläche versteckt werden, zum Beispiel:

[app-screenshot-start width=700; url=`https://fastcomments.com/embed?config=%7B%22tenantId%22%3A%22L177BUDVvSe%22%2C%22useShowCommentsToggle%22%3A%22true%22%2C%22urlId%22%3A%22file%253A%252F%252F%252FC%253A%252FUsers%252Fwinrid%252FAppData%252FRoaming%252FJetBrains%252FWebStorm2021.2%252Fscratches%252Fscratch_295.html%22%2C%22url%22%3A%22file%253A%252F%252F%252FC%253A%252FUsers%252Fwinrid%252FAppData%252FRoaming%252FJetBrains%252FWebStorm2021.2%252Fscratches%252Fscratch_295.html%22%2C%22pageTitle%22%3A%22%22%2C%22instanceId%22%3A%220.1281898364813452.1655790389169%22%7D&wId=comment-ui-v2`; selector = '.fast-comments'; delay=2000; alt='Kommentar-Widget, das hinter einer Schaltfläche eingeklappt ist und die Kommentaranzahl anzeigt, bis ein Leser darauf klickt.'; title='Klicken, um Kommentare anzuzeigen' app-screenshot-end]

Die Schaltfläche verwendet je nach aktuellem Anzeigezustand der Kommentare unterschiedliche übersetzte Texte. Wenn die Kommentare ausgeblendet sind, wird `translations.SHOW_COMMENTS_BUTTON_TEXT` verwendet. Wenn die Kommentare angezeigt werden, wird `translations.HIDE_COMMENTS_BUTTON_TEXT` verwendet. Die Übersetzungen können den Text `[count]` enthalten, der durch die lokalisierte Anzahl ersetzt wird.

[code-example-start config = {useShowCommentsToggle: true}; linesToHighlight = [6]; title = 'Click to Show or Hide Comments'; code-example-end]

Dies ist dazu gedacht, die Konfiguration `hideCommentsUnderCountTextFormat` zu ersetzen.

Die Anzahl wird in Echtzeit mit dem Kommentar‑Thread aktualisiert. Die Schaltfläche wird nicht angezeigt, wenn keine Kommentare vorhanden sind.

Dies kann ohne Code aktiviert werden, indem eine Anpassungsregel erstellt und "Click to Show Comments" aktiviert wird:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments']; selector = '.click-to-show-comments'; alt='Klick zum Anzeigen von Kommentaren Checkbox aktiviert in einer Anpassungsregel auf der Widget-Anpassungsseite'; title='Klick zum Anzeigen von Kommentaren aktivieren' app-screenshot-end]