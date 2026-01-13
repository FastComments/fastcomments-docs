Объект `Subscription` представляет подписку для пользователя.

`Subscription` объекты создаются, когда пользователь нажимает значок уведомлений в виджете комментариев и нажимает "Подписаться на эту страницу".

Подписки также можно создавать через API.

Наличие объекта `Subscription` вызывает создание объектов `Notification` и отправку электронных писем, когда на корне связанной страницы
появляются новые комментарии, для которой предназначена подписка `Subscription`. Отправка писем зависит от типа пользователя. Для обычных пользователей это зависит от `optedInNotifications`. Для SSO-пользователей это зависит от `optedInSubscriptionNotifications`. Обратите внимание, что в некоторых приложениях может отсутствовать понятие веб-доступной страницы, в этом случае просто установите `urlId` равным
идентификатору элемента, на который вы подписываетесь (то же значение для `urlId`, которое вы передали бы в виджет комментариев).

Структура объекта `Subscription` выглядит следующим образом:

[inline-code-attrs-start title = 'Структура Subscription'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Subscription {
    id: string
    tenantId: string
    /** При SSO идентификатор пользователя имеет формат `<tenant id>:<user id>`. **/
    userId: string
    anonUserId?: string
    urlId: string
    url?: string
    pageTitle?: string
    createdAt: string // строка с датой
}
[inline-code-end]

---