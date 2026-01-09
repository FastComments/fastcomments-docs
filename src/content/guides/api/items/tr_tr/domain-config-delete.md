[api-resource-header-start name = 'DomainConfig'; route = 'DELETE /api/v1/domain-configs/:domain'; creditsCost = 1; api-resource-header-end]

Bu rota, tek bir `DomainConfig`'in id'sine göre kaldırılmasını sağlar.

- Not: Bir `DomainConfig`'in kaldırılması, o alan adının FastComments kullanma yetkisini kaldırır.
- Not: UI üzerinden bir alan adını yeniden eklemek nesneyi yeniden oluşturur (yalnızca `domain` alanı doldurulmuş olarak).

[inline-code-attrs-start title = 'DomainConfig Kaldırma cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/domain-configs/example.com?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'DomainConfig Kaldırma İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigRequestQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'DomainConfig Kaldırma Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigDeleteResponse {
    status: 'success' | 'failed'
    /** Başarısızlık halinde dahil edilir. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-domain' | 'domain-config-does-not-exist'
    /** Başarısızlık halinde dahil edilir. **/
    reason?: string
}
[inline-code-end]