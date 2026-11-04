FastComments SSO (<a href="#sso">détails ici</a>) offre à vos utilisateurs la possibilité de commenter sans avoir à se connecter à une autre plateforme.

Cependant, cela seul ne sécurise pas vos fils de commentaires, car par défaut les données des commentaires sont des informations publiquement accessibles - toute personne pouvant consulter la page peut voir les commentaires.

En modifiant un paramètre, nous pouvons restreindre la récupération des commentaires, sauf par un administrateur ou un utilisateur SSO valide.

#### Configuration sans code

Nous pouvons empêcher la consultation et l'interaction avec nos fils de commentaires, lorsque SSO est configuré, en créant une <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">règle de personnalisation</a>.

Ce faisant, recherchez SSO, et vous trouverez cette option :

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.require-sso'; selector = '.require-sso'; alt='Option Exiger SSO pour voir les commentaires activée dans une règle de personnalisation, avec le choix du niveau de sécurité'; title='Exiger SSO Pour Voir Les Commentaires' app-screenshot-end]

Activez-la et enregistrez la règle de personnalisation.

#### Protéger uniquement un certain domaine ou une certaine page

Pour protéger uniquement un certain domaine ou une certaine page, nous allons simplement configurer la règle de personnalisation en conséquence.

En haut de l'interface de personnalisation, nous trouverons deux champs, Domain et URL ID.

Pour protéger uniquement un domaine particulier, saisissez le domaine en question dans le champ « domain ».

Pour protéger une page particulière, saisissez une URL de page dans le champ « URL ID ». Si vous avez une intégration personnalisée avec FastComments, vous pouvez saisir ici un type d'identifiant au lieu d'une URL.

#### Niveaux de sécurité

Lorsque vous exigez SSO, vous devrez décider si vous exigez Simple SSO ou Secure SSO. Si vous exigez Simple SSO, les deux sont autorisés, mais si vous exigez Secure SSO, alors
le contenu doit être récupéré avec un payload Secure SSO haché avec votre clé API pour pouvoir être consulté.

L'option de niveau de sécurité apparaîtra lorsque vous sélectionnerez « Exiger SSO Pour Voir Les Commentaires », et ne prend effet que lorsque cette option est activée.

Les paramètres <a href="#sso-simple">Vérifier automatiquement les commentaires via Simple SSO et Autoriser les administrateurs Simple SSO</a> sont distincts, et s'appliquent que cette option soit activée ou non.

#### Protection au-delà de la lecture

L'activation de cette option protégera la page ou le domaine contre les commentaires, sauf si l'utilisateur est connecté via SSO.

#### Pièges à connaître

Les utilisateurs ayant créé des commentaires avant votre intégration SSO ne pourront pas les voir, à moins qu'ils ne se connectent via votre intégration SSO.