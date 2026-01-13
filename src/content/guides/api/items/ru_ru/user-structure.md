`User` — объект, представляющий наибольшее общее для всех пользователей.

Имейте в виду, что в FastComments у нас есть множество различных сценариев использования пользователей:

- Secure SSO
- Simple SSO
- Tenant Users (Например: Администраторы)
- Commenters

Этот API предназначен для **Commenters** и пользователей, созданных через **Simple SSO**. По сути, любой пользователь, созданный через ваш сайт, доступен через этот API. Tenant Users также можно получить таким способом, но вы получите больше информации, взаимодействуя с API `/tenant-users/`.

Для `Secure SSO` используйте API `/sso-users/`.

Вы не можете обновлять таких пользователей. Они создали свою учётную запись через ваш сайт, поэтому мы предоставляем лишь базовый доступ только для чтения, и вы не можете вносить изменения. Если вы хотите реализовать такой сценарий — вам нужно настроить `Secure SSO`.

Структура объекта `User` выглядит следующим образом:

[inline-code-attrs-start title = 'Структура User'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface User {
    /** Это также идентификатор, используемый как userId в объектах комментариев. **/
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