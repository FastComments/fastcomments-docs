[related-parameter-start name = 'enableViewCounts'; type = 'boolean'; related-parameter-end]

Par défaut, FastComments ne suit pas qui a consulté chaque commentaire ni ne fournit de statistiques à ce sujet.

Cependant, nous pouvons activer cette fonctionnalité, et le système commencera alors à suivre lorsque chaque utilisateur fait défiler jusqu'à un commentaire.

Lorsque cela se produit, un compteur à côté d'une icône en forme d'œil affichée sur chaque commentaire sera incrémenté. Le compteur est mis à jour en direct et abrégé selon la locale de l'utilisateur.

Nous pouvons activer cela en définissant le drapeau **enableViewCounts** sur true :

[code-example-start config = {enableViewCounts: true}; linesToHighlight = [6]; title = 'Enabling Comment View Counts'; code-example-end]

Cela peut être personnalisé sans code, sur la page de personnalisation du widget :

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-view-counts']; selector = '.enable-view-counts'; title='Enabling Comment View Counts' app-screenshot-end]

Nous suivons l'identifiant de l'utilisateur* qui a consulté le commentaire, de sorte que si vous consultez de nouveau le commentaire, le compteur ne s'incrémente pas. Si vous consultez le commentaire de nouveau
après deux ans, le compteur s'incrémentera de nouveau.

- *Remarque : ou l'identifiant de session anonyme, ou l'adresse IP de l'utilisateur sous forme hachée.