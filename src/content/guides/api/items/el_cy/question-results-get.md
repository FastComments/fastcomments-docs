[api-resource-header-start name = 'QuestionResults'; route = 'GET /api/v1/question-results'; creditsCost = 1; api-resource-header-end]

Αυτή η διαδρομή επιστρέφει έως 1000 αντικείμενα `QuestionResults` τη φορά, με σελιδοποίηση. Το κόστος είναι 1 ανά κάθε 100 αντικείμενα. Είναι
ταξινομημένα κατά `createdAt`, αύξουσα. Μπορείτε να φιλτράρετε με διάφορες παραμέτρους.

[inline-code-attrs-start title = 'Παράδειγμα QuestionResults'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/question-results?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Αιτήματος QuestionResults'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Δομή Απάντησης QuestionResults'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
