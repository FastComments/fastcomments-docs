[api-resource-header-start name = 'Tenant'; route = 'POST /api/v1/tenants'; creditsCost = 1; api-resource-header-end]

Questa route fornisce la possibilità di aggiungere un singolo `Tenant`.

La creazione di un `Tenant` ha le seguenti restrizioni:

- Un `name` è obbligatorio.
- `domainConfiguration` è obbligatorio.
- I seguenti valori non possono essere forniti durante la creazione di un `Tenant`:
  - `hasFlexPricing` 
  - `lastBillingIssueReminderDate` 
  - `flexLastBilledAmount`
- La `signUpDate` non può essere nel futuro.
- Il `name` non può essere più lungo di `200 characters`.
- L'`email` non può essere più lunga di `300 characters`.
- L'`email` deve essere unica tra tutti i tenant di FastComments.com.
- Non puoi creare tenant se il tenant genitore non ha definito un `TenantPackage` valido.
  - Se il tuo tenant è stato creato tramite FastComments.com, questo non dovrebbe essere un problema.
- Non puoi creare più tenant di quanti definiti da `maxWhiteLabeledTenants` nel tuo pacchetto.
- Devi specificare il parametro di query `tenantId` che è l'id del tuo `parent tenant` con il white labeling abilitato.

Possiamo creare un `Tenant` con solo pochi parametri:

[inline-code-attrs-start title = 'Esempio cURL per la creazione di un Tenant'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "name": "Some Name",
	"email": "someone@someone.com",
    "domainConfiguration": [ { "domain": "somedomain.com" } ]
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della richiesta di creazione del Tenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della risposta alla creazione del Tenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPostResponse {
    status: 'success' | 'failed'
    /** Incluso in caso di errore. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'not-found' | 'unexpected-param' | 'sign-up-date-in-future' | 'payment-frequency-invalid' | 'cannot-change-payment-frequency' | 'name-invalid' | 'email-invalid' | 'email-taken' | 'no-package' | 'invalid-package' | 'unauthorized' | 'tenant-limit-reached' | 'cannot-move-tenant' | 'cannot-change-package' | 'invalid-billing-info'
    /** Incluso in caso di errore. **/
    reason?: string
    tenant?: Tenant; // Restituiamo il tenant creato completo in caso di successo.
}
[inline-code-end]