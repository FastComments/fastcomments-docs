[api-resource-header-start name = 'EmailTemplate'; route = 'PATCH /api/v1/email-templates/:id'; creditsCost = 1; api-resource-header-end]

Ten punkt końcowy API umożliwia aktualizację szablonu e-mail poprzez podanie jedynie id oraz atrybutów do zaktualizowania.

Zauważ, że wszystkie te same walidacje, które obowiązują przy tworzeniu szablonu, mają zastosowanie również tutaj, na przykład:

- Szablon musi się renderować. Jest to sprawdzane przy każdej aktualizacji.
- Nie można mieć zduplikowanych szablonów dla tej samej domeny (w przeciwnym razie jeden zostałby zignorowany bez komunikatu).

[inline-code-attrs-start title = 'Przykład cURL dla EmailTemplate PATCH'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/email-templates/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"displayName": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura żądania PATCH EmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplatePatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odpowiedzi PATCH EmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface EmailTemplatePatchResponse {
    status: 'success' | 'failed'
    /** Dołączone w przypadku niepowodzenia. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'empty-request' | 'internal' | 'invalid-input' | 'not-found' | 'unexpected-param' | 'invalid-email-template-id' | 'unauthorized' | 'domain-invalid' | 'duplicate' | 'does-not-render';  
    /** Dołączone w przypadku niepowodzenia. **/
    reason?: string
    /** Zaktualizowany szablon e-mail. **/
    emailTemplate?: EmailTemplate
}
[inline-code-end]