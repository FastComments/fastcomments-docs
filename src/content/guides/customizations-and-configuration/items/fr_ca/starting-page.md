[related-parameter-start name = 'startingPage'; type = 'number'; related-parameter-end]

Quand on récupère et affiche des commentaires, le widget de commentaires doit savoir à quelle page commencer. Par défaut, il commence par
la première page, n'affichant que cette page.

Si désiré, la page exacte à afficher peut être transmise au widget de commentaires via le réglage *startingPage*.

[code-example-start config = {startingPage: 1}; linesToHighlight = [6]; title = 'Specifying The Page to Render'; code-example-end]

Notez que les numéros de page commencent à zéro, donc l'exemple ci‑dessus affiche la deuxième page.