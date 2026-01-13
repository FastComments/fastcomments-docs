[api-resource-header-start name = 'QuestionResults'; route = 'GET /api/v1/question-results'; creditsCost = 1; api-resource-header-end]

此路由一次最多傳回 1000 個 `QuestionResults` 物件，分頁回傳。費用為每 100 個物件 1。
它們依 `createdAt` 升序排序。您可以依各種參數進行篩選。

[inline-code-attrs-start title = 'QuestionResults 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/question-results?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'QuestionResults 請求結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** 用於分頁。從 0 開始。 **/
    skip?: number
    /** 用於分頁。 **/
    limit?: number
    /** 取得特定頁面的結果。 **/
    urlId?: string
    /** 取得特定使用者的結果。 **/
    userId?: string
    startDate?: string | number
    questionId?: string | string[]
}
[inline-code-end]

[inline-code-attrs-start title = 'QuestionResults 回應結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsResponse {
    status: 'success' | 'failed'
    /** 失敗時會包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** 失敗時會包含。 **/
    reason?: string
    questionResults: QuestionResults[]
}
[inline-code-end]