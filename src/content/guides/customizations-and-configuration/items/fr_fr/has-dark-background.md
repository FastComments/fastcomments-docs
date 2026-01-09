---
[related-parameter-start name = 'hasDarkBackground'; type = 'boolean'; related-parameter-end]

Par défaut, le widget de commentaires FastComments détectera automatiquement le mode sombre sur la plupart des sites.

Lorsque le mode sombre est détecté, FastComments passera du texte noir sur fond blanc au texte blanc sur fond noir. Les images changeront également.

Au chargement de la page, le widget essaiera de déterminer la luminosité du fond de la page derrière le widget de commentaires. Cela signifie que la page peut avoir un fond blanc, mais si vous placez le widget de commentaires à l'intérieur d'un conteneur au fond noir, le mode sombre devrait toujours s'activer automatiquement afin de rendre les commentaires lisibles.

Cependant, le mécanisme de détection, qui repose sur la détermination de la « luminance », peut ne pas activer le mode sombre lorsque vous le souhaitez. Pour l'activer de force, définissez le drapeau *hasDarkBackground* sur true comme suit :

[code-example-start config = {hasDarkBackground: true}; linesToHighlight = [6]; title = 'Forcer le mode sombre'; additionalDemoCode = '<style>body { background: black; }</style>'; code-example-end]

---