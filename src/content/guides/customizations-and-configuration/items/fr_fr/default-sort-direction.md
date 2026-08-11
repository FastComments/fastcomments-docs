[related-parameter-start name = 'defaultSortDirection'; type = 'string'; related-parameter-end]

Par défaut, FastComments trie les commentaires selon la direction de tri « Most Relevant ».

Le tri « Most Relevant » prend en compte le moment où le commentaire a été laissé ainsi que le nombre de votes pour le classement.

L'utilisateur peut ensuite changer la direction de tri vers « Oldest » ou « Newest First » dans l'interface du widget de commentaires.

Cependant, nous pouvons modifier la valeur par défaut pour l'une des trois options. Par exemple, si vous souhaitez afficher les commentaires les plus anciens en premier :

[code-example-start config = {defaultSortDirection: "OF"}; linesToHighlight = [6]; title = 'Changer le tri par défaut vers le plus ancien d\'abord'; code-example-end]

Nous définissons la valeur de **defaultSortDirection** à « OF » pour définir la direction sur « OF ».

Pour la direction de tri du plus récent en premier, nous ferions ce qui suit :

[code-example-start config = {defaultSortDirection: "NF"}; linesToHighlight = [6]; title = 'Changer le tri par défaut vers le plus récent d\'abord'; code-example-end]

Les valeurs valides pour **defaultSortDirection** sont :

- MR: "Most Recent"
- NF: "Newest First"
- OF: "Oldest First"

Cela peut également être fait sans code. Dans la page de personnalisation du widget, voir la section « Default Sort Direction ».

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-sort-direction'; alt='Sélecteur de direction de tri par défaut offrant le plus pertinent, le plus récent d\'abord et le plus ancien d\'abord'; title='Modification de la direction de tri par défaut'; app-screenshot-end]

Notez que les commentaires sur chaque page pour chaque direction de tri sont pré‑calculés, de sorte que toutes les directions de tri ont les mêmes performances.