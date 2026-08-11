[related-parameter-start name = 'showBadgesInTopBar'; type = 'boolean'; related-parameter-end]

Standaard toont FastComments gebruikersbadges alleen op hun reacties binnen de discussiedraad.

We kunnen echter gebruikersbadges naast hun naam boven het reactieformulier weergeven door deze functie in te schakelen op de widget-aanpassingspagina:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.show-badges-in-top-bar'; alt='Toon badges in de bovenbalk selectievakje op de widget-aanpassingspagina, waarbij badges naast de naam boven het reactieformulier worden geplaatst'; title='Optie om badges in de bovenbalk weer te geven' app-screenshot-end]

Dit zal de badges van de gebruiker naast hun naam in de bovenbalk weergeven, waardoor hun prestaties en status meer opvallen wanneer ze een reactie schrijven.

Let op dat deze functie ingeschakeld moet zijn in de widget-aanpassingsinterface om te werken. Je kunt optioneel de **showBadgesInTopBar**-vlag op false zetten in je codeconfiguratie om deze selectief uit te schakelen, zelfs wanneer deze op serverniveau is ingeschakeld:

[code-example-start config = {showBadgesInTopBar: false}; linesToHighlight = [6]; title = 'Disable Show Badges in Top Bar'; code-example-end]