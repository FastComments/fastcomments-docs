[api-resource-header-start name = 'QuestionConfig'; route = 'GET /api/v1/question-configs'; creditsCost = 1; api-resource-header-end]

Deze route retourneert tot 100 `QuestionConfig`-objecten tegelijk, gepagineerd. De kosten zijn 1 per 100 objecten. Ze worden
gesorteerd op oplopende vraagtekst (veld `question`).

[inline-code-attrs-start title = 'Voorbeeld van QuestionConfig'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/question-configs?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'QuestionConfig Verzoekstructuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionConfigRequestByIdQueryParams {
    tenantId: string
    API_KEY: string
    /** Voor paginering. Begint bij 0. **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'QuestionConfig Responsstructuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionConfigByIdResponse {
    status: 'success' | 'failed'
    /** Opgenomen bij mislukking. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Opgenomen bij mislukking. **/
    reason?: string
    questionConfigs: QuestionConfig[]
}
[inline-code-end]

---