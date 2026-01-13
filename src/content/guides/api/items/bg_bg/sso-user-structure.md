FastComments предоставя лесно за използване SSO решение. Актуализирането на информацията на потребител с интеграцията, базирана на HMAC, е
толкова просто, колкото потребителят да зареди страницата с актуализиран payload.

Въпреки това може да е желателно да управлявате потребител извън този поток, за да подобрите последователността на вашето приложение.

API за SSO потребители предоставя начин за CRUD на обекти, които наричаме SSOUsers. Тези обекти са различни от обикновените Users и
се съхраняват отделно за безопасност на типовете.

Структурата на обекта SSOUser е следната:

[inline-code-attrs-start title = 'Структура на SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

### Фактуриране за SSO потребители

SSO потребителите се таксуват различно въз основа на техните флагове за права:

- **Обикновени SSO потребители**: Потребители без администраторски или модераторски права се таксуват като обикновени SSO потребители
- **SSO администратори**: Потребители с флагове `isAccountOwner` или `isAdminAdmin` се таксуват отделно като SSO администратори (същата ставка като обикновените tenant администратори)
- **SSO модератори**: Потребители с флаг `isCommentModeratorAdmin` се таксуват отделно като SSO модератори (същата ставка като обикновените модератори)

**Важно**: За да се предотврати двойно таксуване, системата автоматично дедублира SSO потребители спрямо обикновени tenant потребители и модератори по имейл адрес. Ако SSO потребител има същия имейл като обикновен tenant потребител или модератор, той няма да бъде таксуван два пъти.

### Контрол на достъпа

Потребителите могат да бъдат разделени на групи. За това служи полето `groupIds` и е незадължително.

### @Споменавания

По подразбиране `@споменаванията` ще използват `username` за търсене на други sso потребители, когато се въведе символът `@`. Ако се използва `displayName`, тогава резултатите, съвпадащи с
`username`, ще бъдат игнорирани, когато има съвпадение за `displayName`, и резултатите от търсенето на `@споменаване` ще използват `displayName`.

### Абонаменти

С FastComments потребителите могат да се абонират за страница, като щракнат върху иконата на камбанката в уиджета за коментари и щракнат върху Абониране.

При обикновен потребител изпращаме имейли за известия въз основа на техните настройки за известия.

При SSO потребители разделяме това за обратна съвместимост. Потребителите ще получават тези допълнителни имейли за известия за абонамент
само ако зададете `optedInSubscriptionNotifications` на `true`.

### Значки

Можете да присвоявате значки на SSO потребители, като използвате свойството `badgeConfig`. Значките са визуални индикатори, които се появяват до името на потребителя в коментарите.

- `badgeIds` - Масив от ID на значки за присвояване на потребителя. Те трябва да бъдат валидни ID на значки, създадени във вашия акаунт в FastComments. Ограничено до 30 значки.
- `override` - Ако е true, всички съществуващи значки, показвани в коментарите, ще бъдат заменени с предоставените. Ако е false или е пропуснато, предоставените значки ще бъдат добавени към съществуващите значки.
- `update` - Ако е true, свойствата за показване на значки ще бъдат актуализирани от конфигурацията на tenant всеки път, когато потребителят влезе.
