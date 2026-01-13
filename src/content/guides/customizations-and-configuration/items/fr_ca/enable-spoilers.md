[related-parameter-start name = 'enableSpoilers'; type = 'boolean'; related-parameter-end]

Nous pouvons activer la prise en charge des spoilers en définissant le drapeau **enableSpoilers** sur true :

[code-example-start config = {enableSpoilers: true}; linesToHighlight = [6]; title = 'Enabling Spoilers'; code-example-end]

Cela peut aussi être fait sans code. Dans la page de personnalisation du widget, consultez l'option "Activer les spoilers".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-spoilers']; selector = '.enable-spoilers'; title='Enable Spoilers' app-screenshot-end]

Lorsqu'un texte est sélectionné et que le bouton `SPOILER` désormais visible est cliqué, le texte sera masqué jusqu'à ce que l'utilisateur passe la souris dessus. Pour le mode sombre, nous faisons la même chose, avec des
couleurs qui s'harmonisent mieux avec le mode sombre.

Ceci est également compatible avec l'éditeur WYSIWYG.