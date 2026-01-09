[api-resource-header-start name = 'EmailTemplate'; route = 'PATCH /api/v1/email-templates/:id'; creditsCost = 1; api-resource-header-end]

Этот API-эндпоинт позволяет обновить шаблон письма, указав только id и атрибуты для обновления.

Обратите внимание, что те же проверки, что применяются при создании шаблона, также действуют и при обновлении, например:

- Шаблон должен корректно рендериться. Это проверяется при каждом обновлении.
- Нельзя иметь дублирующиеся шаблоны для одного и того же домена (в противном случае один из них будет тихо проигнорирован).

[inline-code-attrs-start title = 'Пример cURL-запроса PATCH для EmailTemplate'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/email-templates/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"displayName": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запроса PATCH для EmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplatePatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа PATCH для EmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface EmailTemplatePatchResponse {
    status: 'success' | 'failed'
    /** Включается при ошибке. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'empty-request' | 'internal' | 'invalid-input' | 'not-found' | 'unexpected-param' | 'invalid-email-template-id' | 'unauthorized' | 'domain-invalid' | 'duplicate' | 'does-not-render';  
    /** Включается при ошибке. **/
    reason?: string
    /** Обновлённый шаблон письма. **/
    emailTemplate?: EmailTemplate
}
[inline-code-end]

---