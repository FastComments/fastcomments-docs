[api-resource-header-start name = 'Moderator'; route = 'PATCH /api/v1/moderators/:id'; creditsCost = 1; api-resource-header-end]

Bu API uç noktası, bir `Moderator`'ı `id` ile güncelleme olanağı sağlar.

Bir `Moderator`'ın güncellenmesi aşağıdaki kısıtlamalara tabidir:

- Bir `Moderator` güncellenirken, aşağıdaki değerler sağlanamaz:
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
- Bir `userId` belirtildiğinde, sorgu parametrelerinde belirtilen aynı `tenantId`'e ait olmalıdırlar.
- Aynı tenant içinde iki moderator aynı `email` ile eklenemez.
- Bir `Moderator` ile ilişkili `tenantId`'yi değiştiremezsiniz.

[inline-code-attrs-start title = 'Moderator PATCH cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/moderators/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"name": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'Moderator PATCH İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorPatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Moderator PATCH Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface ModeratorPatchResponse {
    status: 'success' | 'failed'
    /** Başarısızlık halinde dahil edilir. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'name-required' | 'email-required' | 'unexpected-param' | 'not-found';  
    /** Başarısızlık halinde dahil edilir. **/
    reason?: string
}
[inline-code-end]