`User` — це об'єкт, що представляє найбільш загальний знаменник усіх користувачів.

Майте на увазі, що в FastComments у нас є кілька різних сценаріїв використання для користувачів:

- Безпечний SSO
- Простий SSO
- Користувачі орендаря (наприклад: Адміністратори)
- Коментатори

Цей API призначений для **Коментаторів** та користувачів, створених через **Простий SSO**. Насправді будь-який користувач, створений через ваш сайт, може бути отриманий через цей API. Користувачів орендаря також можна отримати цим способом, але ви отримаєте більше інформації, взаємодіючи з `/tenant-users/` API.

Для `Secure SSO` будь ласка використовуйте `/sso-users/` API.

Ви не можете оновлювати ці типи користувачів. Вони створили свій акаунт через ваш сайт, тому ми надаємо базовий доступ лише для читання, але ви не можете вносити зміни. Якщо ви хочете мати такий тип потоку — вам потрібно налаштувати `Secure SSO`.

Структура об'єкта `User` наступна:

[inline-code-attrs-start title = 'Структура об’єкта User'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface User {
    /** Це також id, який використовується як userId в об'єктах коментарів. **/
    id: string
    username: string
    /** Наприклад, посилання на блог коментатора. **/
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