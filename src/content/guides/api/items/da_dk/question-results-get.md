[api-resource-header-start name = 'QuestionResults'; route = 'GET /api/v1/question-results'; creditsCost = 1; api-resource-header-end]

Denne rute returnerer op til 1000 `QuestionResults`-objekter ad gangen, pagineret. Prisen er 1 pr. hver 100 objekter. De er
sorteret efter `createdAt`, stigende. Du kan filtrere efter forskellige parametre.

[inline-code-attrs-start title = 'QuestionResults Eksempel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/question-results?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'QuestionResults Anmodningsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** For pagination. Starts at 0. **/
    skip?: number
    /** For pagination. **/
    limit?: number
    /** Get the results from a specific page. **/
    urlId?: string
    /** Get the results from a specific user. **/
    userId?: string
    startDate?: string | number
    questionId?: string | string[]
}
[inline-code-end]

[inline-code-attrs-start title = 'QuestionResults Svarstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Included on failure. **/
    reason?: string
    questionResults: QuestionResults[]
}
[inline-code-end]
