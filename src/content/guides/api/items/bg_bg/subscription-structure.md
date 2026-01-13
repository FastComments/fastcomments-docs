Обектът `Subscription` представлява абонамент за потребител.

Обектите `Subscription` се създават, когато потребител кликне върху камбанката за известия в уиджета за коментари и щракне върху "Абониране за тази страница".

Абонаментите могат да бъдат създадени и чрез API.

Наличието на обект `Subscription` причинява генерирането на обекти `Notification` и изпращането на имейли, когато нови коментари са оставени в корена на асоциираната страница,
за която е `Subscription`. Изпращането на имейли зависи от типа потребител. За обикновени потребители това зависи от `optedInNotifications`. За SSO потребители това зависи от `optedInSubscriptionNotifications`. Обърнете внимание, че някои приложения може да нямат концепцията за уеб достъпна страница, в този случай просто задайте `urlId` на
id на елемента, за който се абонирате (същата стойност за `urlId`, която бихте подали на уиджета за коментари).

Структурата на обекта `Subscription` е следната:

[inline-code-attrs-start title = 'Структура на Subscription'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
