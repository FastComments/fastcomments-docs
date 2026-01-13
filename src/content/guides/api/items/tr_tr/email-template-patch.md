[api-resource-header-start name = 'EmailTemplate'; route = 'PATCH /api/v1/email-templates/:id'; creditsCost = 1; api-resource-header-end]

Bu API uç noktası, yalnızca id'yi ve güncellenecek özellikleri belirterek bir e-posta şablonunu güncelleme olanağı sağlar.

Bir şablon oluştururken uygulanan aynı doğrulamalar da geçerlidir, örneğin:

- Şablonun render olması gerekir. Bu, her güncellemede kontrol edilir.
- Aynı alan adı için çoğaltılmış şablonlara sahip olamazsınız (aksi takdirde biri sessizce yoksayılır).

[inline-code-attrs-start title = 'EmailTemplate PATCH cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/email-templates/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"displayName": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'EmailTemplate PATCH İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplatePatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'EmailTemplate PATCH Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface EmailTemplatePatchResponse {
    status: 'success' | 'failed'
    /** Başarısızlık durumunda dahil edilir. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'empty-request' | 'internal' | 'invalid-input' | 'not-found' | 'unexpected-param' | 'invalid-email-template-id' | 'unauthorized' | 'domain-invalid' | 'duplicate' | 'does-not-render';  
    /** Başarısızlık durumunda dahil edilir. **/
    reason?: string
    /** Güncellenmiş e-posta şablonu. **/
    emailTemplate?: EmailTemplate
}
[inline-code-end]

---