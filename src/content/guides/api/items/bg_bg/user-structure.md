`User` е обект, който представлява най-общ знаменател на всички потребители.

Имайте предвид, че във FastComments имаме множество различни случаи на употреба за потребители:

- Сигурно SSO
- Просто SSO
- Tenant потребители (Например: Администратори)
- Коментатори

Този API е за **Коментатори** и потребители, създадени чрез **Просто SSO**. По същество всеки потребител, създаден
чрез вашия сайт, може да бъде достъпен чрез този API. Tenant потребители също могат да бъдат извлечени по този начин, но ще получите повече информация, като взаимодействате с API `/tenant-users/`.

За `Сигурно SSO` моля използвайте API `/sso-users/`.

Не можете да актуализирате тези типове потребители. Те са създали акаунта си чрез вашия сайт, така че предоставяме някакъв базов достъп само за четене, но
не можете да правите промени. Ако искате да имате такъв тип поток - трябва да настроите `Сигурно SSO`.

Структурата на обекта `User` е следната:

[inline-code-attrs-start title = 'Структура на User'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface User {
    /** This is also the id used as userId on comment objects. **/
    id: string
    username: string
    /** A link to the commenter's blog, for example. **/
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
