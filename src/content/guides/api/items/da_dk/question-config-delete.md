[api-resource-header-start name = 'QuestionConfig'; route = 'DELETE /api/v1/question-configs/:id'; creditsCost = 100; api-resource-header-end]

Denne rute giver mulighed for at fjerne en `QuestionConfig` efter id.

**Dette vil slette alle tilsvarende spørgsmålsresultater (men ikke kommentarerne).** Dette er en del af den høje kreditomkostning.

[inline-code-attrs-start title = 'QuestionConfig Fjernelse cURL Eksempel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/question-configs/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'QuestionConfig Fjernelse Anmodningsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionConfigDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'QuestionConfig Fjernelse Svarstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionConfigResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
