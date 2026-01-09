SSO, ou authentification unique, est un ensemble de conventions utilisées pour vous permettre, à vous ou à vos utilisateurs, d'utiliser FastComments sans avoir à créer un autre compte.

En supposant que vous n'autorisez pas les commentaires anonymes, un compte est requis pour commenter avec FastComments. Nous facilitons grandement ce processus d'inscription : l'utilisateur laisse simplement son courriel lorsqu'il commente.
Cependant, nous comprenons que même cela représente une friction supplémentaire que certains sites souhaitent éviter.

Nous pouvons réduire cette friction en n'ayant qu'un seul flux de connexion pour l'ensemble de votre site.

### Comment l'obtenir ?
Tous les types de comptes ont actuellement accès au SSO. Cependant, le nombre maximal d'utilisateurs SSO variera selon votre forfait. Comme pour les autres fonctionnalités, les plans Pro et supérieurs offrent un support de développement direct.

Comparons les options, puis détaillons chacune.

### Migration des utilisateurs et des commentaires

Lorsque vous migrez depuis une plateforme avec SSO comme Disqus, vous aurez déjà des utilisateurs et leurs commentaires.

Les commentaires sont importés dans le cadre de votre migration, soit via l'API, soit via notre interface d'importation, soit via le support client. L'interface d'importation est préférable si elle prend en charge la plateforme depuis laquelle vous migrez, car elle intègre la gestion des erreurs, l'extraction et le téléversement des avatars et des médias, ainsi qu'un système de suivi des tâches par lots.

Les utilisateurs eux-mêmes sont ajoutés automatiquement lors de la première visualisation des fils de discussion de commentaires. Alternativement, ils peuvent être ajoutés à l'avance via l'API, mais ce travail n'apporte pas beaucoup d'avantages.

Si les commentaires sont importés et que les utilisateurs SSO ne sont pas ajoutés manuellement via l'API, les commentaires seront automatiquement rattachés au compte de l'utilisateur la première fois que ce compte sera créé lors de la visualisation de n'importe quel fil de commentaires. Ils pourront alors gérer, modifier et supprimer les commentaires qu'ils ont initialement rédigés.

La migration automatique se fait par courriel ou par nom d'utilisateur. Certaines plateformes ne fournissent pas les courriels lors de l'exportation, comme Disqus, nous utilisons donc le nom d'utilisateur dans ce cas.
- Tant que vous fournissez un nom d'utilisateur correspondant, et un courriel dans la charge utile SSO, nous ajouterons le courriel aux objets de commentaire individuels afin que les notifications et les mentions fonctionnent.

Si vous souhaitez importer vos commentaires et utilisateurs en même temps, travaillez avec le support pour migrer les commentaires vers les comptes respectifs des utilisateurs après que ceux-ci aient été importés via l'API.

Pour résumer, la voie la plus simple pour la migration est :

1. Importer les commentaires.
   1. Les avatars et autres médias sont migrés automatiquement si vous utilisez l'Import UI dans `Manage Data -> Imports`.
2. Configurer le SSO Secure ou Simple.
3. Laisser la migration se faire par utilisateur automatiquement lorsqu'ils se connectent pour la première fois.
   1. Cela ajoute généralement moins d'une seconde au temps de chargement de la page si l'utilisateur a moins de 50k commentaires.

### Utilisateurs WordPress
Si vous utilisez notre <a href="https://wordpress.org/plugins/fastcomments/" target="_blank">extension WordPress</a>, il n'y a aucun code à écrire ! Allez simplement à la page d'administration du plugin, cliquez sur SSO Settings, puis Activez.

Cela vous amènera à un assistant d'un seul clic qui créera votre clé API, l'enverra à votre installation WordPress et activera le SSO. Nous avons consolidé cela en un seul clic pour vous.

Notez que si vous installez le plugin pour la première fois, vous devrez suivre le processus de configuration avant de voir la page d'administration avec le bouton SSO Settings.

#### SSO WordPress - Modérateurs

Notez qu'actuellement, pour que le badge « Modérateur » apparaisse à côté de vos modérateurs lorsqu'ils commentent avec le plugin FastComments pour WordPress,
ils doivent également être ajoutés en tant que Modérateur dans le tableau de bord FastComments et avoir leur courriel vérifié.

### Intégrations personnalisées

Pour les intégrations personnalisées, il y a deux options.

### Option Une - SSO sécurisé

Avec le SSO sécurisé, FastComments sait que l'utilisateur qui commente, vote et lit les commentaires est un véritable utilisateur de votre site.

Tant que vous créez une charge utile valide, l'utilisateur bénéficiera toujours d'une expérience de commentaire fluide.

Avec le SSO sécurisé, la charge utile SSO est créée côté serveur en utilisant une authentification HMAC, puis transmise au widget côté client.

Avec le SSO sécurisé, le compte de l'utilisateur est complètement séparé du reste de la base d'utilisateurs de FastComments. Cela signifie que si nous avons deux partenaires
Company A et Company B, chacun peut avoir un utilisateur SSO avec le nom d'utilisateur "Bob".

#### Exigences
- Quelques connaissances de base en développement backend.
- Quelques connaissances de base sur la gestion des clés API secrètes.
- Quelques connaissances de base en développement d'API ou rendu côté serveur.

#### Avantages
- Sécurisé.
- Expérience de commentaire transparente.

#### Inconvénients
- Nécessite du développement backend.

#### Mise à jour des données utilisateur

Avec le SSO sécurisé, chaque fois que vous transmettez la charge utile utilisateur SSO, nous mettrons à jour leur compte avec les dernières informations. Par exemple, si
l'utilisateur a un nom d'utilisateur `X`, et que vous transmettez `Y` dans la charge utile SSO, son nom d'utilisateur deviendra `Y`.

Si vous souhaitez supprimer des valeurs en utilisant cette approche, définissez-les sur `null` (et non `undefined`).

#### API SSO sécurisé

Nous fournissons également une API pour interagir avec les utilisateurs SSO. Voir [la documentation](/guide-api.html#sso-user-structure).

Notez que lors de l'utilisation du SSO sécurisé, les utilisateurs sont automatiquement créés en arrière-plan au chargement de la page. Vous n'avez pas à importer massivement vos utilisateurs.

### Option Deux - SSO simple

L'alternative au SSO sécurisé est de simplement transmettre les informations de l'utilisateur au widget de commentaire.

Fournir un courriel avec le SSO simple n'est pas requis ; toutefois, sans celui-ci leurs commentaires s'afficheront comme « Non vérifié ».

<sup>Remarque !</sup> À compter du début 2022, les noms d'utilisateur avec le SSO simple n'ont pas besoin d'être uniques sur l'ensemble de FastComments.com.

Idéalement, le SSO simple ne devrait être choisi que lorsque vous développez sur une plateforme qui ne fournit pas d'accès backend.

#### Exigences
- Quelques connaissances de base en développement côté client.
- Connaître au moins le courriel de l'utilisateur.

#### Avantages
- Simple.
- Toute l'activité est toujours vérifiée.
- L'utilisateur ne saisit jamais son nom d'utilisateur ou son courriel.

#### Inconvénients
- Moins sécurisé que le SSO sécurisé, car la charge utile côté client pourrait être falsifiée pour devenir n'importe quel utilisateur.

#### API SSO simple

Les utilisateurs créés automatiquement via le flux SSO simple sont stockés sous forme d'objets `SSOUser`. Ils peuvent être consultés et gérés via l'API `SSOUser`. Voir [la documentation](/guide-api.html#sso-user-structure).