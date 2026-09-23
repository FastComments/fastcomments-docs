[api-resource-header-start name = 'Page Reacts'; route = 'GET /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

Restituisce il conteggio per ogni reazione su una pagina e le reazioni che l'utente corrente ha aggiunto.

[inline-code-attrs-start title = 'Esempio cURL per Page Reacts'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della Richiesta Page Reacts'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsRequestQueryParams {
    urlId: string
    /** JSON codificato URI del tuo oggetto SSO. Ometti per utenti anonimi. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della Risposta Page Reacts'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsResponse {
    status: 'success' | 'failed'
    /** Incluso in caso di errore. **/
    code?: string
    /** Incluso in caso di errore. **/
    reason?: string
    /** Conteggio per ID reazione, ad esempio {"heart": 12, "laugh": 3}. Non impostato quando la pagina non ha reazioni. **/
    counts?: Record<string, number>
    /** ID delle reazioni che l'utente che effettua la richiesta ha aggiunto. Non impostato quando non ne ha aggiunte. **/
    reactedIds?: string[]
}
[inline-code-end]