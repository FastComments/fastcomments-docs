---
[api-resource-header-start name = 'SSOUser'; route = 'PUT /api/v1/sso-users'; creditsCost = 1; api-resource-header-end]

Questa route consente di aggiornare un singolo utente SSO.

[inline-code-attrs-start title = 'Esempio cURL di aggiornamento SSOUser'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PUT \
  --url 'https://fastcomments.com/api/v1/sso-users/my-user-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"username": "fordperfect",
	"displayName": "Ford Perfect",
	"email": "fordperfect@galaxy.com",
    "groupIds": ["some-optional-group-id"]
}'
[inline-code-end]

In questo esempio specifichiamo `groupIds` per il controllo degli accessi, ma questo è opzionale.

[inline-code-attrs-start title = 'Struttura richiesta di aggiornamento SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPutQueryParams {
    tenantId: string
    API_KEY: string
    /** Quando l'email o il nome utente viene modificato, puoi impostare questo su true per aggiornare anche i commenti dell'utente. Questo raddoppierà il costo in crediti. **/
    updateComments: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Struttura risposta di aggiornamento SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPutResponse {
    status: 'success' | 'failed'
    /** Incluso in caso di errore. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'empty-request' | 'invalid-input' | 'missing-id' | 'user-exists'
    /** Incluso in caso di errore. **/
    reason?: string
    user?: SSOUser; // Restituiamo l'utente aggiornato in caso di successo.
}
[inline-code-end]


---