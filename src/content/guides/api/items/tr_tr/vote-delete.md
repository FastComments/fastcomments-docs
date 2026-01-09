[api-resource-header-start name = 'Vote'; route = 'DELETE /api/v1/votes/:id'; creditsCost = 1; api-resource-header-end]

Bu rota tek bir `Vote`'u silme imkanı sağlar.

[inline-code-attrs-start title = 'Vote Silme cURL Örneği'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/votes/some-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Vote Silme İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VoteDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Vote Silme Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VoteDeleteResponse {
    status: 'success' | 'failed'
    /** Hata durumunda eklenir. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id'
    /** Hata durumunda eklenir. **/
    reason?: string
}
[inline-code-end]

Notlar:

- Bu API, kiracı düzeyindeki ayarlara uyar. Örneğin, belirli bir sayfa için oylamayı devre dışı bırakırsanız ve API üzerinden bir oy oluşturmaya çalışırsanız, `voting-disabled` hata kodu ile başarısız olur.
- Bu API varsayılan olarak canlıdır.
- Bu API, ilgili `Comment`'in `votes` değerini güncelleyecektir.