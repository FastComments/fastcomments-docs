[related-parameter-start name = 'showLiveRightAway'; type = 'boolean'; related-parameter-end]

Standaard is live reageren ingeschakeld. Dit betekent dat als er reacties worden toegevoegd, verwijderd, bewerkt of vastgezet, de wijzigingen voor alle gebruikers die de discussiedraad bekijken tegelijk zichtbaar moeten zijn.

Echter, standaard zullen die nieuwe reacties verschijnen onder een dynamisch weergegeven knop met tekst die lijkt op "Show 2 New Comments".

Als de nieuwe reacties direct op de pagina reageren, wordt de knop bovenaan de discussiedraad weergegeven. Als ze een reactie op een specifieke reactie zijn, wordt de knop onder die reactie weergegeven.

Dit is om te voorkomen dat de paginagrootte voortdurend verandert voor de gebruiker, wat mogelijk frustratie veroorzaakt bij het proberen vast te pakken van de scrollbalk.

Voor sommige gebruikssituaties, zoals live bieden of online evenementen, is dit niet het gewenste gedrag – je wilt misschien dat de reactiewidget meer lijkt op een "chat"-venster waarbij nieuwe reacties "direct worden getoond".

Daarom de naam van de vlag die die functie inschakelt: **showLiveRightAway**.

We kunnen deze als volgt inschakelen:

[code-example-start config = {showLiveRightAway: true}; linesToHighlight = [6]; title = 'Toon Live Reacties Direct'; code-example-end]

Dit kan zonder code worden aangepast op de widget-aanpassingspagina:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.collapse-live-comments'; selector = '.collapse-live-comments'; alt='Instelling voor het inklappen van live reacties ingeschakeld zodat nieuwe reacties direct verschijnen in plaats van achter een knop'; title='Toon Live Reacties Direct' app-screenshot-end]