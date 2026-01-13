---
A `PendingWebhookEvent` object represents a queued webhook event that is pending.

`PendingWebhookEvent` objects are created automatically and cannot be manually created via the API. They also expire after one year.
They can be deleted which removes the task from the queue.

There are different event types - check `eventType` (`OutboundSyncEventType`) and `type` (`OutboundSyncType`).

A common use case for this API is to implement custom monitoring. You may want to call the `/count` endpoint periodically
to poll the outstanding count for given filters.

The structure for the `PendingWebhookEvent` object is as follows:

[inline-code-attrs-start title = 'Структура PendingWebhookEvent'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum OutboundSyncEventType {
    Create: 0,
    Delete: 1,
    Update: 2
}

enum OutboundSyncType {
    /** Синхронизационная задача, специфичная для WordPress. **/
    WP: 0,
    Webhook: 1
}

interface PendingWebhookEvent {
    id: string
    /** Идентификатор комментария, связанный с событием. **/
    commentId: string
    /** Объект комментария для события на момент его возникновения. Мы начали добавлять их в ноябре 2023 г. **/
    comment: Comment
    /** Внешний идентификатор, который может быть связан с комментарием. **/
    externalId: string | null
    createdAt: Date
    tenantId: string
    attemptCount: number
    /** Устанавливается перед первой попыткой и после каждой неудачи. **/
    nextAttemptAt: Date
    /** Тип события: создание, удаление или обновление... **/
    eventType: OutboundSyncEventType
    /** Тип синхронизации для выполнения (WordPress, вызов API, и т.д.). **/
    type: OutboundSyncType
    /** Домен, совпавший с комментарием. Мы используем этот домен для выбора API-ключа. **/
    domain: string
    /** Последняя произошедшая ошибка. Этот тип не типизирован и представляет собой "дамп" случившегося. Обычно он содержит объект с полями statusCode, body и картой headers. **/
    lastError: object | null
}
[inline-code-end]

---