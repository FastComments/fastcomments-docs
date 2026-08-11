[related-parameter-start name = 'enableSpoilers'; type = 'boolean'; related-parameter-end]

Nous pouvons activer la prise en charge des spoilers en définissant le drapeau **enableSpoilers** sur true :

[code-example-start config = {enableSpoilers: true}; linesToHighlight = [6]; title = 'Activation des spoilers'; code-example-end]

Cela peut également être fait sans code. Dans la page de personnalisation du widget, voyez l'option "Enable Spoilers".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-spoilers']; selector = '.enable-spoilers'; alt='Page de personnalisation du widget avec la case à cocher Enable Spoilers cochée pour ajouter le bouton SPOILER à l\'éditeur'; title='Activer les spoilers' app-screenshot-end]

Lorsque le texte est sélectionné et que le bouton `SPOILER` maintenant visible est cliqué, le texte sera masqué jusqu'à ce que l'utilisateur le survole avec la souris. Pour le mode sombre, nous faisons la même chose, avec des couleurs différentes qui correspondent mieux au mode sombre.

Ceci est également compatible avec l'éditeur WYSIWYG.