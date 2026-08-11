[related-parameter-start name = 'voteStyle'; type = 'number'; related-parameter-end]

Par défaut, FastComments affichera les options de vote sous forme de flèches haut et bas, permettant aux utilisateurs de voter positivement ou négativement un commentaire.

Cependant, il est possible de modifier le style de la barre d'outils de vote. Les options actuelles sont les boutons par défaut Haut/Bas, ou d'utiliser un mécanisme de vote de type cœur.

Nous utilisons le drapeau **voteStyle** comme suit :

[code-example-start config = {voteStyle: 1}; linesToHighlight = [6]; title = 'Activer le bouton cœur'; code-example-end]

Nous vous recommandons fortement de le faire sans code, car cela active également les validations côté serveur. Sur la page de personnalisation du widget, consultez la section « Style de vote ».

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.vote-style'; selector = '.vote-style'; alt='Paramètre du style de vote sur la page de personnalisation du widget, offrant des flèches haut et bas ou un vote cœur'; title='Modifier le style de vote' app-screenshot-end]

Le vote peut également être désactivé, voir `Disable Voting` au-dessus des options de style.