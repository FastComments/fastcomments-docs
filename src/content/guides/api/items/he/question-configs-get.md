[api-resource-header-start name = 'QuestionConfig'; route = 'GET /api/v1/question-configs'; creditsCost = 1; api-resource-header-end]

נתיב זה מחזיר עד 100 אובייקטי `QuestionConfig` בכל פעם, מעומדים. העלות היא 1 לכל 100 אובייקטים. הם
ממוינים לפי טקסט השאלה בסדר עולה (שדה `question`).

[inline-code-attrs-start title = 'דוגמת QuestionConfig'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/question-configs?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'מבנה בקשת QuestionConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionConfigRequestByIdQueryParams {
    tenantId: string
    API_KEY: string
    /** For pagination. Starts at 0. **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'מבנה תגובת QuestionConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionConfigByIdResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Included on failure. **/
    reason?: string
    questionConfigs: QuestionConfig[]
}
[inline-code-end]
