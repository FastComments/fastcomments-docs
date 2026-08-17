[api-resource-header-start name = 'EmailTemplate'; route = 'PATCH /api/v1/email-templates/:id'; creditsCost = 1; api-resource-header-end]

Цей API‑ендпоінт надає можливість оновити шаблон електронної пошти, вказавши лише id та атрибути, які потрібно оновити.

Зверніть увагу, що всі ті ж самі перевірки, які застосовуються при створенні шаблону, також діють, наприклад:

- Шаблон повинен рендеритися. Це перевіряється при кожному оновленні.
- Ви не можете мати дублікати шаблонів для одного домену (інакше один буде тихо проігнорований).

[inline-code-attrs-start title = 'Приклад cURL запиту PATCH EmailTemplate'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/email-templates/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"displayName": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запиту PATCH EmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplatePatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура відповіді PATCH EmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface EmailTemplatePatchResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'empty-request' | 'internal' | 'invalid-input' | 'not-found' | 'unexpected-param' | 'invalid-email-template-id' | 'unauthorized' | 'domain-invalid' | 'duplicate' | 'does-not-render';  
    /** Included on failure. **/
    reason?: string
    /** The updated email template. **/
    emailTemplate?: EmailTemplate
}
[inline-code-end]

---