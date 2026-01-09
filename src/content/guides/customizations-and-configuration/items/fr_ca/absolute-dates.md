[related-parameter-start name = 'absoluteDates'; type = 'boolean'; related-parameter-end]

Par défaut, des dates relatives localisées sont utilisées. Par exemple, à côté d'un commentaire récemment laissé, vous pouvez voir "il y a 11 minutes".

Il peut être nécessaire ou souhaitable d'utiliser des dates absolues, dans ce cas, définissez ce paramètre sur true. 

[code-example-start config = {absoluteDates: true}; linesToHighlight = [6]; title = 'Use Absolute Dates'; code-example-end]

Ceci peut être personnalisé sans code, sur la page de personnalisation du widget, sous Advanced Options :

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates']; selector = '.absolute-dates'; title='Use Absolute Dates' app-screenshot-end]