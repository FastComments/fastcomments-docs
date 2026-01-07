Un objet `NotificationCount` représente le compteur de notifications non lues et les métadonnées pour un utilisateur.

S'il n'y a pas de notifications non lues, il n'y aura pas de `NotificationCount` pour l'utilisateur.

Les objets `NotificationCount` sont créés automatiquement et ne peuvent pas être créés via l'API. Ils expirent également après un an.

Vous pouvez effacer le compteur de notifications non lues d'un utilisateur en supprimant son `NotificationCount`.

La structure de l'objet `NotificationCount` est la suivante :

[inline-code-attrs-start title = 'Structure de NotificationCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCount {
    id: string // user id
    count: number
    createdAt: string // date string
    expireAt: string // date string
}
[inline-code-end]
