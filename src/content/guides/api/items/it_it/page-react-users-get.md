[api-resource-header-start name = 'Page React Users'; route = 'GET /page-reacts/v2/:tenantId/list'; creditsCost = 0; api-resource-header-end]

Restituisce i nomi degli utenti che hanno aggiunto una reazione a una pagina, ordinati alfabeticamente. Vengono cercate fino a 100 reazioni e gli utenti anonimi non sono inclusi. Nelle pagine che richiedono SSO, gli utenti che non possono visualizzare la pagina ricevono un `403` con il codice `sso-required` o `page-forbidden`.

Per ottenere anche avatar e ID utente, usa [`GET /page-reacts/v2/:tenantId`](/guide-api.html#page-reacts-get) con `includeUserInfo=public`.

[inline-code-attrs-start title = 'Esempio cURL Page React Users'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v2/demo/list?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della Richiesta Page React Users'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactUsersRequestQueryParams {
    urlId: string
    /** L'ID della reazione. **/
    id: string
    /** JSON codificato URI del tuo oggetto SSO. Ometti per gli utenti anonimi. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della Risposta Page React Users'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactUsersResponse {
    status: 'success' | 'failed'
    /** Incluso in caso di errore. **/
    code?: string
    /** Incluso in caso di errore. **/
    reason?: string
    userNames: string[]
}
[inline-code-end]

---