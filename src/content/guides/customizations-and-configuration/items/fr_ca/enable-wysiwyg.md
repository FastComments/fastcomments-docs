[related-parameter-start name = 'enableWYSIWYG'; type = 'boolean'; related-parameter-end]

Par défaut, les fonctionnalités de formatage dans FastComments sont réalisées en ajoutant des balises d'ancrage visibles comme `<b></b>` autour de votre texte. Cliquer sur la barre d'outils
ou utiliser des raccourcis fait cela pour vous. Cependant, certaines communautés peuvent souhaiter opter pour l'utilisation du formatage sans balises d'ancrage. On appelle cela l'activation de l'
WYSIWYG (ce que vous voyez est ce que vous obtenez) éditeur. Cet éditeur a exactement la même apparence que celui par défaut, sauf qu'il charge du
code supplémentaire qui permet aux utilisateurs de mettre le texte en gras, le souligner, etc., sans balises d'ancrage visibles.

[code-example-start config = {enableWYSIWYG: true}; linesToHighlight = [6]; title = 'Enabling WYSIWYG Editing'; code-example-end]

Cela peut aussi être fait sans code. Dans la page de personnalisation du widget, consultez l'option « Enable Advanced Formatting ».

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-wysiwyg']; selector = '.enable-search-label'; title='Enable WYSIWYG' app-screenshot-end]