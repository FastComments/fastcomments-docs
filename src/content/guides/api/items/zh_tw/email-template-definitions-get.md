[api-resource-header-start name = 'EmailTemplateDefinition'; route = 'GET /api/v1/email-templates/definitions'; creditsCost = 1; api-resource-header-end]

此 API 可用於擷取所有 `EmailTemplateDefinition` 物件。

[inline-code-attrs-start title = 'EmailTemplateDefinition GET cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/email-templates/definitions?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'EmailTemplateDefinition GET 請求結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface GetEmailTemplateDefinitionsRequestQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'EmailTemplateDefinition GET 回應結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplateDefinition {
    name: string
    description: string
    /** 並非所有範本都與網域相關。例如，某些管理員電子郵件從未在一個或多個網域的情境中傳送。 **/
    canBeDomainSpecific: boolean
    emailTemplateId: string
    defaultTestData: object
    defaultTranslationsByLocale: Record<string, Record<string, string>>
    defaultEJS: string
}

interface GetEmailTemplateDefinitionsResponse {
    status: 'success' | 'failed'
    /** 於失敗時包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** 於失敗時包含。 **/
    reason?: string
    definitions?: EmailTemplateDefinition[]
}
[inline-code-end]