[api-resource-header-start name = 'EmailTemplate'; route = 'PATCH /api/v1/email-templates/:id'; creditsCost = 1; api-resource-header-end]

Questo endpoint API fornisce la possibilità di aggiornare un template email specificando solo l'id e gli attributi da aggiornare.

Nota che tutte le stesse validazioni per la creazione di un template si applicano anche qui, per esempio:

- Il template deve poter essere renderizzato. Questo viene controllato ad ogni aggiornamento.
- Non puoi avere template duplicati per lo stesso dominio (altrimenti uno verrebbe ignorato silenziosamente).

[inline-code-attrs-start title = 'Esempio cURL PATCH EmailTemplate'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/email-templates/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"displayName": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struttura richiesta PATCH EmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplatePatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struttura risposta PATCH EmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface EmailTemplatePatchResponse {
    status: 'success' | 'failed'
    /** Incluso in caso di errore. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'empty-request' | 'internal' | 'invalid-input' | 'not-found' | 'unexpected-param' | 'invalid-email-template-id' | 'unauthorized' | 'domain-invalid' | 'duplicate' | 'does-not-render';  
    /** Incluso in caso di errore. **/
    reason?: string
    /** Il template email aggiornato. **/
    emailTemplate?: EmailTemplate
}
[inline-code-end]

---