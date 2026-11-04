[api-resource-header-start name = 'Page Reacts'; route = 'GET /api/v1/pages/reacts'; creditsCost = 1; api-resource-header-end]

Restituisce il conteggio di ogni reazione su una pagina e, facoltativamente, gli utenti che hanno reagito, con una voce per ogni reazione con il relativo id di reazione, dalla più recente. Questa è la versione server-side di [`GET /page-reacts/v2/:tenantId`](/guide-api.html#page-reacts-get).

`includeUserInfo` controlla quali utenti vengono restituiti:

- `public` esclude gli utenti che hanno reso privata l'attività del proprio profilo.
- `include-private` li include. Usare questa opzione solo quando l'elenco non viene mostrato al pubblico.

Gli utenti anonimi non vengono mai inclusi e vengono cercate fino a 100 reazioni. Le pagine che non esistono non restituiscono alcun `counts` e un elenco `users` vuoto.

[inline-code-attrs-start title = 'Esempio cURL di Page Reacts'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pages/reacts?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=example-id-or-url&includeUserInfo=public'
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della richiesta Page Reacts'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
    /** Ometti per ottenere solo i conteggi. **/
    includeUserInfo?: 'public' | 'include-private'
}
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della risposta Page Reacts'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsResponse {
    status: 'success' | 'failed'
    /** Incluso in caso di errore. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id'
    /** Incluso in caso di errore. **/
    reason?: string
    /** Conteggio per ogni id di reazione, ad esempio {"heart": 12, "laugh": 3}. Non impostato quando la pagina non ha reazioni. **/
    counts?: Record<string, number>
    /** Una voce per ogni reazione, dalla più recente. Impostato solo quando includeUserInfo viene passato. **/
    users?: PageCustomReactUser[]
}

interface PageCustomReactUser {
    /** L'id della reazione, ad esempio "heart". **/
    reactId: string
    userId: string
    username: string
    displayName?: string
    avatarSrc?: string
}
[inline-code-end]

#### Suggerimento utile

Ricordarsi di eseguire l'URI Encoding di valori come `urlId`.