[related-parameter-start name = 'enableWYSIWYG'; type = 'boolean'; related-parameter-end]

Par défaut, les fonctionnalités de formatage dans FastComments sont réalisées en ajoutant des balises visibles comme `<b></b>` autour de votre texte. Cliquer sur la barre d'outils
ou utiliser des raccourcis effectue cela pour vous. Cependant, certaines communautés peuvent souhaiter opter pour un formatage sans balises visibles. On appelle cela l'activation de
l'éditeur WYSIWYG (ce que vous voyez est ce que vous obtenez). Cet éditeur a exactement le même aspect que celui par défaut, sauf qu'il charge du
code supplémentaire qui permet aux utilisateurs de mettre le texte en gras, de le souligner, etc. sans balises visibles.

[code-example-start config = {enableWYSIWYG: true}; linesToHighlight = [6]; title = 'Enabling WYSIWYG Editing'; code-example-end]

Ceci peut aussi être fait sans code. Dans la page de personnalisation du widget, consultez l'option « Activer le formatage avancé ».

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-wysiwyg']; selector = '.enable-search-label'; title='Enable WYSIWYG' app-screenshot-end]