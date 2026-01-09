[related-parameter-start name = 'absoluteDates'; type = 'boolean'; related-parameter-end]

Par défaut, des dates relatives localisées sont utilisées. Par exemple, à côté d'un commentaire laissé récemment vous pouvez voir "il y a 11 minutes".

Il peut être nécessaire ou souhaitable d'utiliser des dates absolues, auquel cas vous définissez ce paramètre sur true. 

[code-example-start config = {absoluteDates: true}; linesToHighlight = [6]; title = 'Utiliser des dates absolues'; code-example-end]

Cela peut être personnalisé sans code, sur la page de personnalisation du widget, dans Options avancées :

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates']; selector = '.absolute-dates'; title='Utiliser des dates absolues' app-screenshot-end]