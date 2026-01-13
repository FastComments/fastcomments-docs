Објекат `PendingWebhookEvent` представља webhook догађај који је стављен у ред и налази се на чекању.

`PendingWebhookEvent` објекти се креирају аутоматски и не могу се ручно креирати преко API-ја. Такође истичу након једне године.
Могу се обрисати, што уклања задатак из реда.

Постоје различити типови догађаја - проверите `eventType` (`OutboundSyncEventType`) и `type` (`OutboundSyncType`).

Чест случај употребе овог API-ја је имплементација прилагођеног надгледања. Можда ћете желети да периодично позивате крајњу тачку `/count`
да бисте упитали преостали број за дате филтере.

Структура објекта `PendingWebhookEvent` је следећа:

[inline-code-attrs-start title = 'Структура PendingWebhookEvent'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum OutboundSyncEventType {
    Create: 0,
    Delete: 1,
    Update: 2
}

enum OutboundSyncType {
    /** Синхронизациона задача специфична за WordPress. **/
    WP: 0,
    Webhook: 1
}

interface PendingWebhookEvent {
    id: string
    /** ИД коментара повезан са догађајем. **/
    commentId: string
    /** Објекат коментара за догађај у тренутку догађаја. Почели смо да их додајемо у новембру 2023. **/
    comment: Comment
    /** Спољни id који може бити повезан са коментаром. **/
    externalId: string | null
    createdAt: Date
    tenantId: string
    attemptCount: number
    /** Постављено пре првог покушаја, и након сваког неуспеха. **/
    nextAttemptAt: Date
    /** Да ли је ово догађај креирања, брисања или ажурирања... **/
    eventType: OutboundSyncEventType
    /** Тип синхронизације који треба извршити (WordPress, позив API-ја, итд). **/
    type: OutboundSyncType
    /** Домен који се поклапа са коментаром. Користимо овај домен за избор API кључа. **/
    domain: string
    /** Последња грешка која се догодила. Овај тип није типизиран и представља „дамп“ онога што се десило. Обично садржи објекат са statusCode, body, и мапом headers. **/
    lastError: object | null
}
[inline-code-end]

---