[api-resource-header-start name = 'Page Reacts'; route = 'GET /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

Restituisce il conteggio per ogni reazione su una pagina e le reazioni che l'utente corrente ha aggiunto.

Passa `includeUserInfo=public` per ottenere anche gli utenti che hanno reagito, come una voce per reazione con il suo ID, dalla più recente alla più vecchia. Gli utenti con attività di profilo privata e gli utenti anonimi sono esclusi, e vengono cercate fino a 100 reazioni. Solo `public` è accettato qui; `include-private` restituisce un `422` ed è disponibile da [`GET /api/v1/pages/reacts`](/guide-api.html#pages-reacts-get) con una chiave API. Sulle pagine che richiedono SSO, `includeUserInfo` restituisce un `403` per gli utenti che non possono visualizzare la pagina.

[inline-code-attrs-start title = 'Esempio cURL di Page Reacts'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della Richiesta Page Reacts'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsRequestQueryParams {
    urlId: string
    /** JSON codificato URI del tuo oggetto SSO. Ometti per gli utenti anonimi. **/
    sso?: string
    /** Imposta a 'public' per includere gli utenti che hanno reagito. **/
    includeUserInfo?: 'public'
}
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della Risposta Page Reacts'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsResponse {
    status: 'success' | 'failed'
    /** Incluso in caso di errore. **/
    code?: 'include-private-not-allowed' | 'sso-required' | 'page-forbidden' | string
    /** Incluso in caso di errore. **/
    reason?: string
    /** Conteggio per ID di reazione, ad esempio {"heart": 12, "laugh": 3}. Non impostato quando la pagina non ha reazioni. **/
    counts?: Record<string, number>
    /** Gli ID di reazione che l'utente che effettua la richiesta ha aggiunto. Non impostato se non ne ha aggiunti. **/
    reactedIds?: string[]
    /** Una voce per reazione, dalla più recente alla più vecchia. Impostato solo quando viene passato includeUserInfo. **/
    users?: PageCustomReactUser[]
}

interface PageCustomReactUser {
    /** L'ID di reazione, ad esempio "heart". **/
    reactId: string
    userId: string
    username: string
    displayName?: string
    avatarSrc?: string
}
[inline-code-end]