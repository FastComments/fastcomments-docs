[api-resource-header-start name = 'Vote'; route = 'DELETE /api/v1/votes/:id'; creditsCost = 1; api-resource-header-end]

Questa route fornisce la possibilità di eliminare un singolo `Vote`.

[inline-code-attrs-start title = 'Esempio cURL per eliminare Vote'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/votes/some-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della richiesta di eliminazione Vote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VoteDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della risposta di eliminazione Vote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VoteDeleteResponse {
    status: 'success' | 'failed'
    /** Incluso in caso di fallimento. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id'
    /** Incluso in caso di fallimento. **/
    reason?: string
}
[inline-code-end]

Note:

- Questa API rispetta le impostazioni a livello di tenant. Ad esempio, se disabiliti il voto per una data pagina, e tenti di creare un voto tramite l'API, fallirà con il codice di errore `voting-disabled`.
- Questa API è attiva per impostazione predefinita.
- Questa API aggiornerà i `votes` del corrispondente `Comment`.