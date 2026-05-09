[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

Standaard toont FastComments geen lijst met gebruikers op de pagina.

Je kunt een lijst weergeven van personen die momenteel de pagina bekijken, naast de commentaarwidget. De lijst wordt live bijgewerkt zodra gebruikers komen en gaan, en toont hun naam, avatar en een online-indicator.

Er zijn drie lay-outopties:

- `1` - Top: een horizontale rij overlappende avatars die boven de commentaarwidget worden weergegeven.
- `2` - Left: een zijbalk met namen en online puntjes die links van de widget worden weergegeven.
- `3` - Right: dezelfde zijbalk die rechts van de widget wordt weergegeven.

Stel de **usersListLocation** vlag in om de functie in te schakelen:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Show Users List on the Right'; code-example-end]

Standaard toont de lijst alleen gebruikers die momenteel online zijn. Om ook personen op te nemen die in het verleden op de pagina hebben gereageerd (maar de pagina nu niet bekijken), stel **usersListIncludeOffline** in op true:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Include Past Commenters'; code-example-end]

Voormalige reageerders worden weergegeven zonder de groene onlinestip zodat duidelijk is wie er nu aanwezig is.

Gebruikers met privéprofielen worden weergegeven met een generieke avatar en een label "Private Profile" zodat het aantal nauwkeurig blijft zonder identiteiten prijs te geven.

Dit kan ook zonder code worden geconfigureerd. Op de pagina voor het aanpassen van de widget, zie de optie "Users List Location". Wanneer de locatie is ingesteld op iets anders dan Off, verschijnt daaronder een selectievakje "Include past commenters".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-settings'; title='Users List Settings'; actions=[{type: 'set-value', selector: '#users-list-location-input', value: '3'}] app-screenshot-end]

Bij meer dan 500 live gebruikers kan de lijst tot 30 seconden achterlopen.