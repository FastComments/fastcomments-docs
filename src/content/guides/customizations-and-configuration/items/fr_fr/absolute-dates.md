[related-parameter-start name = 'absoluteDates'; type = 'boolean'; related-parameter-end]

Par défaut, les dates relatives localisées sont utilisées. Par exemple, à côté d'un commentaire récemment publié, vous pouvez voir "11 minutes ago".

Il peut être nécessaire ou souhaitable d'utiliser des dates absolues, auquel cas vous devez définir ce paramètre sur true. 

[code-example-start config = {absoluteDates: true}; linesToHighlight = [6]; title = 'Utiliser les dates absolues'; code-example-end]

Cela peut être personnalisé sans code, sur la page de personnalisation du widget, sous Options avancées :

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates']; selector = '.absolute-dates'; alt='Options avancées sur la page de personnalisation du widget avec le commutateur de dates absolues activé'; title='Utiliser les dates absolues' app-screenshot-end]