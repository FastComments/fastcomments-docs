FastComments bietet eine einfach zu verwendende SSO-Lösung. Das Aktualisieren der Benutzerinformationen mit der HMAC-basierten Integration ist
so einfach wie das Laden der Seite mit einem aktualisierten Payload durch den Benutzer.

Es kann jedoch wünschenswert sein, einen Benutzer außerhalb dieses Flows zu verwalten, um die Konsistenz Ihrer Anwendung zu verbessern.

Die SSO-Benutzer-API bietet eine Möglichkeit, Objekte zu erstellen, zu lesen, zu aktualisieren und zu löschen, die wir SSOUsers nennen. Diese Objekte unterscheiden sich von regulären Benutzern und
werden aus Gründen der Typsicherheit getrennt gehalten.

Die Struktur des SSOUser-Objekts ist wie folgt:

[inline-code-attrs-start title = 'SSOUser Struktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

### Abrechnung für SSO-Benutzer

SSO-Benutzer werden je nach ihren Berechtigungsflags unterschiedlich abgerechnet:

- **Reguläre SSO-Benutzer**: Benutzer ohne Admin- oder Moderatorberechtigungen werden als reguläre SSO-Benutzer abgerechnet
- **SSO-Admins**: Benutzer mit `isAccountOwner`- oder `isAdminAdmin`-Flags werden separat als SSO-Admins abgerechnet (gleicher Tarif wie reguläre Tenant-Admins)
- **SSO-Moderatoren**: Benutzer mit `isCommentModeratorAdmin`-Flag werden separat als SSO-Moderatoren abgerechnet (gleicher Tarif wie reguläre Moderatoren)

**Wichtig**: Um doppelte Abrechnung zu vermeiden, dedupliziert das System automatisch SSO-Benutzer gegen reguläre Tenant-Benutzer und Moderatoren nach E-Mail-Adresse. Wenn ein SSO-Benutzer dieselbe E-Mail wie ein regulärer Tenant-Benutzer oder Moderator hat, wird er nicht doppelt abgerechnet.

### Zugriffskontrolle

Benutzer können in Gruppen aufgeteilt werden. Dafür ist das Feld `groupIds` da, und es ist optional.

### @Mentions

Standardmäßig verwendet `@mentions` `username`, um nach anderen SSO-Benutzern zu suchen, wenn das `@`-Zeichen eingegeben wird. Wenn `displayName` verwendet wird, werden Ergebnisse, die mit
`username` übereinstimmen, ignoriert, wenn es eine Übereinstimmung für `displayName` gibt, und die `@mention`-Suchergebnisse verwenden `displayName`.

### Abonnements

Mit FastComments können Benutzer eine Seite abonnieren, indem sie auf das Glockensymbol im Kommentar-Widget klicken und auf Abonnieren klicken.

Bei einem regulären Benutzer senden wir ihm Benachrichtigungs-E-Mails basierend auf seinen Benachrichtigungseinstellungen.

Bei SSO-Benutzern teilen wir dies aus Gründen der Abwärtskompatibilität auf. Benutzer erhalten diese zusätzlichen Abonnement-Benachrichtigungs-E-Mails
nur, wenn Sie `optedInSubscriptionNotifications` auf `true` setzen.

### Badges

Sie können SSO-Benutzern Badges über die `badgeConfig`-Eigenschaft zuweisen. Badges sind visuelle Indikatoren, die neben dem Benutzernamen in Kommentaren erscheinen.

- `badgeIds` - Ein Array von Badge-IDs, die dem Benutzer zugewiesen werden sollen. Diese müssen gültige Badge-IDs sein, die in Ihrem FastComments-Konto erstellt wurden. Auf 30 Badges begrenzt.
- `override` - Wenn wahr, werden alle vorhandenen Badges, die in Kommentaren angezeigt werden, durch die bereitgestellten ersetzt. Wenn falsch oder weggelassen, werden die bereitgestellten Badges zu allen vorhandenen Badges hinzugefügt.
- `update` - Wenn wahr, werden Badge-Anzeigeeigenschaften aus der Tenant-Konfiguration aktualisiert, wann immer sich der Benutzer anmeldet.
