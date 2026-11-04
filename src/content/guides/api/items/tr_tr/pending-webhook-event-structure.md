Bir `PendingWebhookEvent` nesnesi, bekleyen kuyruğa alınmış bir webhook olayını temsil eder.

`PendingWebhookEvent` nesneleri otomatik olarak oluşturulur ve API üzerinden manuel olarak oluşturulamaz. Ayrıca bir yıl sonra son kullanma tarihine ulaşırlar.
Silinebilirler, bu da görevi kuyruktan kaldırır.

Farklı olay türleri vardır - `eventType` (`OutboundSyncEventType`) ve `type` (`OutboundSyncType`) alanlarını kontrol edin.

Bu API'nin yaygın bir kullanım senaryosu özel izleme uygulamaktır. Belirli filtreler için bekleyen sayıyı yoklamak amacıyla `/count` uç noktasını düzenli olarak çağırmak isteyebilirsiniz.

`PendingWebhookEvent` nesnesinin yapısı aşağıdaki gibidir:

[inline-code-attrs-start title = 'PendingWebhookEvent Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum OutboundSyncEventType {
    Create: 0,
    Delete: 1,
    Update: 2
}

enum OutboundSyncType {
    /** WordPress'e özel senkronizasyon görevi. **/
    WP: 0,
    Webhook: 1
}

interface PendingWebhookEvent {
    id: string
    /** Olayla ilişkili yorum kimliği. **/
    commentId: string
    /** Olay anındaki olaya ait yorum nesnesi. Bunları eklemeye Kasım 2023'te başladık. **/
    comment: Comment
    /** Yorumla ilişkilendirilmiş olabilecek harici bir kimlik. **/
    externalId: string | null
    createdAt: Date
    tenantId: string
    attemptCount: number
    /** İlk denemeden önce ve her başarısızlıktan sonra ayarlanır. **/
    nextAttemptAt: Date
    /** Bunun bir oluşturma, silme veya güncelleme olayı olup olmadığı... **/
    eventType: OutboundSyncEventType
    /** Gerçekleştirilecek senkronizasyonun türü (WordPress, API çağrısı vb). **/
    type: OutboundSyncType
    /** Yorumla eşleşen alan adı. API anahtarını seçmek için bu alan adını kullanıyoruz. **/
    domain: string
    /** Oluşan son hata. Bu tür tiplenmemiştir ve olan her şeyin bir "dökümüdür". Genellikle statusCode, body ve bir headers haritası içeren bir nesne içerir. **/
    lastError: object | null
}
[inline-code-end]