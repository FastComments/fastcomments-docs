[api-resource-header-start name = 'QuestionConfig'; route = 'GET /api/v1/question-configs'; creditsCost = 1; api-resource-header-end]

此路由會一次回傳最多 100 個 `QuestionConfig` 物件，並採用分頁。每 100 個物件的成本為 1。它們會
依照問題文字以升冪排序（`question` 欄位）。

[inline-code-attrs-start title = 'QuestionConfig 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/question-configs?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'QuestionConfig 請求結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionConfigRequestByIdQueryParams {
    tenantId: string
    API_KEY: string
    /** 用於分頁。從 0 開始。 **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'QuestionConfig 回應結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionConfigByIdResponse {
    status: 'success' | 'failed'
    /** 失敗時包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** 失敗時包含。 **/
    reason?: string
    questionConfigs: QuestionConfig[]
}
[inline-code-end]

---