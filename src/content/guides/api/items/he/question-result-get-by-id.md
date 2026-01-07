[api-resource-header-start name = 'QuestionResult'; route = 'GET /api/v1/question-results/:id'; creditsCost = 1; api-resource-header-end]

נתיב זה מחזיר `QuestionResult` יחיד לפי המזהה שלו.

[inline-code-attrs-start title = 'דוגמת cURL ל-QuestionResult לפי מזהה'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/question-results/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'מבנה בקשת QuestionResult'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultRequestByIdQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'מבנה תגובת QuestionResult'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultByIdResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found'
    /** Included on failure. **/
    reason?: string
    questionResult?: QuestionResult
}
[inline-code-end]
