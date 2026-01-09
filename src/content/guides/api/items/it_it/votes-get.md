[api-resource-header-start name = 'Vote'; route = 'GET /api/v1/votes'; creditsCost = 10; api-resource-header-end]

I voti devono essere recuperati tramite `urlId`.

### Tipi di voti

Esistono tre tipi di voti:

- Voti autenticati, che vengono applicati al commento corrispondente. È possibile crearli tramite questa API.
- Voti autenticati, che sono **in attesa** di verifica e quindi non sono ancora applicati al commento. Questi vengono creati quando un utente usa il meccanismo FastComments.com *accedi per votare*.
- Voti anonimi, che vengono applicati al commento corrispondente. Questi vengono creati insieme ai commenti anonimi.

Questi vengono restituiti in liste separate nell'API per ridurre la confusione.

[inline-code-attrs-start title = 'Esempio cURL per Votes'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/votes?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=test'
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della richiesta Votes'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della risposta Votes'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesResponse {
    status: 'success' | 'failed'
    /** Incluso in caso di errore. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id'
    /** Incluso in caso di errore. **/
    reason?: string
    /** Voti autorizzati e verificati, applicati ai rispettivi commenti. **/
    appliedAuthorizedVotes: Vote[]
    /** Voti anonimi, applicati ai rispettivi commenti. **/
    appliedAnonymousVotes: Vote[]
    /** Voti in attesa di verifica, non ancora applicati ai rispettivi commenti. **/
    pendingVotes: Vote[]
}
[inline-code-end]

#### Note sui voti anonimi

Nota che i voti anonimi creati tramite questa API appariranno nella lista `appliedAuthorizedVotes`. Sono considerati autorizzati poiché sono stati creati tramite l'API con una API key.

La struttura `appliedAnonymousVotes` è per voti creati senza email, API key, ecc.