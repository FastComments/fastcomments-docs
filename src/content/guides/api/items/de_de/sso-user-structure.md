FastComments provides an easy to use SSO solution. Updating a user's information with the HMAC-based integration is
as simple as having the user load the page with an updated payload.

However, it may be desirable to manage a user outside that flow, to improve consistency of your application.

The SSO User API provides a way to CRUD objects that we call SSOUsers. These objects are different from regular Users and
kept separate for type safety.

The structure for the SSOUser object is as follows:

[inline-code-attrs-start title = 'SSOUser-Struktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    isAccountOwner?: boolean // Administratorberechtigung - SSO-Benutzer mit diesem Flag werden als SSO-Administratoren abgerechnet (separat von regulären SSO-Benutzern)
    isAdminAdmin?: boolean // Administratorberechtigung - SSO-Benutzer mit diesem Flag werden als SSO-Administratoren abgerechnet (separat von regulären SSO-Benutzern)
    isCommentModeratorAdmin?: boolean // Moderatorberechtigung - SSO-Benutzer mit diesem Flag werden als SSO-Moderatoren abgerechnet (separat von regulären SSO-Benutzern)
    /** Wenn null, wird keine Zugriffskontrolle auf den Benutzer angewendet. Bei einer leeren Liste kann dieser Benutzer keine Seiten sehen oder andere Benutzer per @mention erwähnen. **/
    groupIds?: string[] | null
    createdFromSimpleSSO?: boolean
    /** Verhindert, dass andere Benutzer die Aktivitäten dieses Benutzers (einschließlich Kommentare) in seinem Profil sehen. Standardmäßig true, um Profile von Haus aus sicher zu machen. **/
    isProfileActivityPrivate?: boolean
    /** Verhindert, dass andere Benutzer Kommentare im Profil des Benutzers hinterlassen oder vorhandene Profilkommentare sehen. Standardmäßig false. **/
    isProfileCommentsPrivate?: boolean
    /** Verhindert, dass andere Benutzer diesem Benutzer Direktnachrichten senden. Standardmäßig false. **/
    isProfileDMDisabled?: boolean
    karma?: number
    /** Optionale Konfiguration für Benutzerabzeichen. **/
    badgeConfig?: {
        /** Array von Abzeichen-IDs, die dem Benutzer zugewiesen werden. Auf 30 Abzeichen begrenzt. Die Reihenfolge wird beibehalten. Dies sind globale Abzeichen, die auf allen Seiten sichtbar sind. **/
        badgeIds: string[]
        /** Array von Abzeichen-IDs, die auf die aktuelle Seite (urlId) beschränkt sind. Diese Abzeichen werden nur auf der Seite angezeigt, auf der sie vergeben wurden. **/
        pageBadgeIds?: string[]
        /** Wenn true, werden alle vorhandenen angezeigten Abzeichen durch die bereitgestellten ersetzt. Globale und seitenbezogene Abzeichen werden unabhängig voneinander überschrieben. Wenn false, werden die bereitgestellten Abzeichen zu den bestehenden hinzugefügt. **/
        override?: boolean
        /** Wenn true, werden die Anzeigeeigenschaften der Abzeichen aus der Mandantenkonfiguration aktualisiert. **/
        update?: boolean
    }
}
[inline-code-end]

### Billing for SSO Users

SSO users are billed differently based on their permission flags:

- **Regular SSO Users**: Users without admin or moderator permissions are billed as regular SSO users
- **SSO Admins**: Users with `isAccountOwner` or `isAdminAdmin` flags are billed separately as SSO Admins (same rate as regular tenant admins)
- **SSO Moderators**: Users with `isCommentModeratorAdmin` flag are billed separately as SSO Moderators (same rate as regular moderators)

**Important**: To prevent double billing, the system automatically deduplicates SSO users against regular tenant users and moderators by email address. If an SSO user has the same email as a regular tenant user or moderator, they will not be billed twice.

### Access Control

Users can be broken into groups. This is what the `groupIds` field is for, and is optional.

### @Mentions

By default `@mentions` will use `username` to search for other sso users when the `@` character is typed. If `displayName` is used, then results matching
`username` will be ignored when there is a match for `displayName`, and the `@mention` search results will use `displayName`.

### Subscriptions

With FastComments, users can subscribe to a page by clicking the bell icon in the comment widget and clicking Subscribe.

With a regular user, we send them notification emails based on their notification settings.

With SSO Users, we split this up for backwards compatibility. Users will only get sent these additional subscription notification
emails if you set `optedInSubscriptionNotifications` to `true`.

### Badges

You can assign badges to SSO users using the `badgeConfig` property. Badges are visual indicators that appear next to a user's name in comments.

- `badgeIds` - An array of badge IDs to assign to the user. These are global badges visible on all pages. Must be valid badge IDs created in your FastComments account. Limited to 30 badges.
- `pageBadgeIds` - An optional array of badge IDs scoped to the current page (`urlId`). These badges are only displayed on the page where they were assigned. Different pages can have different page-scoped badges for the same user.
- `override` - If true, all existing displayed badges will be replaced with the provided ones. Global and page-scoped badges are overridden independently — overriding global badges does not affect page-scoped badges, and vice versa. If false or omitted, the provided badges will be added to any existing badges.
- `update` - If true, badge display properties will be updated from the tenant configuration whenever the user logs in.