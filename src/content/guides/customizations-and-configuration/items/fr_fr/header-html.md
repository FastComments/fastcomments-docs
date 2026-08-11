[related-parameter-start name = 'headerHTML'; type = 'string'; related-parameter-end]

Du texte, comme un en-tête ou un message, peut être affiché sous le compteur de commentaires mais au-dessus du texte d'état de connexion.

Nous appelons cela l'en-tête, et par défaut il est masqué.

[code-example-start config = {headerHTML: "<h1>Leave a Comment!</h1>"}; linesToHighlight = [6]; title = 'Specifying Header HTML'; code-example-end]

Cela peut être personnalisé sans code, sur la page de personnalisation du widget, sous Options avancées :

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.show-advanced-option'; selector = '.absolute-dates'; alt='Zone des Options avancées de la page de personnalisation du widget où le HTML d\'en-tête personnalisé est saisi'; title='Spécifier le HTML d\'en-tête' app-screenshot-end]