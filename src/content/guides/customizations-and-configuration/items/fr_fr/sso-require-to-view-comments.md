---
FastComments SSO (<a href="#sso">détails ici</a>) fournit à vos utilisateurs un moyen de commenter sans avoir à se connecter à une autre plateforme.

Cependant, cela seul ne sécurise pas vos fils de commentaires, car par défaut les données des commentaires sont des informations publiques - toute personne pouvant voir la page peut voir les commentaires.

En modifiant un réglage, nous pouvons restreindre la récupération des commentaires sauf si elle est effectuée par un administrateur ou un utilisateur SSO valide.

#### Configuration sans code

Nous pouvons empêcher l'affichage et l'interaction avec nos fils de commentaires, lorsque le SSO est configuré, en créant une <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">règle de personnalisation</a>.

Lors de cette opération, recherchez SSO, et vous trouverez cette option :

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.require-sso'; selector = '.require-sso'; title='Require SSO To View Comments' app-screenshot-end]

Activez-la et enregistrez la règle de personnalisation.

#### Protéger uniquement un domaine ou une page spécifique

Pour ne protéger qu'un certain Domain ou Page, nous configurerons simplement la règle de personnalisation pour le faire.

En haut de l'interface de personnalisation, vous trouverez deux champs, Domain et URL ID.

Pour protéger uniquement un domaine particulier, saisissez le domaine en question dans le champ "domain".

Pour protéger une page particulière, saisissez l'URL de la page dans le champ "URL ID". Si vous avez une intégration personnalisée avec FastComments, vous pouvez entrer un type d'ID ici au lieu d'une URL.

#### Niveaux de sécurité

Lorsque vous exigez le SSO, vous devrez décider si vous exigez SSO simple ou SSO sécurisé. Si vous exigez SSO simple, alors les deux sont autorisés, mais si vous exigez SSO sécurisé alors le contenu doit être récupéré avec une charge utile SSO sécurisé hachée avec votre clé API afin d'être affiché.

L'option de niveau de sécurité apparaîtra lorsque vous sélectionnez "Require SSO To View Comments".

#### Protection au-delà de la lecture

L'activation de cette option protégera la page ou le domaine contre les commentaires, sauf si l'utilisateur est connecté via SSO.

#### Pièges

Les utilisateurs qui ont créé des commentaires avant votre intégration SSO ne pourront pas les voir, à moins qu'ils ne se connectent via votre intégration SSO.

---