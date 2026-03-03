FastComments fournit une solution SSO facile à utiliser. La mise à jour des informations d'un utilisateur avec l'intégration basée sur HMAC est aussi simple que de faire charger à l'utilisateur la page avec une charge utile mise à jour.

Cependant, il peut être souhaitable de gérer un utilisateur en dehors de ce flux, afin d'améliorer la cohérence de votre application.

L'API SSO User fournit un moyen de CRUD des objets que nous appelons SSOUsers. Ces objets sont différents des Users réguliers et sont gardés séparés pour la sécurité des types.

La structure de l'objet SSOUser est la suivante :

[inline-code-attrs-start title = 'Structure SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    id: string
    username: string
    email?: string
    websiteUrl?: string
    signUpDate: number
    createdFromUrlId?: string
    loginCount?: number
    avatarSrc?: string
    optedInNotifications?: boolean
    optedInSubscriptionNotifications?: boolean
    displayLabel?: string
    displayName?: string
    isAccountOwner?: boolean // Permission d'administration - les utilisateurs SSO avec ce flag sont facturés en tant qu'administrateurs SSO (distincts des utilisateurs SSO réguliers)
    isAdminAdmin?: boolean // Permission d'administration - les utilisateurs SSO avec ce flag sont facturés en tant qu'administrateurs SSO (distincts des utilisateurs SSO réguliers)
    isCommentModeratorAdmin?: boolean // Permission de modérateur - les utilisateurs SSO avec ce flag sont facturés en tant que modérateurs SSO (distincts des utilisateurs SSO réguliers)
    /** Si null, le contrôle d'accès ne sera pas appliqué à l'utilisateur. Si la liste est vide, cet utilisateur ne pourra voir aucune page ni utiliser @ pour mentionner d'autres utilisateurs. **/
    groupIds?: string[] | null
    createdFromSimpleSSO?: boolean
    /** Empêche les autres utilisateurs de voir l'activité de cet utilisateur, y compris les commentaires, sur son profil. Par défaut true pour garantir des profils sécurisés. **/
    isProfileActivityPrivate?: boolean
    /** Empêche les autres utilisateurs de laisser des commentaires sur le profil de l'utilisateur, ou de voir les commentaires de profil existants. Par défaut false. **/
    isProfileCommentsPrivate?: boolean
    /** Empêche les autres utilisateurs d'envoyer des messages directs à cet utilisateur. Par défaut false. **/
    isProfileDMDisabled?: boolean
    karma?: number
    /** Configuration optionnelle pour les badges utilisateur. **/
    badgeConfig?: {
        /** Tableau d'IDs de badges à attribuer à l'utilisateur. Limité à 30 badges. L'ordre est respecté. Ce sont des badges globaux visibles sur toutes les pages. **/
        badgeIds: string[]
        /** Tableau d'IDs de badges limité à la page courante (urlId). Ces badges ne sont affichés que sur la page où ils ont été attribués. **/
        pageBadgeIds?: string[]
        /** Si true, remplace tous les badges affichés existants par ceux fournis. Les badges globaux et page-scopés sont écrasés indépendamment. Si false, ajoute aux badges existants. **/
        override?: boolean
        /** Si true, met à jour les propriétés d'affichage des badges depuis la configuration du tenant. **/
        update?: boolean
    }
}
[inline-code-end]

### Facturation des utilisateurs SSO

Les utilisateurs SSO sont facturés différemment en fonction de leurs indicateurs d'autorisation :

- **Utilisateurs SSO réguliers**: Les utilisateurs sans permissions d'administrateur ou de modérateur sont facturés comme des utilisateurs SSO réguliers
- **Administrateurs SSO**: Les utilisateurs avec les flags `isAccountOwner` ou `isAdminAdmin` sont facturés séparément en tant qu'administrateurs SSO (même tarif que les administrateurs réguliers du tenant)
- **Modérateurs SSO**: Les utilisateurs avec le flag `isCommentModeratorAdmin` sont facturés séparément en tant que modérateurs SSO (même tarif que les modérateurs réguliers)

**Important**: Pour éviter la double facturation, le système déduplique automatiquement les utilisateurs SSO par rapport aux utilisateurs et modérateurs réguliers du tenant en utilisant l'adresse e-mail. Si un utilisateur SSO a la même adresse e-mail qu'un utilisateur ou modérateur régulier du tenant, il ne sera pas facturé deux fois.

### Contrôle d'accès

Les utilisateurs peuvent être répartis en groupes. C'est à cela que sert le champ `groupIds`, et il est optionnel.

### @Mentions

Par défaut, les `@mentions` utiliseront `username` pour rechercher d'autres utilisateurs SSO lorsque le caractère `@` est tapé. Si `displayName` est utilisé, alors les résultats correspondant à `username` seront ignorés lorsqu'il y a une correspondance pour `displayName`, et les résultats de recherche de `@mention` utiliseront `displayName`.

### Abonnements

Avec FastComments, les utilisateurs peuvent s'abonner à une page en cliquant sur l'icône de cloche dans le widget de commentaires puis sur S'abonner.

Avec un utilisateur régulier, nous lui envoyons des e-mails de notification en fonction de ses paramètres de notification.

Avec les utilisateurs SSO, nous séparons cela pour assurer la compatibilité descendante. Les utilisateurs ne recevront ces e-mails de notification d'abonnement supplémentaires que si vous définissez `optedInSubscriptionNotifications` sur `true`.

### Badges

Vous pouvez attribuer des badges aux utilisateurs SSO en utilisant la propriété `badgeConfig`. Les badges sont des indicateurs visuels qui apparaissent à côté du nom d'un utilisateur dans les commentaires.

- `badgeIds` - Un tableau d'IDs de badges à attribuer à l'utilisateur. Ce sont des badges globaux visibles sur toutes les pages. Doivent être des IDs de badges valides créés dans votre compte FastComments. Limité à 30 badges.
- `pageBadgeIds` - Un tableau optionnel d'IDs de badges limité à la page courante (`urlId`). Ces badges ne sont affichés que sur la page où ils ont été attribués. Différentes pages peuvent avoir des badges page-scopés différents pour le même utilisateur.
- `override` - Si true, tous les badges affichés existants seront remplacés par ceux fournis. Les badges globaux et page-scopés sont écrasés indépendamment — écraser les badges globaux n'affecte pas les badges page-scopés, et vice versa. Si false ou omis, les badges fournis seront ajoutés aux badges existants.
- `update` - Si true, les propriétés d'affichage des badges seront mises à jour depuis la configuration du tenant chaque fois que l'utilisateur se connecte.

---