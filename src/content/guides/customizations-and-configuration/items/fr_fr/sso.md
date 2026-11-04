Le SSO, ou authentification unique (single-sign-on), est un ensemble de conventions permettant à vous ou à vos utilisateurs d'utiliser FastComments sans avoir à créer un autre compte.

En supposant que vous n'autorisiez pas les commentaires anonymes, un compte est requis pour commenter avec FastComments. Nous rendons ce processus d'inscription très simple - l'utilisateur indique simplement son e-mail lorsqu'il commente.
Cependant, nous comprenons que même cela représente une friction supplémentaire que certains sites souhaitent éviter.

Nous pouvons réduire cette friction en n'ayant qu'un seul flux de connexion pour l'ensemble de votre site.

### Comment l'obtenir ?
Tous les types de comptes ont actuellement accès au SSO. Cependant, le nombre maximum d'utilisateurs SSO variera selon votre formule. Comme pour les autres fonctionnalités, les formules Pro et supérieures offrent un support de développement direct.

Comparons les options, puis détaillons chacune d'elles.

### Migrations d'utilisateurs et de commentaires

Lors d'une migration depuis une plateforme avec SSO comme Disqus, vous avez déjà des utilisateurs et leurs commentaires.

Les commentaires sont importés dans le cadre de votre migration, soit via l'API, notre interface d'importation, ou le support client. L'interface d'importation est préférée si elle prend en charge la plateforme depuis laquelle vous migrez, car elle intègre la gestion des erreurs, l'extraction et le téléversement des avatars et médias, ainsi qu'un système de suivi des traitements par lots.

Les utilisateurs eux-mêmes sont ajoutés automatiquement lors de la première consultation des fils de commentaires. Alternativement, ils peuvent être pré-ajoutés via l'API, mais ce travail n'offre pas
de nombreux avantages.

Si les commentaires sont importés et que les utilisateurs SSO ne sont pas ajoutés manuellement via l'API, alors les commentaires seront automatiquement migrés vers le compte de l'utilisateur lors de sa première création, quand il consulte n'importe quel fil de commentaires. Il pourra ensuite gérer, modifier et supprimer les commentaires qu'il a initialement écrits.

La migration automatique se fait via l'e-mail ou le nom d'utilisateur. Certaines plateformes ne fournissent pas les e-mails lors de l'export, comme Disqus, nous utilisons donc le nom d'utilisateur dans ce cas.
- Tant que vous transmettez un nom d'utilisateur correspondant et un e-mail dans le payload SSO, nous ajouterons l'e-mail aux objets de commentaires individuels afin que les notifications et les mentions fonctionnent.

Si vous souhaitez importer vos commentaires et vos utilisateurs en une seule fois, travaillez avec le support pour migrer les commentaires vers les comptes respectifs des utilisateurs après que les utilisateurs ont été importés
via l'API.

Pour résumer, le chemin le plus simple pour la migration est :

1. Importer les commentaires.
   1. Les avatars et autres médias sont migrés automatiquement si vous utilisez l'interface d'importation dans `Manage Data -> Imports`.
2. Configurer le SSO Sécurisé ou Simple.
3. Laisser la migration se faire automatiquement par utilisateur lors de leur première connexion.
   1. Cela ajoute généralement moins d'une seconde au temps de chargement de la page si l'utilisateur a moins de 50 000 commentaires.

### Utilisateurs WordPress
Si vous utilisez notre <a href="https://wordpress.org/plugins/fastcomments/" target="_blank">plugin WordPress</a>, il n'y a aucun code à écrire ! Rendez-vous simplement sur la page Admin du plugin, cliquez sur SSO Settings, puis sur Enable.

Cela vous amènera à un assistant en un seul clic qui créera votre clé API, l'enverra à votre installation WordPress et activera le SSO. Nous avons consolidé tout cela en un seul clic pour vous.

Notez que si vous installez le plugin pour la première fois, vous devrez suivre le processus de configuration avant de voir la page d'administration avec le bouton SSO Settings.

#### WordPress SSO - Modérateurs

Notez qu'actuellement, pour que le badge « Modérateur » s'affiche à côté de vos modérateurs lorsqu'ils commentent avec le plugin WordPress FastComments,
ils doivent également être ajoutés comme modérateurs dans le tableau de bord FastComments et avoir leur e-mail vérifié.

### Intégrations personnalisées

Pour les intégrations personnalisées, il existe deux options.

### Option Un - SSO Sécurisé

Avec le SSO Sécurisé, FastComments sait que l'utilisateur qui commente, vote et lit les commentaires est un utilisateur réel de votre site.

Tant que vous créez un payload valide, l'utilisateur aura toujours une expérience de commentaire fluide.

Avec le SSO Sécurisé, le payload SSO est créé **côté serveur** à l'aide de l'authentification HMAC puis transmis au widget côté **client**.

Avec le SSO Sécurisé, le compte de l'utilisateur est **complètement séparé** du reste de la base d'utilisateurs FastComments. Cela signifie que si nous avons deux partenaires,
la Société A et la Société B, chacune peut avoir un utilisateur SSO avec le nom d'utilisateur « Bob ».

#### Prérequis
- Des connaissances de base en développement backend.
- Des connaissances de base dans la gestion des clés API secrètes.
- Des connaissances de base en développement d'API ou en rendu côté serveur.

#### Avantages
- Sécurisé.
- Expérience de commentaire fluide.

#### Inconvénients
- Nécessite un développement backend.

#### Mise à jour des données utilisateur

Avec le SSO Sécurisé, chaque fois que vous transmettez le payload de l'utilisateur SSO, nous mettons à jour son utilisateur avec les dernières informations. Par exemple, si
l'utilisateur a un nom d'utilisateur `X` et que vous transmettez `Y` dans le payload SSO, son nom d'utilisateur deviendra `Y`.

Si vous souhaitez supprimer des valeurs avec cette approche, définissez-les sur `null` (pas `undefined`).

#### API du SSO Sécurisé

Nous fournissons également une API pour interagir avec les utilisateurs SSO. Voir [la documentation](/guide-api.html#sso-user-structure).

Notez que lors de l'utilisation du SSO Sécurisé, les utilisateurs sont automatiquement créés en arrière-plan au chargement de la page. Vous n'avez pas besoin d'importer vos utilisateurs en masse.

### Option Deux - SSO Simple

L'alternative au SSO Sécurisé est de simplement transmettre les informations de l'utilisateur au widget de commentaires.

Fournir un e-mail avec le SSO Simple n'est pas obligatoire, mais sans cela, leurs commentaires s'afficheront comme « Non vérifié ».

<sup>Note !</sup> Depuis début 2022, les noms d'utilisateur avec le SSO Simple n'ont plus besoin d'être uniques sur l'ensemble de FastComments.com.

Idéalement, le SSO Simple ne devrait être choisi que lors du développement sur une plateforme qui ne fournit pas d'accès backend.

#### Prérequis
- Des connaissances de base en développement côté client.
- Connaître au moins l'e-mail de l'utilisateur.

#### Avantages
- Simple.
- Toute l'activité reste vérifiée.
- L'utilisateur ne saisit jamais son nom d'utilisateur ou son e-mail.

#### Inconvénients
- Moins sécurisé que le SSO Sécurisé car le payload côté client pourrait être forgé pour devenir n'importe quel utilisateur.

#### API du SSO Simple

Les utilisateurs créés automatiquement via le flux SSO Simple sont stockés en tant qu'objets `SSOUser`. Ils peuvent être consultés et gérés via l'API `SSOUser`. Voir [la documentation](/guide-api.html#sso-user-structure).