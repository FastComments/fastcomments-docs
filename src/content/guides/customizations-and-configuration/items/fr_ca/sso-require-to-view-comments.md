FastComments SSO (<a href="#sso">détails ici</a>) fournit à vos utilisateurs un moyen de commenter sans avoir à se connecter à une autre plateforme.

Cependant, cela seul ne sécurise pas vos fils de commentaires, car par défaut les données des commentaires sont des informations publiques - toute personne pouvant voir la page peut voir les commentaires.

En modifiant un paramètre, nous pouvons restreindre la récupération des commentaires sauf s'il s'agit d'un administrateur ou d'un utilisateur SSO valide.

#### Configuration sans code

Nous pouvons empêcher la consultation et l'interaction avec nos fils de commentaires, lorsque le SSO est configuré, en créant une <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">règle de personnalisation</a>.

En procédant ainsi, recherchez SSO, et vous trouverez cette option :

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.require-sso'; selector = '.require-sso'; title='Require SSO To View Comments' app-screenshot-end]

Activez-la et enregistrez la règle de personnalisation.

#### Protéger uniquement un domaine ou une page

Pour ne protéger qu'un domaine ou une page particulière, il suffit de configurer la règle de personnalisation en conséquence.

En haut de l'interface de personnalisation, vous trouverez deux champs : Domaine et ID d'URL.

Pour protéger simplement un domaine particulier, saisissez le domaine en question dans le champ "Domaine".

Pour protéger une page spécifique, saisissez l'URL de la page dans le champ "ID d'URL". Si vous avez une intégration personnalisée avec FastComments, vous pouvez saisir ici un type d'ID au lieu d'une URL.

#### Niveaux de sécurité

Lorsque vous exigez le SSO, vous devrez décider si vous exigez Simple SSO ou Secure SSO. Si vous exigez Simple SSO, les deux sont autorisés ; mais si vous exigez Secure SSO, le contenu doit être récupéré avec une charge utile Secure SSO hachée avec votre clé API afin de pouvoir être affiché.

L'option de niveau de sécurité apparaîtra lorsque vous sélectionnerez "Require SSO To View Comments".

#### Protection au-delà de la consultation

L'activation de cette option empêchera les commentaires sur la page ou le domaine, sauf si l'utilisateur est connecté via SSO.

#### Remarques

Les utilisateurs qui ont créé des commentaires avant votre intégration SSO ne pourront pas les voir, sauf s'ils se connectent via votre intégration SSO.