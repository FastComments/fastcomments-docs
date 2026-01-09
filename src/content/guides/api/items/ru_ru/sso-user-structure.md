FastComments предоставляет простое в использовании SSO-решение. Обновление информации пользователя при интеграции на основе HMAC
так же просто, как загрузка пользователем страницы с обновлённым payload.

Тем не менее, может понадобиться управлять пользователем вне этого потока, чтобы повысить согласованность вашего приложения.

SSO User API предоставляет способ CRUD-операций с объектами, которые мы называем SSOUsers. Эти объекты отличаются от обычных Users и
хранятся отдельно для типобезопасности.

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

### Оплата SSO-пользователей

SSO-пользователи тарифицируются по-разному в зависимости от их флагов разрешений:

- **Regular SSO Users**: Пользователи без прав администратора или модератора тарифицируются как обычные SSO-пользователи
- **SSO Admins**: Пользователи с флагами `isAccountOwner` или `isAdminAdmin` тарифицируются отдельно как SSO Admins (по той же ставке, что и обычные администраторы тенанта)
- **SSO Moderators**: Пользователи с флагом `isCommentModeratorAdmin` тарифицируются отдельно как SSO Moderators (по той же ставке, что и обычные модераторы)

**Важно**: Чтобы предотвратить двойное выставление счетов, система автоматически дедуплицирует SSO-пользователей по отношению к обычным пользователям и модераторам тенанта по адресу электронной почты. Если у SSO-пользователя тот же email, что у обычного пользователя или модератора тенанта, он не будет оплачивать дважды.

### Управление доступом

Пользователей можно разделять на группы. Для этого и служит поле `groupIds`, и оно опционально.

### @Mentions

По умолчанию при вводе символа `@` поиск для `@mentions` использует `username` для поиска других sso-пользователей. Если используется `displayName`, то результаты, соответствующие
`username`, будут игнорироваться при наличии совпадения по `displayName`, и результаты поиска для `@mention` будут использовать `displayName`.

### Подписки

В FastComments пользователи могут подписаться на страницу, нажав на значок колокольчика в виджете комментариев и выбрав Subscribe.

Для обычного пользователя мы отправляем уведомления по электронной почте в соответствии с его настройками уведомлений.

Для SSO-пользователей мы разделяем это ради обратной совместимости. Дополнительные письма с уведомлениями о подписке будут отправляться только в том случае, если вы установите `optedInSubscriptionNotifications` в `true`.

### Значки

Вы можете назначать значки SSO-пользователям с помощью свойства `badgeConfig`. Значки — это визуальные индикаторы, которые отображаются рядом с именем пользователя в комментариях.

- `badgeIds` - Массив ID значков, которые нужно назначить пользователю. Они должны быть действительными ID значков, созданных в вашей учётной записи FastComments. Ограничение — до 30 значков.
- `override` - Если true, все существующие отображаемые значки в комментариях будут заменены на предоставленные. Если false или опущено, предоставленные значки будут добавлены к существующим.
- `update` - Если true, свойства отображения значков будут обновляться из конфигурации тенанта при каждом входе пользователя.