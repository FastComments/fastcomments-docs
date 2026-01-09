[api-resource-header-start name = 'QuestionConfig'; route = 'GET /api/v1/question-configs'; creditsCost = 1; api-resource-header-end]

Bu rota sayfalandırılmış şekilde tek seferde en fazla 100 `QuestionConfig` nesnesi döndürür. Maliyet 100 nesne başına 1'dir. Bunlar
soru metnine göre artan sırada sıralanırlar (`question` alanı).

[inline-code-attrs-start title = 'QuestionConfig Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/question-configs?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'QuestionConfig İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionConfigRequestByIdQueryParams {
    tenantId: string
    API_KEY: string
    /** Sayfalama için. 0'dan başlar. **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'QuestionConfig Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionConfigByIdResponse {
    status: 'success' | 'failed'
    /** Başarısızlık durumunda dahil edilir. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Başarısızlık durumunda dahil edilir. **/
    reason?: string
    questionConfigs: QuestionConfig[]
}
[inline-code-end]

---