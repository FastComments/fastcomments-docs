[api-resource-header-start name = 'EmailTemplate'; route = 'GET /api/v1/email-templates'; creditsCost = 1; api-resource-header-end]

이 API는 페이지 매김을 사용하며, `page` 쿼리 매개변수로 제공합니다. EmailTemplates는 `100`개씩 페이지로 반환되며, `createdAt`과 그 다음으로 `id`로 정렬됩니다.

[inline-code-attrs-start title = 'EmailTemplate cURL 예제'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/email-templates?tenantId=demo&page=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'EmailTemplate 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplatesRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** 가져올 페이지로, 0부터 시작합니다. **/
    page: number
}
[inline-code-end]

[inline-code-attrs-start title = 'EmailTemplate 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplatesResponse {
    status: 'success' | 'failed'
    /** 실패 시 포함됩니다. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** 실패 시 포함됩니다. **/
    reason?: string
    emailTemplates: EmailTemplate[]
}
[inline-code-end]

---