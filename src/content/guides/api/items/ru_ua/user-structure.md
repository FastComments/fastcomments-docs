`User` — это объект, который представляет собой наибольший общий знаменатель всех пользователей.

Имейте в виду, что в FastComments у нас есть несколько разных сценариев использования пользователей:

- Secure SSO
- Simple SSO
- Tenant Users (For example: Administrators)
- Commenters

Этот API предназначен для **Комментаторов** и пользователей, созданных через **Simple SSO**. По сути, любой пользователь, созданный через ваш сайт, доступен через этот API. Пользователи арендатора также могут быть получены таким образом, но вы получите больше информации, взаимодействуя с API `/tenant-users/`.

Для `Secure SSO` используйте API `/sso-users/`.

Вы не можете обновлять эти типы пользователей. Они создали свои аккаунты через ваш сайт, поэтому мы предоставляем лишь базовый доступ только для чтения, но вы не можете вносить изменения. Если вы хотите иметь такой поток — вам нужно настроить `Secure SSO`.

Структура объекта `User` выглядит следующим образом:

[inline-code-attrs-start title = 'Структура User'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface User {
    /** Это также id, используемый как userId в объектах комментариев. **/
    id: string
    username: string
    /** Ссылка, например, на блог комментатора. **/
    websiteUrl?: string
    email: string
    signUpDate: number
    createdFromUrlId: string
    createdFromTenantId: string
    avatarSrc?: string
    locale: FastCommentsLocale
    displayLabel?: string
    karma?: number
}
[inline-code-end]

---