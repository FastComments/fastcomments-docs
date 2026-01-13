[related-parameter-start name = 'voteStyle'; type = 'number'; related-parameter-end]

Par défaut, FastComments affichera des options de vote sous forme de flèches haut et bas, permettant aux utilisateurs de voter pour ou contre un commentaire.

Cependant, il est possible de changer le style de la barre d'outils de vote. Les options actuelles sont les boutons par défaut Haut/Bas, ou d'utiliser un mécanisme de vote de type cœur.

Nous utilisons le paramètre **voteStyle** comme suit :

[code-example-start config = {voteStyle: 1}; linesToHighlight = [6]; title = 'Enable Heart Button'; code-example-end]

Nous vous recommandons fortement de le faire sans code, car cela active également des validations côté serveur. Dans la page de personnalisation du widget, consultez la section « Style de vote ».

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.vote-style'; selector = '.vote-style'; title='Change Voting Style' app-screenshot-end]

Le vote peut également être désactivé, voir `Disable Voting` au-dessus des options de style.

---