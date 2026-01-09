[related-parameter-start name = 'defaultSortDirection'; type = 'string'; related-parameter-end]

Par défaut, FastComments triera les commentaires selon la direction de tri "Most Relevant".

Le tri "Most Relevant" prend en compte l'heure à laquelle le commentaire a été laissé et le nombre de votes.

L'utilisateur peut ensuite modifier la direction de tri vers Oldest ou Newest First dans l'interface du widget de commentaires.

Cependant, nous pouvons définir la valeur par défaut sur l'une des trois. Par exemple, si vous souhaitez afficher d'abord les commentaires les plus anciens :

[code-example-start config = {defaultSortDirection: "OF"}; linesToHighlight = [6]; title = 'Changing The Default Sort To Oldest First'; code-example-end]

Nous définissons la valeur de **defaultSortDirection** sur "OF" pour définir la direction sur "OF".

Pour la direction de tri Newest First, nous ferions ce qui suit :

[code-example-start config = {defaultSortDirection: "NF"}; linesToHighlight = [6]; title = 'Changing The Default Sort To Newest First'; code-example-end]

Les valeurs valides pour **defaultSortDirection** sont :

- MR: "Most Recent"
- NF: "Newest First"
- OF: "Oldest First"

Cela peut aussi être fait sans code. Sur la page de personnalisation du widget, consultez la section « Default Sort Direction ».

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-sort-direction'; title='Changing The Default Sort Direction' app-screenshot-end]

Notez que les commentaires sur chaque page pour chaque direction de tri sont pré-calculés, donc toutes les directions de tri ont les mêmes performances.