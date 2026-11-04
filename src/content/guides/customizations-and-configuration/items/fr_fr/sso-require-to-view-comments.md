FastComments SSO (<a href="#sso">détails ici</a>) offre à vos utilisateurs un moyen de commenter sans devoir se connecter à une autre plateforme.

Cependant, cela ne sécurise pas vos fils de commentaires, car par défaut les données des commentaires sont des informations publiques — tout le monde pouvant voir la page peut voir les commentaires.

En modifiant un paramètre, nous pouvons restreindre la récupération des commentaires sauf s’ils sont demandés par un administrateur ou un utilisateur SSO valide.

#### No-Code Setup

Nous pouvons empêcher la visualisation et l’interaction avec nos fils de commentaires, lorsque le SSO est configuré, en créant une <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">règle de personnalisation</a>.

Lors de cette opération, recherchez « SSO », et vous trouverez cette option :

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.require-sso'; selector = '.require-sso'; alt='Option Exiger SSO pour voir les commentaires activée dans une règle de personnalisation, avec le choix du niveau de sécurité'; title='Exiger SSO pour voir les commentaires' app-screenshot-end]

Activez‑la et enregistrez la règle de personnalisation.

#### Only Protect a Certain Domain or Page

Pour ne protéger qu’un domaine ou une page spécifique, nous configurerons simplement la règle de personnalisation en conséquence.

En haut de l’interface de personnalisation, vous trouverez deux champs, Domaine et ID d’URL.

Pour protéger uniquement un domaine particulier, saisissez le domaine concerné dans le champ « domaine ».

Pour protéger une page particulière, saisissez l’URL de la page dans le champ « ID d’URL ». Si vous avez une intégration personnalisée avec FastComments, vous pouvez saisir un type d’ID ici à la place d’une URL.

#### Security Levels

Lorsque vous exigez le SSO, vous devez décider si vous requérez Simple SSO ou Secure SSO. Si vous choisissez Simple SSO, les deux sont autorisés, mais si vous choisissez Secure SSO, le contenu doit être récupéré avec une charge Secure SSO hachée avec votre clé API afin d’être affiché.

L’option de niveau de sécurité apparaîtra lorsque vous sélectionnez « Require SSO To View Comments », et ne prendra effet que tant que cette option est activée.

Les paramètres <a href="#sso-simple">Vérification automatique des commentaires via Simple SSO et autoriser les administrateurs Simple SSO</a> sont séparés, et s’appliquent que l’option soit activée ou non.

#### Protection Beyond Reading

Activer cette option protégera la page ou le domaine contre les commentaires, sauf si l’utilisateur est connecté via SSO.

#### Gotchas

Tout utilisateur ayant créé des commentaires avant votre intégration SSO ne pourra plus les voir, à moins de se connecter via votre intégration SSO.