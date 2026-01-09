[api-resource-header-start name = 'EmailTemplate'; route = 'POST /api/v1/email-templates/render'; creditsCost = 1; api-resource-header-end]

Questo endpoint API fornisce la possibilità di visualizzare in anteprima i modelli di email.

[inline-code-attrs-start title = 'Esempio minimo cURL di anteprima EmailTemplate POST'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/email-templates/render?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "emailTemplateId": "comment-user-mention",
    "displayName": "I'm a custom template.",
    "ejs": "This is an @mention notification! My name is <%= comment.commenterName %>."
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della richiesta POST EmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplatePostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della risposta POST EmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface EmailTemplatePostResponse {
    status: 'success' | 'failed'
    /** Incluso in caso di fallimento. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'unexpected-param' | 'does-not-render';
    /** Incluso in caso di fallimento. **/
    reason?: string
    /** L'HTML renderizzato in caso di successo. **/
    html?: string
}
[inline-code-end]

---