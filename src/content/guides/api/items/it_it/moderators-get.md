[api-resource-header-start name = 'Moderator'; route = 'GET /api/v1/moderators'; creditsCost = 1; api-resource-header-end]

Questa API utilizza la paginazione, fornita dal parametro di query `skip`. I moderatori vengono restituiti in pagine da `100`, ordinati per `createdAt` e `id`.

Il costo si basa sul numero di moderatori restituiti, con un costo di `1 credit per 10` moderatori restituiti.

[inline-code-attrs-start title = 'Esempio cURL Moderatore'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/moderators?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della richiesta Moderatore'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Il numero di moderatori da saltare per la paginazione. **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della risposta Moderatore'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorsResponse {
    status: 'success' | 'failed'
    /** Incluso in caso di errore. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Incluso in caso di errore. **/
    reason?: string
    moderators?: Moderator[]
}
[inline-code-end]

---