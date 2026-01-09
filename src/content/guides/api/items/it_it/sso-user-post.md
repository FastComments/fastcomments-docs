[api-resource-header-start name = 'SSOUser'; route = 'POST /api/v1/sso-users'; creditsCost = 1; api-resource-header-end]

Questa route fornisce la creazione di un singolo utente SSO.

Tentare di creare due utenti con lo stesso ID genererà un errore.

[inline-code-attrs-start title = 'Esempio cURL di creazione SSOUser'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"id": "my-user-id",
	"username": "fordperfect",
	"displayName": "Ford Perfect",
	"email": "fordperfect@galaxy.com",
    "groupIds": ["some-optional-group-id"]
}'
[inline-code-end]

In questo esempio specifichiamo `groupIds` per il controllo degli accessi, ma è facoltativo.

[inline-code-attrs-start title = 'Struttura della richiesta di creazione SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della risposta di creazione SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPostResponse {
    status: 'success' | 'failed'
    /** Incluso in caso di errore. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'empty-request' | 'invalid-input' | 'missing-id' | 'user-exists'
    /** Incluso in caso di errore. **/
    reason?: string
    user?: SSOUser; // Restituiamo l'utente creato in caso di successo.
}
[inline-code-end]

#### Nota sull'integrazione

I dati inviati tramite l'API possono essere sovrascritti semplicemente passando un payload HMAC dell'utente SSO diverso. Ad esempio, se imposti un username tramite l'API, ma poi ne fornisci uno diverso tramite il flusso SSO al caricamento della pagina, aggiorneremo automaticamente il loro username.

Non aggiorneremo i parametri dell'utente in questo flusso a meno che non li specifichi esplicitamente o li imposti a null (non undefined).