Објекат `PendingWebhookEvent` представља ставку вебхук догађаја у реду која је на чекању.

Објекти `PendingWebhookEvent` се аутоматски креирају и не могу бити ручно креирани преко API-ја. Такође истичу након једне године.
Могу бити обрисани, што уклања задатак из реда.

Постоје различити типови догађаја — провјерите `eventType` (`OutboundSyncEventType`) и `type` (`OutboundSyncType`).

Чест случај коришћења овог API-ја је имплементација прилагођеног надзора. Можда ћете желети да периодично позивате `/count` крајњу тачку
да бисте провјерили број нерешених ставки за задате филтере.

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
    /** Објекат коментара за догађај у тренутку догађаја. Почели смо да додамо ово у новембру 2023. **/
    comment: Comment
    /** Спољни ид који може бити повезан са коментаром. **/
    externalId: string | null
    createdAt: Date
    tenantId: string
    attemptCount: number
    /** Постављено пре првог покушаја и након сваког неуспјеха. **/
    nextAttemptAt: Date
    /** Да ли је ово догађај креирања, брисања или ажурирања... **/
    eventType: OutboundSyncEventType
    /** Тип синхронизације која ће се извршити (WordPress, позив API-ја, итд). **/
    type: OutboundSyncType
    /** Домен који је одговарао коментару. Користимо овај домен да изаберемо API кључ. **/
    domain: string
    /** Последња грешка која се десила. Овај тип није типизован и представља "dump" онога што се десило. Обично садржи објекат са statusCode, body, и мапом headers. **/
    lastError: object | null
}
[inline-code-end]

---