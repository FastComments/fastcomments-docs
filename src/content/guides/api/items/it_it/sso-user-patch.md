[api-resource-header-start name = 'SSOUser'; route = 'PATCH /api/v1/sso-users/:id'; creditsCost = 1; api-resource-header-end]

Questa route consente di aggiornare un singolo utente SSO.

[inline-code-attrs-start title = 'Esempio cURL di aggiornamento SSOUser'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/sso-users/my-user-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"username": "notfordperfect"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della richiesta di aggiornamento SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPatchQueryParams {
    tenantId: string
    API_KEY: string
    /** Quando l'email o il nome utente viene cambiato, puoi impostare questo su true per aggiornare anche i commenti dell'utente. Questo raddoppierà il costo in crediti. **/
    updateComments: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della risposta di aggiornamento SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPatchResponse {
    status: 'success' | 'failed'
    /** Incluso in caso di fallimento. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'empty-request' | 'invalid-input' | 'missing-id' | 'user-does-not-exist'
    /** Incluso in caso di fallimento. **/
    reason?: string
    user?: SSOUser; // Restituiamo l'utente aggiornato completo in caso di successo.
}
[inline-code-end]


---