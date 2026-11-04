Об'єкт `Subscription` представляє підписку користувача.

Об'єкти `Subscription` створюються, коли користувач натискає на дзвіночок сповіщень у віджеті коментарів та обирає "Subscribe to this page".

Підписки також можна створювати через API.

Наявність об'єкта `Subscription` призводить до генерації об'єктів `Notification` та надсилання електронних листів, коли залишаються нові коментарі в корені пов'язаної сторінки,
для якої створено `Subscription`. Надсилання листів залежить від типу користувача. Для звичайних користувачів це залежить від `optedInNotifications`. Для SSO-користувачів це залежить від `optedInSubscriptionNotifications`. Зверніть увагу, що деякі застосунки можуть не мати концепції вебдоступної сторінки — у такому випадку просто встановіть `urlId` як
ідентифікатор елемента, на який ви підписуєтеся (те саме значення `urlId`, яке ви передали б у віджет коментарів).

Структура об'єкта `Subscription` виглядає наступним чином:

[inline-code-attrs-start title = 'Структура Subscription'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Subscription {
    id: string
    tenantId: string
    /** With SSO, the user id is in the format `<tenant id>:<user id>`. **/
    userId: string
    anonUserId?: string
    urlId: string
    url?: string
    pageTitle?: string
    createdAt: string // date string
}
[inline-code-end]