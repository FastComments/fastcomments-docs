FastComments fournit une solution SSO facile à utiliser. Mettre à jour les informations d'un utilisateur avec l'intégration basée sur HMAC
est aussi simple que de charger la page avec une charge utile mise à jour.

Cependant, il peut être souhaitable de gérer un utilisateur en dehors de ce flux, afin d'améliorer la cohérence de votre application.

L'API SSO User fournit un moyen de CRUD des objets que nous appelons SSOUsers. Ces objets sont différents des Users réguliers et
sont conservés séparément pour la sécurité des types.

La structure de l'objet SSOUser est la suivante :

[inline-code-attrs-start title = 'Structure de SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    isAccountOwner?: boolean // Permission admin - les utilisateurs SSO avec ce drapeau sont facturés en tant qu'administrateurs SSO (séparés des utilisateurs SSO réguliers)
    isAdminAdmin?: boolean // Permission admin - les utilisateurs SSO avec ce drapeau sont facturés en tant qu'administrateurs SSO (séparés des utilisateurs SSO réguliers)
    isCommentModeratorAdmin?: boolean // Permission modérateur - les utilisateurs SSO avec ce drapeau sont facturés en tant que modérateurs SSO (séparés des utilisateurs SSO réguliers)
    /** Si null, le contrôle d'accès ne sera pas appliqué à l'utilisateur. Si une liste vide, cet utilisateur ne pourra voir aucune page ni mentionner d'autres utilisateurs via @mention. **/
    groupIds?: string[] | null
    createdFromSimpleSSO?: boolean
    /** N'autorisez pas les autres utilisateurs à voir l'activité de cet utilisateur, y compris les commentaires, sur son profil. Par défaut true pour fournir des profils sécurisés par défaut. **/
    isProfileActivityPrivate?: boolean
    /** N'autorisez pas les autres utilisateurs à laisser des commentaires sur le profil de l'utilisateur, ni à voir les commentaires de profil existants. Par défaut false. **/
    isProfileCommentsPrivate?: boolean
    /** N'autorisez pas les autres utilisateurs à envoyer des messages directs à cet utilisateur. Par défaut false. **/
    isProfileDMDisabled?: boolean
    karma?: number
    /** Configuration optionnelle pour les badges utilisateur. **/
    badgeConfig?: {
        /** Tableau d'ID de badges à attribuer à l'utilisateur. Limité à 30 badges. L'ordre est respecté. Ce sont des badges globaux visibles sur toutes les pages. **/
        badgeIds: string[]
        /** Tableau facultatif d'ID de badges limité à la page actuelle (urlId). Ces badges ne sont affichés que sur la page où ils ont été attribués. **/
        pageBadgeIds?: string[]
        /** Si true, remplace tous les badges affichés existants par ceux fournis. Les badges globaux et ceux limités à une page sont remplacés indépendamment. Si false, ajoute aux badges existants. **/
        override?: boolean
        /** Si true, met à jour les propriétés d'affichage des badges à partir de la configuration du locataire. **/
        update?: boolean
    }
}
[inline-code-end]

### Facturation des utilisateurs SSO

Les utilisateurs SSO sont facturés différemment selon leurs drapeaux de permission :

- **Utilisateurs SSO réguliers** : Les utilisateurs sans permissions d'administrateur ou de modérateur sont facturés comme utilisateurs SSO réguliers
- **Administrateurs SSO** : Les utilisateurs avec les drapeaux `isAccountOwner` ou `isAdminAdmin` sont facturés séparément en tant qu'administrateurs SSO (même tarif que les administrateurs réguliers du locataire)
- **Modérateurs SSO** : Les utilisateurs avec le drapeau `isCommentModeratorAdmin` sont facturés séparément en tant que modérateurs SSO (même tarif que les modérateurs réguliers)

**Important** : Pour éviter la double facturation, le système déduplique automatiquement les utilisateurs SSO par rapport aux utilisateurs et modérateurs réguliers du locataire en fonction de l'adresse courriel. Si un utilisateur SSO a la même adresse courriel qu'un utilisateur ou un modérateur régulier du locataire, il ne sera pas facturé deux fois.

### Contrôle d'accès

Les utilisateurs peuvent être répartis en groupes. C'est à cela que sert le champ `groupIds`, et il est optionnel.

### @Mentions

Par défaut, `@mentions` utilisera `username` pour rechercher d'autres utilisateurs sso lorsque le caractère `@` est tapé. Si `displayName` est utilisé, alors les résultats correspondant à
`username` seront ignorés lorsqu'il y a une correspondance pour `displayName`, et les résultats de recherche de `@mention` utiliseront `displayName`.

### Abonnements

Avec FastComments, les utilisateurs peuvent s'abonner à une page en cliquant sur l'icône de cloche dans le widget de commentaires et en cliquant sur S'abonner.

Avec un utilisateur régulier, nous leur envoyons des courriels de notification en fonction de leurs paramètres de notification.

Avec les utilisateurs SSO, nous séparons cela pour des raisons de rétrocompatibilité. Les utilisateurs ne recevront ces courriels de notification d'abonnement supplémentaires que si vous définissez `optedInSubscriptionNotifications` sur `true`.

### Badges

Vous pouvez attribuer des badges aux utilisateurs SSO en utilisant la propriété `badgeConfig`. Les badges sont des indicateurs visuels qui apparaissent à côté du nom d'un utilisateur dans les commentaires.

- `badgeIds` - Un tableau d'ID de badges à attribuer à l'utilisateur. Ce sont des badges globaux visibles sur toutes les pages. Doivent être des ID de badges valides créés dans votre compte FastComments. Limité à 30 badges.
- `pageBadgeIds` - Un tableau optionnel d'ID de badges limités à la page actuelle (`urlId`). Ces badges ne sont affichés que sur la page où ils ont été attribués. Différentes pages peuvent avoir des badges limités à la page différents pour le même utilisateur.
- `override` - Si true, tous les badges affichés existants seront remplacés par ceux fournis. Les badges globaux et ceux limités à la page sont remplacés indépendamment — le remplacement des badges globaux n'affecte pas les badges limités à la page, et vice versa. Si false ou omis, les badges fournis seront ajoutés aux badges existants.
- `update` - Si true, les propriétés d'affichage des badges seront mises à jour à partir de la configuration du locataire chaque fois que l'utilisateur se connecte.

---