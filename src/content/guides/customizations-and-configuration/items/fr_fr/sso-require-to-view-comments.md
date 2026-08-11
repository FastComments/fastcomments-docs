---
FastComments SSO (<a href="#sso">détails ici</a>) offre à vos utilisateurs un moyen de commenter sans devoir se connecter à une autre plateforme.

Cependant, cela ne suffit pas à sécuriser vos fils de commentaires, car par défaut les données des commentaires sont des informations publiques – toute personne pouvant voir la page peut voir les commentaires.

En modifiant un paramètre, nous pouvons restreindre la récupération des commentaires sauf s'ils sont demandés par un administrateur ou un utilisateur SSO valide.

#### Configuration sans code

Nous pouvons empêcher la visualisation et l'interaction avec nos fils de commentaires, lorsqu'un SSO est configuré, en créant une <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">règle de personnalisation</a>.

Lors de cette opération, recherchez SSO, et vous trouverez cette option :

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.require-sso'; selector = '.require-sso'; alt='Option Exiger SSO pour afficher les commentaires activée dans une règle de personnalisation, avec le choix du niveau de sécurité'; title='Require SSO To View Comments' app-screenshot-end]

Activez-la et enregistrez la règle de personnalisation.

#### Protéger uniquement un domaine ou une page spécifique

Pour ne protéger qu'un domaine ou une page spécifique, nous configurerons simplement la règle de personnalisation en conséquence.

En haut de l'interface de personnalisation, vous trouverez deux champs, Domaine et ID d'URL.

Pour protéger uniquement un domaine particulier, saisissez le domaine concerné dans le champ « domain ».

Pour protéger une page particulière, saisissez l'URL de la page dans le champ « URL ID ». Si vous avez une intégration personnalisée avec FastComments, vous pouvez saisir ici un type d'ID à la place d'une URL.

#### Niveaux de sécurité

Lorsque vous exigez le SSO, vous devez décider si vous requérez un SSO Simple ou un SSO Sécurisé. Si vous choisissez le SSO Simple, les deux sont autorisés, mais si vous choisissez le SSO Sécurisé, le contenu doit être récupéré avec une charge utile SSO Sécurisée hachée avec votre clé API afin d'être affiché.

L'option de niveau de sécurité apparaîtra lorsque vous sélectionnez « Require SSO To View Comments ».

#### Protection au-delà de la lecture

Activer cette option protégera la page ou le domaine contre les commentaires, sauf si l'utilisateur est connecté via SSO.

#### Points d'attention

Les utilisateurs qui ont créé des commentaires avant votre intégration SSO ne pourront pas les voir, à moins de se connecter via votre intégration SSO.

---