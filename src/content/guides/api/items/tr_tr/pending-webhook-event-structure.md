A `PendingWebhookEvent` object represents a queued webhook event that is pending.

`PendingWebhookEvent` objects are created automatically and cannot be manually created via the API. They also expire after one year.
They can be deleted which removes the task from the queue.

There are different event types - check `eventType` (`OutboundSyncEventType`) and `type` (`OutboundSyncType`).

A common use case for this API is to implement custom monitoring. You may want to call the `/count` endpoint periodically
to poll the outstanding count for given filters.

The structure for the `PendingWebhookEvent` object is as follows:

[inline-code-attrs-start title = 'PendingWebhookEvent Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum OutboundSyncEventType {
    Create: 0,
    Delete: 1,
    Update: 2
}

enum OutboundSyncType {
    /** WordPress'e özgü senkronizasyon görevi. **/
    WP: 0,
    Webhook: 1
}

interface PendingWebhookEvent {
    id: string
    /** Olayla ilişkili yorum kimliği. **/
    commentId: string
    /** Olay anındaki yorum nesnesi. Bunları Kasım 2023'te eklemeye başladık. **/
    comment: Comment
    /** Yorumla ilişkilendirilebilecek harici bir kimlik. **/
    externalId: string | null
    createdAt: Date
    tenantId: string
    attemptCount: number
    /** İlk denemeden önce ve her hata sonrası ayarlanır. **/
    nextAttemptAt: Date
    /** Bunun bir oluşturma, silme veya güncelleme olayı olup olmadığı... **/
    eventType: OutboundSyncEventType
    /** Yapılacak senkronizasyon türü (WordPress, API çağrısı, vb.). **/
    type: OutboundSyncType
    /** Yorumla eşleşen alan adı. API anahtarını seçmek için bu alan adını kullanıyoruz. **/
    domain: string
    /** Oluşan son hata. Bu tür tiplenmemiştir ve olanların bir "dump"ıdır. Genellikle statusCode, body ve bir headers haritası içeren bir nesne içerir. **/
    lastError: object | null
}
[inline-code-end]

---