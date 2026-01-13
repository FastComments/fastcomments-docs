[related-parameter-start name = 'pageTitle'; type = 'string'; related-parameter-end]

Le titre de la page actuelle est associé à l'**urlId** spécifié et est enregistré pour être utilisé dans les outils de modération.

Par défaut, il est récupéré depuis **document.title**.

Si vous le souhaitez, vous pouvez spécifier votre propre titre de page comme suit :

[code-example-start config = {pageTitle: "Article 42"}; linesToHighlight = [6]; title = 'Specifying The Page Title'; code-example-end]