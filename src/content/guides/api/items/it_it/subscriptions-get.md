[api-resource-header-start name = 'Subscription'; route = 'GET /api/v1/subscriptions'; creditsCost = 1; api-resource-header-end]

Questa route restituisce fino a 30 oggetti `Subscription` ordinati per `createdAt`, dal più recente.

Puoi filtrare per `userId`. Con SSO, l'ID utente ha il formato `<tenant id>:<user id>`.

[inline-code-attrs-start title = 'Esempio cURL: Sottoscrizioni per utente'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/subscriptions?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID'
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della richiesta: Sottoscrizioni'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SubscriptionsGetQueryParams {
    tenantId: string
    API_KEY: string
    /** Paginazione saltando i record. **/
    skip?: number
    /** Filtra per utente. **/
    userId?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della risposta: Sottoscrizioni'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SubscriptionsGetResponse {
    status: 'success' | 'failed'
    /** Incluso in caso di errore. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'unexpected-param' | 'not-found'
    /** Incluso in caso di errore. **/
    reason?: string
    subscriptions?: Subscription[]
}
[inline-code-end]

---