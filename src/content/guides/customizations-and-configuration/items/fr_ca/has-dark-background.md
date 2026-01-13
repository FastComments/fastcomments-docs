---
[related-parameter-start name = 'hasDarkBackground'; type = 'boolean'; related-parameter-end]

Par défaut, le widget de commentaires FastComments détecte automatiquement le mode sombre sur la plupart des sites.

Lorsque le mode sombre est détecté, FastComments passera du texte noir sur fond blanc au texte blanc sur fond noir. Les images changeront aussi.

Au chargement de la page, le widget tentera de déterminer l'obscurité de l'arrière-plan de la page derrière le widget de commentaires. Cela signifie que
la page peut avoir un arrière-plan blanc, mais si vous placez le widget de commentaires à l'intérieur d'un conteneur avec un fond noir, le mode sombre devrait
toujours s'activer automatiquement pour rendre les commentaires lisibles.

Cependant, le mécanisme de détection, qui s'appuie sur la détermination de "luminance", peut ne pas activer le mode sombre lorsque vous le souhaitez. Pour l'activer de force, réglez le
*drapeau* *hasDarkBackground* sur true comme suit :

[code-example-start config = {hasDarkBackground: true}; linesToHighlight = [6]; title = 'Force Dark Background Mode'; additionalDemoCode = '<style>body { background: black; }</style>'; code-example-end]

---