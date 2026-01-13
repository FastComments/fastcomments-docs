[api-resource-header-start name = 'EmailTemplate'; route = 'POST /api/v1/email-templates'; creditsCost = 1; api-resource-header-end]

Questo endpoint API consente di creare modelli email.

Note:

- Non puoi avere più template con lo stesso `emailTemplateId` nello stesso dominio.
- Tuttavia puoi avere un template wildcard (`domain` = `*`) e un template specifico per dominio per lo stesso `emailTemplateId`.
- Specificare `domain` è rilevante solo se hai domini diversi, o vuoi usare template specifici per i test (`domain` impostato su `localhost` ecc).
- Se specifichi `domain` deve corrispondere a una `DomainConfig`. In caso di errore viene fornito un elenco di domini validi.
- La sintassi del template è EJS e viene renderizzata con un timeout di 500ms. Il P99 per il rendering è <5ms, quindi se raggiungi i 500ms qualcosa non va.
- **Il tuo template deve essere renderizzato con i `testData` forniti** per poter essere salvato. Gli errori di rendering vengono aggregati e riportati nella dashboard (presto disponibile via API). 

I dati minimi richiesti per aggiungere un template sono i seguenti:

[inline-code-attrs-start title = 'Esempio minimo di richiesta cURL POST per EmailTemplate'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/email-templates?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "emailTemplateId": "comment-user-mention",
    "displayName": "I'm a custom template.",
    "ejs": "This is an @mention notification! My name is <%= comment.commenterName %>."
}'
[inline-code-end]

Potresti voler avere template per sito, nel qual caso definisci `domain`:

[inline-code-attrs-start title = 'Esempio di richiesta cURL POST per EmailTemplate'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/email-templates?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "emailTemplateId": "comment-user-mention",
    "displayName": "I'm a custom template.",
    "ejs": "This is some email content!",
    "domain": "somespecificsite.com",
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della richiesta POST per EmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplatePostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della risposta POST per EmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface EmailTemplatePostResponse {
    status: 'success' | 'failed'
    /** Incluso in caso di errore. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'unexpected-param' | 'invalid-email-template-id' | 'domain-invalid' | 'duplicate' | 'does-not-render';
    /** Incluso in caso di errore. **/
    reason?: string
    /** Il template creato. **/
    emailTemplate?: EmailTemplate
}
[inline-code-end]