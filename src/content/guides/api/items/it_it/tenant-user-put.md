[api-resource-header-start name = 'TenantUser'; route = 'PUT /api/v1/tenant-users/:id'; creditsCost = 1; api-resource-header-end]

Questo endpoint consente di sostituire un singolo `TenantUser`.

La sostituzione di un `TenantUser` ha le seguenti restrizioni:

- La `signUpDate` non può essere nel futuro.
- Il `locale` deve essere presente nella lista di [Supported Locales](https://github.com/FastComments/fastcomments-typescript/blob/main/src/constants.ts#L1).
- Lo `username` deve essere univoco su tutto FastComments.com. Se ciò rappresenta un problema, consigliamo di utilizzare SSO.
- L'`email` deve essere univoca su tutto FastComments.com. Se ciò rappresenta un problema, consigliamo di utilizzare SSO.
- Non è possibile aggiornare il `tenantId` di un utente.

Possiamo sostituire un `TenantUser` come segue

[inline-code-attrs-start title = 'Esempio cURL per la sostituzione di TenantUser'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PUT \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "username": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della richiesta per la sostituzione di TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPutQueryParams {
    tenantId: string
    API_KEY: string
    /** Quando l'email o lo username vengono modificati, è possibile impostare questo su true per aggiornare anche i commenti dell'utente. Ciò raddoppierà il costo in crediti. **/
    updateComments: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della risposta per la sostituzione di TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPutResponse {
    status: 'success' | 'failed'
    /** Incluso in caso di errore. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'sign-up-date-in-future' | 'unsupported-locale' | 'unauthorized' | 'username-taken' | 'email-taken' | 'unauthorized' | 'no-package' | 'invalid-package' | 'tenant-user-limit-reached' | 'user-does-not-exist'
    /** Incluso in caso di errore. **/
    reason?: string
}
[inline-code-end]

---