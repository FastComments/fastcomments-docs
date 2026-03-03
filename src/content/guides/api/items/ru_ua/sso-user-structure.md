FastComments предоставляет простое в использовании SSO-решение. Обновление информации о пользователе с интеграцией на основе HMAC
так же просто, как загрузка страницы пользователем с обновлённым payload.

Тем не менее, может быть целесообразно управлять пользователем вне этого потока, чтобы повысить согласованность вашего приложения.

SSO User API предоставляет способ CRUD объектов, которые мы называем SSOUsers. Эти объекты отличаются от обычных Users и
хранятся отдельно для обеспечения типобезопасности.

Структура объекта SSOUser выглядит следующим образом:

[inline-code-attrs-start title = 'Структура SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    isAccountOwner?: boolean // Права администратора - SSO-пользователи с этим флагом учитываются как SSO-администраторы (отдельно от обычных SSO-пользователей)
    isAdminAdmin?: boolean // Права администратора - SSO-пользователи с этим флагом учитываются как SSO-администраторы (отдельно от обычных SSO-пользователей)
    isCommentModeratorAdmin?: boolean // Права модератора - SSO-пользователи с этим флагом учитываются как SSO-модераторы (отдельно от обычных SSO-пользователей)
    /** Если null, Контроль Доступа не будет применяться к пользователю. Если пустой список, этот пользователь не сможет видеть никакие страницы или упоминать других пользователей с помощью @. **/
    groupIds?: string[] | null
    createdFromSimpleSSO?: boolean
    /** Не позволяйте другим пользователям видеть активность этого пользователя, включая комментарии, в его профиле. По умолчанию true, чтобы обеспечить безопасность профилей по умолчанию. **/
    isProfileActivityPrivate?: boolean
    /** Не позволяйте другим пользователям оставлять комментарии в профиле пользователя или видеть существующие комментарии профиля. По умолчанию false. **/
    isProfileCommentsPrivate?: boolean
    /** Не позволяйте другим пользователям отправлять этому пользователю личные сообщения. По умолчанию false. **/
    isProfileDMDisabled?: boolean
    karma?: number
    /** Необязательная конфигурация для бейджей пользователя. **/
    badgeConfig?: {
        /** Массив идентификаторов бейджей для назначения пользователю. Ограничение — 30 бейджей. Порядок сохраняется. Это глобальные бейджи, видимые на всех страницах. **/
        badgeIds: string[]
        /** Массив идентификаторов бейджей, привязанных к текущей странице (urlId). Эти бейджи отображаются только на странице, где они были назначены. **/
        pageBadgeIds?: string[]
        /** Если true, заменяет все существующие отображаемые бейджи предоставленными. Глобальные и страницы-специфичные бейджи переопределяются независимо. Если false, добавляет к существующим бейджам. **/
        override?: boolean
        /** Если true, обновляет свойства отображения бейджей из конфигурации арендатора. **/
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

---