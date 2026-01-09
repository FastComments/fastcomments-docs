[api-resource-header-start name = 'TenantUser'; route = 'POST /api/v1/tenant-users'; creditsCost = 1; api-resource-header-end]

Questa route consente di aggiungere un singolo `TenantUser`.

La creazione di un `TenantUser` ha le seguenti restrizioni:

- Un `username` è obbligatorio.
- Una `email` è obbligatoria.
- La `signUpDate` non può essere nel futuro.
- La `locale` deve essere nella lista di [Locali supportate](https://github.com/FastComments/fastcomments-typescript/blob/main/src/constants.ts#L1).
- Il `username` deve essere unico su tutto FastComments.com. Se questo è un problema, suggeriamo di usare l'SSO.
- La `email` deve essere unica su tutto FastComments.com. Se questo è un problema, suggeriamo di usare l'SSO.
- Non è possibile creare più tenant user di quanti definiti sotto `maxTenantUsers` nel tuo pacchetto.

Possiamo creare un `TenantUser` come segue

[inline-code-attrs-start title = 'Esempio cURL di creazione TenantUser'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "username": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struttura richiesta creazione TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struttura risposta creazione TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostResponse {
    status: 'success' | 'failed'
    /** Incluso in caso di errore. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'username-required' | 'email-required' | 'sign-up-date-in-future' | 'unsupported-locale' | 'unauthorized' | 'username-taken' | 'email-taken' | 'tenant-user-limit-reached'
    /** Incluso in caso di errore. **/
    reason?: string
    tenantUser?: TenantUser; // Restituiamo il TenantUser completo creato in caso di successo.
}
[inline-code-end]

---