FastComments fournit une solution SSO facile à utiliser. La mise à jour des informations d'un utilisateur avec l'intégration basée sur HMAC est
aussi simple que de faire charger la page à l'utilisateur avec un payload mis à jour.

Cependant, il peut être souhaitable de gérer un utilisateur en dehors de ce flux, pour améliorer la cohérence de votre application.

L'API SSO User fournit un moyen de faire du CRUD sur des objets que nous appelons SSOUsers. Ces objets sont différents des Users réguliers et
gardés séparés pour la sécurité de type.

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
    isAccountOwner?: boolean // Admin permission - SSO users with this flag are billed as SSO Admins (separate from regular SSO users)
    isAdminAdmin?: boolean // Admin permission - SSO users with this flag are billed as SSO Admins (separate from regular SSO users)
    isCommentModeratorAdmin?: boolean // Moderator permission - SSO users with this flag are billed as SSO Moderators (separate from regular SSO users)
    /** If null, Access Control will not be applied to the user. If an empty list, this user will not be able to see any pages or @mention other users. **/
    groupIds?: string[] | null
    createdFromSimpleSSO?: boolean
    /** Don't let other users see this user's activity, including comments, on their profile. Default is true to provide secure profiles by default. **/
    isProfileActivityPrivate?: boolean
    /** Don't let other users leave comments on the user's profile, or see existing profile comments. Default false. **/
    isProfileCommentsPrivate?: boolean
    /** Don't let other users send direct messages to this user. Default false. **/
    isProfileDMDisabled?: boolean
    karma?: number
    /** Optional configuration for user badges. **/
    badgeConfig?: {
        /** Array of badge IDs to assign to the user. Limited to 30 badges. Order is respected. **/
        badgeIds: string[]
        /** If true, replaces all existing displayed badges with the provided ones. If false, adds to existing badges. **/
        override?: boolean
        /** If true, updates badge display properties from tenant configuration. **/
        update?: boolean
    }
}
[inline-code-end]

### Facturation pour les Utilisateurs SSO

Les utilisateurs SSO sont facturés différemment selon leurs drapeaux de permission :

- **Utilisateurs SSO Réguliers** : Les utilisateurs sans permissions d'administrateur ou de modérateur sont facturés comme utilisateurs SSO réguliers
- **Administrateurs SSO** : Les utilisateurs avec les drapeaux `isAccountOwner` ou `isAdminAdmin` sont facturés séparément comme administrateurs SSO (même tarif que les administrateurs de locataire réguliers)
- **Modérateurs SSO** : Les utilisateurs avec le drapeau `isCommentModeratorAdmin` sont facturés séparément comme modérateurs SSO (même tarif que les modérateurs réguliers)

**Important** : Pour éviter la double facturation, le système déduplique automatiquement les utilisateurs SSO par rapport aux utilisateurs et modérateurs de locataire réguliers par adresse email. Si un utilisateur SSO a le même email qu'un utilisateur ou modérateur de locataire régulier, il ne sera pas facturé deux fois.

### Contrôle d'Accès

Les utilisateurs peuvent être divisés en groupes. C'est à cela que sert le champ `groupIds`, et c'est optionnel.

### @Mentions

Par défaut, `@mentions` utilisera `username` pour rechercher d'autres utilisateurs SSO lorsque le caractère `@` est tapé. Si `displayName` est utilisé, alors les résultats correspondant à
`username` seront ignorés lorsqu'il y a une correspondance pour `displayName`, et les résultats de recherche `@mention` utiliseront `displayName`.

### Abonnements

Avec FastComments, les utilisateurs peuvent s'abonner à une page en cliquant sur l'icône de cloche dans le widget de commentaires et en cliquant sur S'abonner.

Avec un utilisateur régulier, nous leur envoyons des emails de notification basés sur leurs paramètres de notification.

Avec les utilisateurs SSO, nous séparons cela pour la rétrocompatibilité. Les utilisateurs ne recevront ces emails de notification d'abonnement supplémentaires
que si vous définissez `optedInSubscriptionNotifications` à `true`.

### Badges

Vous pouvez attribuer des badges aux utilisateurs SSO en utilisant la propriété `badgeConfig`. Les badges sont des indicateurs visuels qui apparaissent à côté du nom d'un utilisateur dans les commentaires.

- `badgeIds` - Un tableau d'IDs de badges à attribuer à l'utilisateur. Ceux-ci doivent être des IDs de badges valides créés dans votre compte FastComments. Limité à 30 badges.
- `override` - Si vrai, tous les badges existants affichés sur les commentaires seront remplacés par ceux fournis. Si faux ou omis, les badges fournis seront ajoutés aux badges existants.
- `update` - Si vrai, les propriétés d'affichage des badges seront mises à jour depuis la configuration du locataire chaque fois que l'utilisateur se connecte.
