[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

Standaard toont FastComments geen lijst met gebruikers op de pagina.

Je kunt een lijst weergeven van mensen die de pagina momenteel bekijken, naast de commentaarwidget. De lijst wordt live bijgewerkt wanneer gebruikers komen en gaan, en toont hun naam, avatar en een online-indicator.

Er zijn drie lay-outopties:

- `1` - Boven: een horizontale rij van overlappende avatars die boven de commentaarwidget worden weergegeven.
- `2` - Links: een zijbalk met namen en online-indicatoren die links van de widget worden weergegeven.
- `3` - Rechts: dezelfde zijbalk die rechts van de widget wordt weergegeven.

Set the **usersListLocation** flag to enable the feature:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Show Users List on the Right'; code-example-end]

Standaard toont de lijst alleen gebruikers die momenteel online zijn. Om ook mensen op te nemen die in het verleden op de pagina hebben gereageerd (maar de pagina niet actief bekijken), zet **usersListIncludeOffline** op true:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Include Past Commenters'; code-example-end]

Gebruikers die in het verleden hebben gereageerd worden weergegeven zonder de groene online-indicator, zodat duidelijk is wie er nu aanwezig is.

Gebruikers met privéprofielen worden weergegeven met een generieke avatar en een label "Privéprofiel", zodat het aantal nauwkeurig blijft zonder identiteiten te onthullen.

Dit kan ook worden geconfigureerd zonder code. Op de pagina voor het aanpassen van de widget, zie de optie "Users List Location":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-location'; title='Users List Location' app-screenshot-end]

Wanneer de locatie is ingesteld op iets anders dan Off, wordt het selectievakje "Include Past Commenters" eronder weergegeven:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-include-offline'; title='Include Past Commenters' app-screenshot-end]

---