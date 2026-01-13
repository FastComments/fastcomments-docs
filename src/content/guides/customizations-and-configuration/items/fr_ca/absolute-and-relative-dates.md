---
[related-parameter-start name = 'absoluteAndRelativeDates'; type = 'boolean'; related-parameter-end]

Par défaut, les dates relatives localisées sont utilisées. Par exemple, à côté d'un commentaire laissé récemment, vous pouvez voir "il y a 11 minutes".

Il peut être nécessaire ou souhaitable de conserver ce format de date relative tout en affichant la date complète à côté; dans ce cas, réglez ce paramètre sur true. 

[code-example-start config = {absoluteAndRelativeDates: true}; linesToHighlight = [6]; title = 'Use Both Absolute and Relative Dates'; code-example-end]

Cela peut être personnalisé sans code, sur la page de personnalisation du widget, dans Options avancées. Vous devrez d'abord activer les Dates absolues pour voir cette option dans l'interface utilisateur.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates', '.relative-and-absolute-dates']; selector = '.relative-and-absolute-dates'; title='Use Both Absolute and Relative Dates' app-screenshot-end]

---