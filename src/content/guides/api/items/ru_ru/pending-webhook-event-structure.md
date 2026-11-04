Объект `PendingWebhookEvent` представляет событие вебхука в очереди, ожидающее обработки.

Объекты `PendingWebhookEvent` создаются автоматически и не могут быть созданы вручную через API. Они также истекают через один год.
Их можно удалить, что уберет задачу из очереди.

Существуют разные типы событий — проверьте `eventType` (`OutboundSyncEventType`) и `type` (`OutboundSyncType`).

Распространенный вариант использования этого API — реализация пользовательского мониторинга. Возможно, вы захотите периодически вызывать эндпоинт `/count`,
чтобы опрашивать количество ожидающих событий по заданным фильтрам.

Структура объекта `PendingWebhookEvent` выглядит следующим образом:

[inline-code-attrs-start title = 'Структура PendingWebhookEvent'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum OutboundSyncEventType {
    Create: 0,
    Delete: 1,
    Update: 2
}

enum OutboundSyncType {
    /** Задача синхронизации, специфичная для WordPress. **/
    WP: 0,
    Webhook: 1
}

interface PendingWebhookEvent {
    id: string
    /** Идентификатор комментария, связанного с событием. **/
    commentId: string
    /** Объект комментария на момент события. Мы начали добавлять это в ноябре 2023 года. **/
    comment: Comment
    /** Внешний идентификатор, который может быть связан с комментарием. **/
    externalId: string | null
    createdAt: Date
    tenantId: string
    attemptCount: number
    /** Устанавливается перед первой попыткой и после каждой неудачи. **/
    nextAttemptAt: Date
    /** Является ли это событием создания, удаления или обновления... **/
    eventType: OutboundSyncEventType
    /** Тип синхронизации для выполнения (WordPress, вызов API и т.д.). **/
    type: OutboundSyncType
    /** Домен, который совпал с комментарием. Мы используем этот домен для выбора API-ключа. **/
    domain: string
    /** Последняя произошедшая ошибка. Этот тип не типизирован и представляет собой "дамп" того, что произошло. Обычно содержит объект с statusCode, body и картой headers. **/
    lastError: object | null
}
[inline-code-end]