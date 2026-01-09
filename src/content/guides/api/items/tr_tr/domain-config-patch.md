[api-resource-header-start name = 'DomainConfig'; route = 'PATCH /api/v1/domain-configs/:domain'; creditsCost = 1; api-resource-header-end]

Bu API uç noktası, yalnızca alan adını ve güncellenecek özniteliği belirterek bir alan yapılandırmasını güncelleme yeteneği sağlar.

[inline-code-attrs-start title = 'DomainConfig PATCH cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/domain-configs/example.com?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"emailFromName": "Some New From Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'DomainConfig PATCH İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigPatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'DomainConfig PATCH Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface DomainConfigPatchResponse {
    status: 'success' | 'failed'
    /** Başarısızlık durumunda dahil edilir. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'empty-request' | 'internal' | 'invalid-input' | 'missing-domain' | 'domain-does-not-exist' | 'update-would-create-duplicate' | 'domain-too-long' | 'domain-invalid';  
    /** Başarısızlık durumunda dahil edilir. **/
    reason?: string
    /** Güncellenmiş yapılandırma. **/
    configuration?: DomainConfig
}
[inline-code-end]

---