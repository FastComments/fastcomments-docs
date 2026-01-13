[api-resource-header-start name = 'EmailTemplate'; route = 'PATCH /api/v1/email-templates/:id'; creditsCost = 1; api-resource-header-end]

Овај API крајњи пункт пружа могућност ажурирања шаблона е-поште навођењем само id и атрибута које треба ажурирати.

Имајте на уму да важе све исте валидације као при креирању шаблона, на пример:

- Шаблон мора да се рендерује. Ово се проверава при сваком ажурирању.
- Не можете имати дупликат шаблона за исти домен (иначе би један био тихо игнорисан).

[inline-code-attrs-start title = 'Пример cURL за EmailTemplate PATCH'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/email-templates/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"displayName": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'Структура захтева за EmailTemplate PATCH'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplatePatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура одговора за EmailTemplate PATCH'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface EmailTemplatePatchResponse {
    status: 'success' | 'failed'
    /** Укључено у случају неуспеха. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'empty-request' | 'internal' | 'invalid-input' | 'not-found' | 'unexpected-param' | 'invalid-email-template-id' | 'unauthorized' | 'domain-invalid' | 'duplicate' | 'does-not-render';  
    /** Укључено у случају неуспеха. **/
    reason?: string
    /** Ажурирани шаблон е-поште. **/
    emailTemplate?: EmailTemplate
}
[inline-code-end]

---