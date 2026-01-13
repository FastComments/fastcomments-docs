[related-parameter-start name = 'showLiveRightAway'; type = 'boolean'; related-parameter-end]

Standaard zijn live reacties ingeschakeld. Dit betekent dat wanneer reacties worden toegevoegd, verwijderd, bewerkt of vastgezet, de wijzigingen moeten verschijnen
voor alle gebruikers die de reactiedraad op hetzelfde moment bekijken.

Echter, standaard verschijnen die nieuwe reacties onder een dynamisch weergegeven knop met een tekst zoals "Toon 2 nieuwe reacties".

Als de nieuwe reacties rechtstreeks op de pagina zijn gericht, verschijnt de knop bovenaan de reactiedraad. Als het reacties zijn op een specifieke reactie, 
verschijnt de knop onder die reactie.

Dit voorkomt dat de paginagrootte voortdurend verandert voor de gebruiker, wat tot frustratie kan leiden bij het proberen de schuifbalk vast te pakken.

Voor sommige gebruikssituaties, zoals live bieden of online evenementen, is dit niet het gewenste gedrag - u wilt misschien dat de reacties-widget
meer als een "chat"-vak is waarin nieuwe reacties "direct verschijnen".

Daarom heet de vlag die deze functie inschakelt: **showLiveRightAway**.

We kunnen het als volgt inschakelen:

[code-example-start config = {showLiveRightAway: true}; linesToHighlight = [6]; title = 'Show Live Comments Right Away'; code-example-end]

Dit kan zonder code worden aangepast op de pagina voor het aanpassen van de widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.collapse-live-comments'; selector = '.collapse-live-comments'; title='Show Live Comments Right Away' app-screenshot-end]

---