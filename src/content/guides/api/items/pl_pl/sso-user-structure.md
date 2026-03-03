FastComments provides an easy to use SSO solution. Aktualizacja informacji użytkownika przy użyciu integracji opartej na HMAC jest
as simple as having the user load the page with an updated payload.

However, it may be desirable to manage a user outside that flow, to improve consistency of your application.

The SSO User API provides a way to CRUD objects that we call SSOUsers. These objects are different from regular Users and
kept separate for type safety.

The structure for the SSOUser object is as follows:

[inline-code-attrs-start title = 'Struktura SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    isAccountOwner?: boolean // Uprawnienie administratora - użytkownicy SSO z tą flagą są rozliczani jako administratorzy SSO (oddzielnie od zwykłych użytkowników SSO)
    isAdminAdmin?: boolean // Uprawnienie administratora - użytkownicy SSO z tą flagą są rozliczani jako administratorzy SSO (oddzielnie od zwykłych użytkowników SSO)
    isCommentModeratorAdmin?: boolean // Uprawnienie moderatora - użytkownicy SSO z tą flagą są rozliczani jako moderatorzy SSO (oddzielnie od zwykłych użytkowników SSO)
    /** Jeśli null, Kontrola dostępu nie zostanie zastosowana do użytkownika. Jeśli pusta lista, ten użytkownik nie będzie mógł zobaczyć żadnych stron ani oznaczać innych użytkowników. **/
    groupIds?: string[] | null
    createdFromSimpleSSO?: boolean
    /** Nie pozwalaj innym użytkownikom widzieć aktywności tego użytkownika, w tym komentarzy, na jego profilu. Domyślnie true, aby zapewnić bezpieczne profile. **/
    isProfileActivityPrivate?: boolean
    /** Nie pozwalaj innym użytkownikom zostawiać komentarzy na profilu tego użytkownika ani widzieć istniejących komentarzy profilowych. Domyślnie false. **/
    isProfileCommentsPrivate?: boolean
    /** Nie pozwalaj innym użytkownikom wysyłać wiadomości bezpośrednich do tego użytkownika. Domyślnie false. **/
    isProfileDMDisabled?: boolean
    karma?: number
    /** Opcjonalna konfiguracja odznak użytkownika. **/
    badgeConfig?: {
        /** Tablica identyfikatorów odznak do przypisania użytkownikowi. Ograniczenie do 30 odznak. Kolejność jest zachowana. To są globalne odznaki widoczne na wszystkich stronach. **/
        badgeIds: string[]
        /** Tablica identyfikatorów odznak przypisanych do bieżącej strony (urlId). Te odznaki są wyświetlane tylko na stronie, na której je przypisano. **/
        pageBadgeIds?: string[]
        /** Jeśli true, zastępuje wszystkie istniejące wyświetlane odznaki podanymi. Odznaki globalne i przypisane do stron są nadpisywane niezależnie. Jeśli false, dodaje do istniejących odznak. **/
        override?: boolean
        /** Jeśli true, aktualizuje właściwości wyświetlania odznak z konfiguracji najemcy. **/
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

### @Wzmianki

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