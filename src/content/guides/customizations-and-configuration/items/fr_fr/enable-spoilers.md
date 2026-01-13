---
[related-parameter-start name = 'enableSpoilers'; type = 'boolean'; related-parameter-end]

Nous pouvons activer la prise en charge des spoilers en définissant le drapeau **enableSpoilers** sur true:

[code-example-start config = {enableSpoilers: true}; linesToHighlight = [6]; title = 'Enabling Spoilers'; code-example-end]

Cela peut également être fait sans code. Dans la page de personnalisation du widget, consultez l'option "Activer les spoilers".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-spoilers']; selector = '.enable-spoilers'; title='Enable Spoilers' app-screenshot-end]

Lorsqu'un texte est surligné et que l'on clique sur le bouton `SPOILER` désormais visible, le texte est masqué jusqu'à ce que l'utilisateur le survole avec la souris. Pour le mode sombre, nous procédons de la même manière, en utilisant des couleurs différentes mieux adaptées.

Ceci est également compatible avec l'éditeur WYSIWYG.

---