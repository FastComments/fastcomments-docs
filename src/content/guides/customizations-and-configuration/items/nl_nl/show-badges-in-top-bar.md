[related-parameter-start name = 'showBadgesInTopBar'; type = 'boolean'; related-parameter-end]

Standaard toont FastComments gebruikersbadges alleen bij hun opmerkingen binnen de discussiedraad.

We kunnen deze gebruikersbadges echter naast hun naam boven het reactieformulier tonen door deze functie in de widget-aanpassingspagina in te schakelen:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.show-badges-in-top-bar'; title='Show Badges in Top Bar Option' app-screenshot-end]

Dit toont de badges van de gebruiker naast hun naam in de bovenste balk, waardoor hun prestaties en status prominenter worden wanneer ze een reactie aan het schrijven zijn.

Houd er rekening mee dat deze functie ingeschakeld moet zijn in de widget-aanpassingsinterface om te werken. Je kunt optioneel de **showBadgesInTopBar**-vlag in je codeconfiguratie op false zetten om deze selectief uit te schakelen, zelfs wanneer deze op serverniveau is ingeschakeld:

[code-example-start config = {showBadgesInTopBar: false}; linesToHighlight = [6]; title = 'Disable Show Badges in Top Bar'; code-example-end]