---
[api-resource-header-start name = 'Vote'; route = 'GET /api/v1/votes/for-user'; creditsCost = 1; api-resource-header-end]

Permette di recuperare i voti lasciati da un utente su un determinato `urlId`. Richiede un `userId` che può essere un utente FastComments.com o un `SSO User`.

Questo è utile se vuoi mostrare se un utente ha votato un commento. Quando recuperi i commenti, chiama semplicemente questa API nello stesso momento per l'utente con lo stesso `urlId`.

Se usi il voto anonimo, dovresti invece passare `anonUserId`.

[inline-code-attrs-start title = 'Esempio cURL Voti per Utente'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/votes/for-user?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=test&userId=some-user-id'
[inline-code-end]

[inline-code-attrs-start title = 'Esempio cURL Voti per Utente Anonimo'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/votes/for-user?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=test&anonUserId=some-user-id'
[inline-code-end]

Nota che i voti anonimi appariranno nella lista `appliedAuthorizedVotes`. Sono considerati autorizzati poiché sono stati creati tramite l'API con una API key.

[inline-code-attrs-start title = 'Struttura della richiesta Voti per Utente'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesForUserRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
    userId?: string
    anonUserId?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della risposta Voti per Utente'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesForUserResponse {
    status: 'success' | 'failed'
    /** Incluso in caso di errore. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'invalid-user-id'
    /** Incluso in caso di errore. **/
    reason?: string
    /** Voti autorizzati e verificati, applicati ai commenti corrispondenti. **/
    appliedAuthorizedVotes: Vote[]
    /** Voti in attesa di verifica, non ancora applicati ai commenti corrispondenti. **/
    pendingVotes: Vote[]
}
[inline-code-end]

---