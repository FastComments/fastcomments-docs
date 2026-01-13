[api-resource-header-start name = 'EmailTemplate'; route = 'GET /api/v1/email-templates'; creditsCost = 1; api-resource-header-end]

Bu API, `page` sorgu parametresiyle sağlanan sayfalandırma kullanır. EmailTemplate'ler, `createdAt` sonra `id` sırasına göre, her sayfada `100` öğe olacak şekilde döndürülür.

[inline-code-attrs-start title = 'EmailTemplate cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/email-templates?tenantId=demo&page=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'EmailTemplate İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplatesRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Alınacak sayfa, 0'dan başlar. **/
    page: number
}
[inline-code-end]

[inline-code-attrs-start title = 'EmailTemplate Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplatesResponse {
    status: 'success' | 'failed'
    /** Başarısızlık durumunda dahil edilir. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Başarısızlık durumunda dahil edilir. **/
    reason?: string
    emailTemplates: EmailTemplate[]
}
[inline-code-end]