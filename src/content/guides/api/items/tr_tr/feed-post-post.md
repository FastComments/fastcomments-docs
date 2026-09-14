[api-resource-header-start name = 'FeedPost'; route = 'POST /api/v1/feed-posts'; creditsCost = 1; api-resource-header-end]

Bu yol tek bir `FeedPost` oluşturur. Her gönderinin bir yazarı vardır, bu yüzden `fromUserId` gereklidir ve hesabınızdaki mevcut bir FastComments veya SSO kullanıcısının kimliği olmalıdır.

[inline-code-attrs-start title = 'FeedPost Oluşturma cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/feed-posts?tenantId=demo&isLive=true&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "fromUserId": "some-user-id",
    "title": "Release 2.0 is out",
    "contentHTML": "<p>Read the notes and tell us what you think.</p>",
    "tags": ["releases"],
    "links": [
        {
            "url": "https://example.com/releases/2.0",
            "title": "Release notes",
            "description": "Everything that changed in 2.0."
        }
    ]
}'
[inline-code-end]

[inline-code-attrs-start title = 'FeedPost Oluşturma İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostPostQueryParams {
    tenantId: string
    API_KEY: string
    /** Gönderiyi şu anda bir tarayıcıda açık olan beslemelere gönder. Varsayılan değer false. **/
    isLive?: boolean
    /** Gönderiyi kaydetmeden önce spam motorundan geçir. Varsayılan değer false. **/
    doSpamCheck?: boolean
    /** doSpamCheck'in bir parçası olarak çalışan yinelenen içerik kontrolünü atla. Varsayılan değer false. **/
    skipDupCheck?: boolean
    /** En fazla 256 karakter. Canlı dinleyicilere yankılanır, böylece bir istemci kendi yayınına göz ardı edebilir. **/
    broadcastId?: string
}

interface FeedPostPostBody {
    /** Gerekli. Bir FastComments veya SSO kullanıcı kimliği. **/
    fromUserId: string
    title?: string
    /** HTML. Kaydedilirken temizlenir. **/
    contentHTML?: string
    /** Kullanıcıdan alınan görüntüleme adını geçersiz kılar. **/
    fromUserDisplayName?: string
    tags?: string[]
    media?: FeedPostMediaItem[]
    links?: FeedPostLink[]
    meta?: Record<string, string>
}
[inline-code-end]

[inline-code-attrs-start title = 'FeedPost Oluşturma Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostPostResponse {
    status: 'success' | 'failed'
    /** Başarısızlıkta dahil edilir. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-from-user-id' | 'invalid-user' | 'broadcast-id-too-long' | 'spam-blocked' | 'user-rate-limited' | 'internal'
    /** Başarısızlıkta dahil edilir. **/
    reason?: string
    feedPost?: FeedPost; // Başarı durumunda oluşturulan tam gönderiyi döneriz.
}
[inline-code-end]