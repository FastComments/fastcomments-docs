---
[related-parameter-start name = 'startingPage'; type = 'number'; related-parameter-end]

Lors de la récupération et du rendu des commentaires, le widget de commentaires doit savoir sur quelle page commencer. Par défaut, il commence par
la première page et n'affiche que cette page.

Si nécessaire, la page exacte à afficher peut être transmise au widget de commentaires via le paramètre *startingPage*.

[code-example-start config = {startingPage: 1}; linesToHighlight = [6]; title = 'Specifying The Page to Render'; code-example-end]

Notez que la numérotation des pages commence à zéro, donc l'exemple ci-dessus affiche la deuxième page.

---