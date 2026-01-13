FastComments provides an easy to use SSO solution. Updating a user's information with the HMAC-based integration is
as simple as having the user load the page with an updated payload.

However, it may be desirable to manage a user outside that flow, to improve consistency of your application.

The SSO User API provides a way to CRUD objects that we call SSOUsers. These objects are different from regular Users and
kept separate for type safety.

The structure for the SSOUser object is as follows:

[inline-code-attrs-start title = 'SSOUser Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    isAccountOwner?: boolean // Yönetici izni - Bu bayrağa sahip SSO kullanıcıları SSO Admins olarak faturalandırılır (normal SSO kullanıcılarından ayrı)
    isAdminAdmin?: boolean // Yönetici izni - Bu bayrağa sahip SSO kullanıcıları SSO Admins olarak faturalandırılır (normal SSO kullanıcılarından ayrı)
    isCommentModeratorAdmin?: boolean // Moderatör izni - Bu bayrağa sahip SSO kullanıcıları SSO Moderators olarak faturalandırılır (normal SSO kullanıcılarından ayrı)
    /** Eğer null ise, Erişim Kontrolü kullanıcıya uygulanmaz. Eğer boş bir liste ise, bu kullanıcı herhangi bir sayfayı göremeyecek veya diğer kullanıcıları @mention yapamayacaktır. **/
    groupIds?: string[] | null
    createdFromSimpleSSO?: boolean
    /** Diğer kullanıcıların bu kullanıcının profilindeki etkinlikleri, yorumlar dahil, görmesini engelle. Varsayılan olarak güvenli profiller sağlamak için true'dur. **/
    isProfileActivityPrivate?: boolean
    /** Diğer kullanıcıların kullanıcının profiline yorum bırakmasını veya mevcut profil yorumlarını görmesini engelle. Varsayılan false. **/
    isProfileCommentsPrivate?: boolean
    /** Diğer kullanıcıların bu kullanıcıya doğrudan mesaj göndermesini engelle. Varsayılan false. **/
    isProfileDMDisabled?: boolean
    karma?: number
    /** Kullanıcı rozetleri için isteğe bağlı yapılandırma. **/
    badgeConfig?: {
        /** Kullanıcıya atanacak rozet ID'lerinden oluşan dizi. 30 rozet ile sınırlıdır. Sıra korunur. **/
        badgeIds: string[]
        /** Eğer true ise, görüntülenen mevcut tüm rozetleri sağlananlarla değiştirir. Eğer false ise mevcut rozetlere ekler. **/
        override?: boolean
        /** Eğer true ise, rozet görüntü özelliklerini kiracı yapılandırmasından günceller. **/
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

- `badgeIds` - An array of badge IDs to assign to the user. These must be valid badge IDs created in your FastComments account. Limited to 30 badges.
- `override` - If true, all existing badges displayed on comments will be replaced with the provided ones. If false or omitted, the provided badges will be added to any existing badges.
- `update` - If true, badge display properties will be updated from the tenant configuration whenever the user logs in.

---