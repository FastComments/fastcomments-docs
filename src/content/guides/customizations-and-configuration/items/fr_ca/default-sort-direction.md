[related-parameter-start name = 'defaultSortDirection'; type = 'string'; related-parameter-end]

Par défaut, FastComments triera les commentaires selon la direction de tri "Le plus pertinent".

Le tri "Le plus pertinent" tient compte du moment où le commentaire a été laissé et du nombre de votes pour le tri.

L'utilisateur peut ensuite modifier la direction de tri vers "Plus anciens" ou "Plus récents" dans l'interface du widget de commentaires.

Cependant, nous pouvons définir la valeur par défaut sur l'une des trois options. Par exemple, si vous souhaitez afficher les commentaires les plus anciens en premier :

[code-example-start config = {defaultSortDirection: "OF"}; linesToHighlight = [6]; title = 'Changing The Default Sort To Oldest First'; code-example-end]

Nous définissons la valeur de **defaultSortDirection** sur "OF" pour définir la direction sur "OF".

Pour le sens de tri « Plus récents en premier », nous ferions ce qui suit :

[code-example-start config = {defaultSortDirection: "NF"}; linesToHighlight = [6]; title = 'Changing The Default Sort To Newest First'; code-example-end]

Les valeurs valides pour **defaultSortDirection** sont:

- MR: « Le plus récent »
- NF: « Plus récents en premier »
- OF: « Plus anciens en premier »

Ceci peut également être fait sans code. Sur la page de personnalisation du widget, consultez la section « Direction de tri par défaut ».

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-sort-direction'; title='Changing The Default Sort Direction' app-screenshot-end]

Notez que les commentaires sur chaque page pour chaque sens de tri sont pré-calculés, donc tous les sens de tri ont les mêmes performances.