[api-resource-header-start name = 'QuestionResults'; route = 'GET /api/v1/question-results'; creditsCost = 1; api-resource-header-end]

이 라우트는 한 번에 최대 1000개의 `QuestionResults` 객체를 페이지네이션된 형태로 반환합니다. 비용은 100개당 1입니다. 결과는 `createdAt` 기준 오름차순으로 정렬됩니다. 다양한 매개변수로 필터링할 수 있습니다.

[inline-code-attrs-start title = 'QuestionResults 예시'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/question-results?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'QuestionResults 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** 페이지네이션용. 0부터 시작합니다. **/
    skip?: number
    /** 페이지네이션용. **/
    limit?: number
    /** 특정 페이지의 결과를 가져옵니다. **/
    urlId?: string
    /** 특정 사용자의 결과를 가져옵니다. **/
    userId?: string
    startDate?: string | number
    questionId?: string | string[]
}
[inline-code-end]

[inline-code-attrs-start title = 'QuestionResults 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsResponse {
    status: 'success' | 'failed'
    /** 실패 시 포함됩니다. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** 실패 시 포함됩니다. **/
    reason?: string
    questionResults: QuestionResults[]
}
[inline-code-end]