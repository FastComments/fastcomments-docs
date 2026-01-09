[related-parameter-start name = 'useShowCommentsToggle'; type = 'boolean'; related-parameter-end]

Standardmäßig rendert FastComments gleichzeitig das Kommentareingabefeld und den Kommentar-Thread. Um etwas vertikalen Platz zu sparen,
versteckt es außerdem alle anderen erforderlichen Felder, bis mit dem Widget interagiert wird.

[app-screenshot-start width=700; url=`https://fastcomments.com/embed?config=%7B%22tenantId%22%3A%22L177BUDVvSe%22%2C%22useShowCommentsToggle%22%3A%22true%22%2C%22urlId%22%3A%22file%253A%252F%252F%252FC%253A%252FUsers%252Fwinrid%252FAppData%252FRoaming%252FJetBrains%252FWebStorm2021.2%252Fscratches%252Fscratch_295.html%22%2C%22url%22%3A%22file%253A%252F%252F%252FC%253A%252FUsers%252Fwinrid%252FAppData%252FRoaming%252FJetBrains%252FWebStorm2021.2%252Fscratches%252Fscratch_295.html%22%2C%22pageTitle%22%3A%22%22%2C%22instanceId%22%3A%220.1281898364813452.1655790389169%22%7D&wId=comment-ui-v2`; selector = '.fast-comments'; delay=2000; title='Click to Show Comments' app-screenshot-end]

Die Schaltfläche verwendet je nachdem, ob die Kommentare derzeit angezeigt werden oder nicht, unterschiedlichen übersetzten Text. Wenn die Kommentare ausgeblendet sind, verwendet sie `translations.SHOW_COMMENTS_BUTTON_TEXT`. Wenn die
Kommentare angezeigt werden, verwendet sie `translations.HIDE_COMMENTS_BUTTON_TEXT`. Die Übersetzungen können den Text `[count]` enthalten, der durch die lokalisierte Anzahl ersetzt wird.

[code-example-start config = {useShowCommentsToggle: true}; linesToHighlight = [6]; title = 'Click to Show or Hide Comments'; code-example-end]

Dies ist dafür vorgesehen, die Konfiguration `hideCommentsUnderCountTextFormat` zu ersetzen.

Die Anzahl wird zusammen mit dem Kommentar-Thread live aktualisiert. Die Schaltfläche wird nicht angezeigt, wenn keine Kommentare vorhanden sind.

Dies kann ohne Code aktiviert werden, indem eine Anpassungsregel erstellt und "Klicken, um Kommentare anzuzeigen" aktiviert wird:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments']; selector = '.click-to-show-comments'; title='Enable Click to Show Comments' app-screenshot-end]