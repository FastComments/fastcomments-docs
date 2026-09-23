[related-parameter-start name = 'enableViewCounts'; type = 'boolean'; related-parameter-end]

Par défaut, FastComments ne suit pas qui a vu chaque commentaire et ne fournit aucune statistique à ce sujet.

Cependant, nous pouvons activer cette fonctionnalité, et le système commencera alors à suivre chaque fois qu'un utilisateur fait défiler un commentaire.

Lorsque cela se produit, un compteur à côté d'une icône œil affichée sur chaque commentaire sera incrémenté. Le compteur est mis à jour en temps réel et abrégé selon la locale de l'utilisateur.

Nous pouvons activer cela en définissant le drapeau **enableViewCounts** sur true :

[code-example-start config = {enableViewCounts: true}; linesToHighlight = [6]; title = 'Activation du comptage des vues de commentaires'; code-example-end]

Cela peut être personnalisé sans code, sur la page de personnalisation du widget :

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-view-counts']; selector = '.enable-view-counts'; alt='Page de personnalisation du widget avec la case à cocher des comptages de vues activée, de sorte que chaque commentaire affiche une icône œil et un compteur'; title='Activation du comptage des vues de commentaires' app-screenshot-end]

Nous suivons l'ID utilisateur* qui a vu le commentaire pendant une semaine, de sorte que si vous revoyez le commentaire dans cette semaine, il n'est pas incrémenté. Si vous revoyez le commentaire après la semaine écoulée, le compteur s'incrémentera de nouveau.

- *Note : ou l'ID de session anonyme, ou l'IP de l'utilisateur sous forme de valeur hachée.