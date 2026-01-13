[api-resource-header-start name = 'EmailTemplateDefinition'; route = 'GET /api/v1/email-templates/definitions'; creditsCost = 1; api-resource-header-end]

Цей API дозволяє отримати всі об'єкти `EmailTemplateDefinition`.

[inline-code-attrs-start title = 'Приклад GET cURL для EmailTemplateDefinition'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/email-templates/definitions?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Структура GET-запиту EmailTemplateDefinition'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface GetEmailTemplateDefinitionsRequestQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура відповіді GET для EmailTemplateDefinition'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplateDefinition {
    name: string
    description: string
    /** Не всі шаблони пов'язані з доменом. Наприклад, деякі адміністративні електронні листи ніколи не надсилаються в контексті одного або декількох доменів. **/
    canBeDomainSpecific: boolean
    emailTemplateId: string
    defaultTestData: object
    defaultTranslationsByLocale: Record<string, Record<string, string>>
    defaultEJS: string
}

interface GetEmailTemplateDefinitionsResponse {
    status: 'success' | 'failed'
    /** Включається у разі помилки. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Включається у разі помилки. **/
    reason?: string
    definitions?: EmailTemplateDefinition[]
}
[inline-code-end]

---