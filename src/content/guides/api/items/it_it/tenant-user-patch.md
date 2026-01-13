[api-resource-header-start name = 'TenantUser'; route = 'PATCH /api/v1/tenant-users/:id'; creditsCost = 1; api-resource-header-end]

Questa route fornisce la possibilità di aggiornare un singolo `TenantUser`.

L'aggiornamento di un `TenantUser` ha le seguenti restrizioni:

- Il `signUpDate` non può essere nel futuro.
- Il `locale` deve essere nella lista dei [Locali supportati](https://github.com/FastComments/fastcomments-typescript/blob/main/src/constants.ts#L1).
- Il `username` deve essere univoco in tutto FastComments.com. Se questo è un problema, suggeriamo di usare SSO.
- La `email` deve essere univoca in tutto FastComments.com. Se questo è un problema, suggeriamo di usare SSO.
- Non è possibile aggiornare il `tenantId` di un utente.

Possiamo creare un `TenantUser` come segue

[inline-code-attrs-start title = 'Esempio cURL di aggiornamento TenantUser'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "username": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della richiesta di aggiornamento TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPatchQueryParams {
    tenantId: string
    API_KEY: string
    /** Quando email o username vengono modificati, puoi impostare questo su true per aggiornare anche i commenti dell'utente. Questo raddoppierà il costo in crediti. **/
    updateComments: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della risposta di aggiornamento TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPatchResponse {
    status: 'success' | 'failed'
    /** Incluso in caso di errore. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'sign-up-date-in-future' | 'unsupported-locale' | 'unauthorized' | 'username-taken' | 'email-taken' | 'unauthorized' | 'no-package' | 'invalid-package' | 'tenant-user-limit-reached' | 'user-does-not-exist'
    /** Incluso in caso di errore. **/
    reason?: string
}
[inline-code-end]

---