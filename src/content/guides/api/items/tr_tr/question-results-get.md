[api-resource-header-start name = 'QuestionResults'; route = 'GET /api/v1/question-results'; creditsCost = 1; api-resource-header-end]

Bu rota sayfalandırılmış olarak aynı anda en fazla 1000 `QuestionResults` nesnesi döndürür. Ücret her 100 nesne için 1'dir. Onlar
`createdAt`'e göre artan sırada sıralanırlar. Çeşitli parametrelere göre filtreleyebilirsiniz.

[inline-code-attrs-start title = 'QuestionResults Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/question-results?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'QuestionResults İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Sayfalandırma için. 0'dan başlar. **/
    skip?: number
    /** Sayfalandırma için. **/
    limit?: number
    /** Belirli bir sayfadan sonuçları alır. **/
    urlId?: string
    /** Belirli bir kullanıcıdan sonuçları alır. **/
    userId?: string
    startDate?: string | number
    questionId?: string | string[]
}
[inline-code-end]

[inline-code-attrs-start title = 'QuestionResults Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsResponse {
    status: 'success' | 'failed'
    /** Başarısızlık halinde eklenir. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Başarısızlık halinde eklenir. **/
    reason?: string
    questionResults: QuestionResults[]
}
[inline-code-end]

---