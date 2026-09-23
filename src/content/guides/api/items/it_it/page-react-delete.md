[api-resource-header-start name = 'Page Reacts'; route = 'DELETE /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

Rimuove una delle reazioni dell'utente corrente da una pagina. Se l'utente non ha aggiunto quella reazione, la richiesta ha successo con il codice `no-react` e non modifica il conteggio.

[inline-code-attrs-start title = 'Esempio cURL per eliminare una reazione alla pagina'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della richiesta di eliminazione della reazione alla pagina'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactDeleteRequestQueryParams {
    urlId: string
    /** L'ID della reazione. **/
    id: string
    /** JSON codificato URI del tuo oggetto SSO. Ometti per gli utenti anonimi. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della risposta di eliminazione della reazione alla pagina'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactDeleteResponse {
    status: 'success' | 'failed'
    /** 'no-react' quando l'utente non ha aggiunto questa reazione. Altrimenti incluso in caso di errore. **/
    code?: 'no-react' | string
    /** Incluso in caso di errore. **/
    reason?: string
}
[inline-code-end]

---