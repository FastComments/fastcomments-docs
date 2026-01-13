Об'єкт `Subscription` представляє підписку для користувача.

`Subscription` objects are created when a user clicks the notification bell in the comment widget and clicks "Підписатися на цю сторінку".

Підписки також можна створювати через API.

Наявність об'єкта `Subscription` призводить до створення об'єктів `Notification` і надсилання електронних листів, коли на корені пов'язаної сторінки, для якої призначена підписка, з'являються нові коментарі. Надсилання електронних листів залежить від типу користувача. Для звичайних користувачів це залежить від `optedInNotifications`. Для SSO-користувачів це залежить від `optedInSubscriptionNotifications`. Зверніть увагу, що деякі застосунки можуть не мати поняття веб-доступної сторінки; у такому випадку просто встановіть `urlId` в id елемента, на який ви підписуєтесь (те ж значення `urlId`, яке ви передали б віджету коментарів).

Структура об'єкта `Subscription` виглядає так:

[inline-code-attrs-start title = 'Структура Subscription'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Subscription {
    id: string
    tenantId: string
    /** У SSO ідентифікатор користувача має формат `<tenant id>:<user id>`. **/
    userId: string
    anonUserId?: string
    urlId: string
    url?: string
    pageTitle?: string
    createdAt: string // рядок дати
}
[inline-code-end]

---