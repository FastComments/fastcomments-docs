---
Объект `Subscription` представляет подписку для пользователя.

`Subscription` объекты создаются, когда пользователь нажимает на колокольчик уведомлений в виджете комментариев и нажимает "Подписаться на эту страницу".

Подписки также можно создавать через API.

Наличие объекта `Subscription` приводит к генерации объектов `Notification` и отправке писем, когда в корне связанной страницы, на которую оформлена подписка, оставляются новые комментарии. Отправка писем зависит от типа пользователя. Для обычных пользователей это зависит от `optedInNotifications`. Для пользователей SSO это зависит от `optedInSubscriptionNotifications`. Обратите внимание, что в некоторых приложениях может отсутствовать понятие веб-доступной страницы, в таком случае просто установите `urlId` равным id элемента, на который вы подписываетесь (то же значение для `urlId`, которое вы передали бы виджету комментариев).

Структура объекта `Subscription` выглядит следующим образом:

[inline-code-attrs-start title = 'Структура объекта Subscription'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Subscription {
    id: string
    tenantId: string
    /** При использовании SSO, идентификатор пользователя имеет формат `<tenant id>:<user id>`. **/
    userId: string
    anonUserId?: string
    urlId: string
    url?: string
    pageTitle?: string
    createdAt: string // строковое представление даты
}
[inline-code-end]

---