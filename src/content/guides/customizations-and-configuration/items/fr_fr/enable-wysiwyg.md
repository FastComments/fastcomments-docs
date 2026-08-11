---
[related-parameter-start name = 'enableWYSIWYG'; type = 'boolean'; related-parameter-end]

Par défaut, les fonctionnalités de formatage dans FastComments sont réalisées en ajoutant des balises d'ancrage visibles comme `<b></b>` autour de votre texte. Cliquer sur la barre d'outils
ou utiliser des raccourcis le fait pour vous. Cependant, certaines communautés peuvent souhaiter activer le formatage sans balises d'ancrage. Cela s'appelle l'activation du
WYSIWYG (what you see is what you get) éditeur. Cet éditeur ressemble exactement à celui par défaut, sauf qu'il charge du
code supplémentaire qui permet aux utilisateurs de mettre du texte en gras, souligné, etc. sans balises d'ancrage visibles.

[code-example-start config = {enableWYSIWYG: true}; linesToHighlight = [6]; title = 'Activation de l\'édition WYSIWYG'; code-example-end]

Cela peut également être fait sans code. Dans la page de personnalisation du widget, voyez l'option "Enable Advanced Formatting".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-wysiwyg']; selector = '.enable-search-label'; alt='Page de personnalisation du widget avec la case à cocher Enable Advanced Formatting cochée pour activer l\'éditeur WYSIWYG'; title='Activer WYSIWYG' app-screenshot-end]

---