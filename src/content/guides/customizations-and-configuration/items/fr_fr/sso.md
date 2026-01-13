SSO, ou authentification unique, est un ensemble de conventions utilisé pour vous permettre, à vous ou à vos utilisateurs, d'utiliser FastComments sans avoir à créer un autre compte.

Si vous n'autorisez pas les commentaires anonymes, un compte est requis pour commenter avec FastComments. Nous rendons ce processus d'inscription très simple - l'utilisateur laisse simplement son e-mail lorsqu'il commente.
Cependant, nous comprenons que même cela représente une friction supplémentaire que certains sites souhaitent éviter.

Nous pouvons réduire cette friction en n'ayant qu'un seul flux de connexion pour l'ensemble de votre site.

### Comment l'obtenir ?
Tous les types de comptes ont actuellement accès au SSO. Cependant, le nombre maximal d'utilisateurs SSO variera en fonction de votre forfait. Comme pour les autres fonctionnalités, les plans Pro et supérieurs offrent un support de développement direct.

Comparons les options, puis examinons les détails de chacune.

### Migration des utilisateurs et des commentaires

Lorsque vous migrez depuis une plateforme avec SSO comme Disqus, vous aurez déjà des utilisateurs et leurs commentaires.

Les commentaires sont importés dans le cadre de votre migration, soit via l'API, notre interface d'importation, soit via le support client. L'interface d'importation est préférée si elle prend en charge la plateforme depuis laquelle vous migrez, car elle intègre la gestion des erreurs, l'extraction et le téléversement des avatars et médias, ainsi qu'un système de suivi des tâches par lots.

Les utilisateurs eux-mêmes sont ajoutés automatiquement lorsqu'ils consultent des fils de commentaires pour la première fois. Alternativement, ils peuvent être ajoutés à l'avance via l'API, mais ce travail n'offre pas beaucoup d'avantages.

Si les commentaires sont importés et que les utilisateurs SSO ne sont pas ajoutés manuellement via l'API, alors les commentaires seront automatiquement migrés vers le compte de l'utilisateur la première fois que celui-ci est créé lorsqu'il consulte un fil de commentaires. Ils pourront alors gérer, modifier et supprimer les commentaires qu'ils ont initialement écrits.

La migration automatique se fait via l'email ou le nom d'utilisateur. Certaines plateformes n'indiquent pas les emails lors de l'export, comme Disqus, donc nous revenons au nom d'utilisateur dans ce cas.
- Tant que vous fournissez un nom d'utilisateur correspondant, et un e-mail dans la charge utile SSO, nous ajouterons l'e-mail aux objets de commentaire individuels afin que les notifications et mentions fonctionnent.

Si vous souhaitez importer vos commentaires et utilisateurs en même temps, travaillez avec le support pour migrer les commentaires vers les comptes respectifs des utilisateurs après que les utilisateurs ont été importés via l'API.

Pour résumer, le chemin le plus simple pour la migration est :

1. Importer les commentaires.
   1. Les avatars et autres médias sont migrés automatiquement si vous utilisez l'Import UI dans `Manage Data -> Imports`.
2. Configurer Secure ou Simple SSO.
3. Laisser la migration s'effectuer automatiquement par utilisateur lorsqu'ils se connectent pour la première fois.
   1. Cela ajoute généralement moins d'une seconde au temps de chargement de la page si l'utilisateur a moins de 50k commentaires.

### Utilisateurs WordPress
Si vous utilisez notre <a href="https://wordpress.org/plugins/fastcomments/" target="_blank">plugin WordPress</a>, il n'y a aucun code à écrire ! Allez simplement sur la page d'administration du plugin, cliquez sur SSO Settings, puis activez.

Cela vous mènera à un assistant en un seul clic qui créera votre clé API, l'enverra à votre installation WordPress et activera le SSO. Nous avons regroupé cela en un seul clic pour vous.

Notez que si vous installez le plugin pour la première fois, vous devrez suivre le processus d'installation avant de voir la page d'administration avec le bouton SSO Settings.

#### WordPress SSO - Moderators

Notez qu'actuellement pour que le badge "Moderator" apparaisse à côté de vos modérateurs lorsqu'ils commentent avec le plugin FastComments pour WordPress,
ils doivent également être ajoutés en tant que Moderator dans le tableau de bord FastComments, et avoir leur e-mail vérifié.

### Intégrations personnalisées

Pour les intégrations personnalisées, il existe deux options.

### Option One - Secure SSO

Avec Secure SSO, FastComments sait que l'utilisateur qui commente, vote et lit les commentaires est un véritable utilisateur de votre site.

Tant que vous créez une charge utile valide, l'utilisateur aura toujours une expérience de commentaire transparente.

Avec Secure SSO, la charge utile SSO est créée **côté serveur** en utilisant l'authentification HMAC puis transmise au widget côté **client**.

Avec Secure SSO, le compte de l'utilisateur est **complètement séparé** du reste de la base d'utilisateurs FastComments. Cela signifie que si nous avons deux partenaires
Company A et Company B, chacun peut avoir un utilisateur SSO avec le nom d'utilisateur "Bob".

#### Exigences
- Quelques connaissances de base en développement backend.
- Quelques connaissances de base sur la gestion des clés API secrètes.
- Quelques connaissances de base en développement d'API ou en rendu côté serveur.

#### Avantages
- Sécurisé.
- Expérience de commentaire transparente.

#### Inconvénients
- Nécessite du développement backend.

#### Mise à jour des données utilisateur

Avec Secure SSO, chaque fois que vous transmettez la charge utile utilisateur sso, nous mettons à jour leur utilisateur avec les informations les plus récentes. Par exemple, si
l'utilisateur a un nom d'utilisateur `X`, et que vous transmettez `Y` dans la charge utile SSO, son nom d'utilisateur deviendra `Y`.

Si vous souhaitez supprimer des valeurs en utilisant cette approche, définissez-les sur `null` (pas `undefined`).

#### Secure SSO API

Nous fournissons également une API pour interagir avec les utilisateurs SSO. Voir [la documentation](/guide-api.html#sso-user-structure).

Notez que lors de l'utilisation de Secure SSO, les utilisateurs sont automatiquement créés en coulisses au chargement de la page. Vous n'avez pas à importer massivement vos utilisateurs.

### Option Two - Simple SSO

L'alternative à Secure SSO est de simplement transmettre les informations de l'utilisateur au widget de commentaires.

Fournir un e-mail avec Simple SSO n'est pas obligatoire, cependant sans cela leurs commentaires apparaîtront comme "Unverified".

<sup>Remarque !</sup> Depuis début 2022, les noms d'utilisateur avec Simple SSO n'ont pas besoin d'être uniques sur tout FastComments.com.

Idéalement, Simple SSO ne devrait être choisi que lors du développement sur une plateforme qui ne fournit pas d'accès backend.

#### Exigences
- Quelques connaissances de base en développement côté client.
- Il faut connaître au moins l'e-mail de l'utilisateur.

#### Avantages
- Simple.
- Toute l'activité est toujours vérifiée.
- L'utilisateur ne saisit jamais son nom d'utilisateur ni son e-mail.

#### Inconvénients
- Moins sécurisé que Secure SSO car la charge utile côté client pourrait être fabriquée pour devenir n'importe quel utilisateur.

#### Simple SSO API

Les utilisateurs créés automatiquement via le flux Simple SSO sont stockés en tant qu'objets `SSOUser`. Ils peuvent être consultés et gérés via l'API `SSOUser`. Voir [la documentation](/guide-api.html#sso-user-structure).