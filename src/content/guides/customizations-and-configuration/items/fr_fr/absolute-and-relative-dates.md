[related-parameter-start name = 'absoluteAndRelativeDates'; type = 'boolean'; related-parameter-end]

Par défaut, les dates relatives localisées sont utilisées. Par exemple, à côté d'un commentaire récemment publié, vous pouvez voir « il y a 11 minutes ».

Il peut être nécessaire ou souhaitable de conserver ce format de date relative, mais également d'afficher la date complète à côté, auquel cas vous devez définir ce paramètre sur true. 

[code-example-start config = {absoluteAndRelativeDates: true}; linesToHighlight = [6]; title = 'Use Both Absolute and Relative Dates'; code-example-end]

Cela peut être personnalisé sans code, sur la page de personnalisation du widget, sous Options avancées. Vous devrez d'abord activer les dates absolues pour voir cette option dans l'interface.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates', '.relative-and-absolute-dates']; selector = '.relative-and-absolute-dates'; alt='Options avancées sur la page de personnalisation du widget avec les dates absolues et le paramètre combiné de date relative activés'; title='Utiliser à la fois les dates absolues et relatives' app-screenshot-end]