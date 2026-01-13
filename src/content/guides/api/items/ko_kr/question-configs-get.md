[api-resource-header-start name = 'QuestionConfig'; route = 'GET /api/v1/question-configs'; creditsCost = 1; api-resource-header-end]

이 경로는 한 번에 페이징된 최대 100개의 `QuestionConfig` 객체를 반환합니다. 비용은 100개당 1입니다. 그들은
질문 텍스트 오름차순(`question` 필드)으로 정렬됩니다.

[inline-code-attrs-start title = 'QuestionConfig 예제'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/question-configs?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'QuestionConfig 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionConfigRequestByIdQueryParams {
    tenantId: string
    API_KEY: string
    /** 페이징을 위해 사용합니다. 0부터 시작합니다. **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'QuestionConfig 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionConfigByIdResponse {
    status: 'success' | 'failed'
    /** 실패 시 포함됩니다. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** 실패 시 포함됩니다. **/
    reason?: string
    questionConfigs: QuestionConfig[]
}
[inline-code-end]

---