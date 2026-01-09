---
[api-resource-header-start name = 'Moderator'; route = 'POST /api/v1/moderators'; creditsCost = 1; api-resource-header-end]

Bu rota tek bir `Moderator` ekleme yeteneği sağlar.

Bir `Moderator` oluşturmanın aşağıdaki kısıtlamaları vardır:

- Bir `name` ve `email` her zaman sağlanmalıdır. Bir `userId` isteğe bağlıdır.
- Aşağıdaki değerler `Moderator` oluşturulurken sağlanamaz:
    - `acceptedInvite`
    - `markReviewedCount`
    - `deletedCount`
    - `markedSpamCount`
    - `approvedCount`
    - `editedCount`
    - `bannedCount`
    - `verificationId`
    - `createdAt`
- Bir `userId` belirtildiğinde, o kullanıcı mevcut olmalıdır.
- Bir `userId` belirtildiğinde, sorgu parametrelerinde belirtilen aynı `tenantId`'ye ait olmalıdır.
- Aynı tenant içindeki iki moderatör aynı `email` ile eklenemez.

Sadece e-postasını bildiğimiz bir kullanıcı için bir `Moderator` oluşturabiliriz:

[inline-code-attrs-start title = 'E-posta ile Moderator Oluşturma cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/moderators?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "name": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

Veya moderasyon istatistiklerini takip etmek için tenant'ımıza ait bir kullanıcı için bir `Moderator` oluşturabiliriz:

[inline-code-attrs-start title = 'Tenant Kullanıcısı ile Moderator Oluşturma cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/moderators?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "name": "Some Name",
	"email": "someone@someone.com",
    "userId": "some-tenant-user-id"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Moderator Oluşturma İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Moderator Oluşturma Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorPostResponse {
    status: 'success' | 'failed'
    /** Başarısızlık durumunda dahil edilir. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'name-required' | 'email-required' | 'unexpected-param' | 'not-found'
    /** Başarısızlık durumunda dahil edilir. **/
    reason?: string
    moderator?: Moderator; // Başarılı olduğunda oluşturulan moderatorun tamamını döndürürüz.
}
[inline-code-end]

---