[api-resource-header-start name = 'EmailTemplateDefinition'; route = 'GET /api/v1/email-templates/definitions'; creditsCost = 1; api-resource-header-end]

이 API는 모든 `EmailTemplateDefinition` 객체를 가져오는 기능을 제공합니다.

[inline-code-attrs-start title = 'EmailTemplateDefinition GET cURL 예제'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/email-templates/definitions?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'EmailTemplateDefinition GET 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface GetEmailTemplateDefinitionsRequestQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'EmailTemplateDefinition GET 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplateDefinition {
    name: string
    description: string
    /** 모든 템플릿이 도메인 관련인 것은 아닙니다. 예를 들어, 일부 관리자 이메일은 하나 이상의 도메인 컨텍스트에서 전송되지 않습니다. **/
    canBeDomainSpecific: boolean
    emailTemplateId: string
    defaultTestData: object
    defaultTranslationsByLocale: Record<string, Record<string, string>>
    defaultEJS: string
}

interface GetEmailTemplateDefinitionsResponse {
    status: 'success' | 'failed'
    /** 실패 시 포함됩니다. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** 실패 시 포함됩니다. **/
    reason?: string
    definitions?: EmailTemplateDefinition[]
}
[inline-code-end]

---