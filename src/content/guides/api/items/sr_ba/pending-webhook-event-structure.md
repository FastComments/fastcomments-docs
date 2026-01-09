---
Објекат `PendingWebhookEvent` представља webhook догађај који је стављен у ред и чека.

`PendingWebhookEvent` објекти се аутоматски креирају и не могу се ручно креирати преко API-ја. Такође истичу након једне године.
Могу се обрисати чиме се задатак уклања из реда.

Постоје различите врсте догађаја — проверите `eventType` (`OutboundSyncEventType`) и `type` (`OutboundSyncType`).

Чест случај употребе овог API-ја је имплементација прилагођеног надзора. Можда ћете желети повремено позивати `/count` ендпоинт
да бисте периодично провјеравали број преосталих задатака за одређене филтере.

Структура објекта `PendingWebhookEvent` је следећа:

[inline-code-attrs-start title = 'Структура PendingWebhookEvent'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum OutboundSyncEventType {
    Create: 0,
    Delete: 1,
    Update: 2
}

enum OutboundSyncType {
    /** Задатак синхронизације специфичан за WordPress. **/
    WP: 0,
    Webhook: 1
}

interface PendingWebhookEvent {
    id: string
    /** ИД коментара повезан са догађајем. **/
    commentId: string
    /** Објекат коментара за догађај у тренутку догађаја. Почели смо да их додајемо у новембру 2023. **/
    comment: Comment
    /** Спољни ИД који може бити повезан са коментаром. **/
    externalId: string | null
    createdAt: Date
    tenantId: string
    attemptCount: number
    /** Поставља се пре првог покушаја и након сваког неуспеха. **/
    nextAttemptAt: Date
    /** Да ли је ово догађај креирања, брисања или ажурирања... **/
    eventType: OutboundSyncEventType
    /** Врста синхронизације која ће се извршити (WordPress, позив API-ја, итд). **/
    type: OutboundSyncType
    /** Домен који је одговарао коментару. Користимо овај домен да изаберемо API кључ. **/
    domain: string
    /** Последња грешка која се десила. Овај тип није типизиран и представља "dump" онога што се десило. Обично садржи објекат са statusCode, body, и мапом headers. **/
    lastError: object | null
}
[inline-code-end]

---