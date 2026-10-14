[api-resource-header-start name = 'Page Reacts'; route = 'GET /api/v1/pages/reacts'; creditsCost = 1; api-resource-header-end]

Restituisce il conteggio per ogni reazione su una pagina e, facoltativamente, gli utenti che hanno reagito, come una voce per reazione con il suo ID, più recente per prima. Questa è la versione server‑side di [`GET /page-reacts/v2/:tenantId`](/guide-api.html#page-reacts-get).

`includeUserInfo` controlla quali utenti vengono restituiti:

- `public` esclude gli utenti che hanno impostato la loro attività del profilo come privata.
- `include-private` li include. Usalo solo quando l'elenco non viene mostrato al pubblico.

Gli utenti anonimi non sono mai inclusi e vengono cercate fino a 100 reazioni. Le pagine che non esistono non restituiscono `counts` e restituiscono una lista `users` vuota.

[inline-code-attrs-start title = 'Esempio cURL per Page Reacts'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pages/reacts?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=example-id-or-url&includeUserInfo=public'
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della Richiesta Page Reacts'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
    /** Ometti per ottenere solo i conteggi. **/
    includeUserInfo?: 'public' | 'include-private'
}
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della Risposta Page Reacts'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsResponse {
    status: 'success' | 'failed'
    /** Incluso in caso di errore. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id'
    /** Incluso in caso di errore. **/
    reason?: string
    /** Conteggio per ID di reazione, ad esempio {"heart": 12, "laugh": 3}. Non impostato quando la pagina non ha reazioni. **/
    counts?: Record<string, number>
    /** Una voce per reazione, più recente per prima. Impostato solo quando includeUserInfo è fornito. **/
    users?: PageCustomReactUser[]
}

interface PageCustomReactUser {
    /** L'ID della reazione, ad esempio "heart". **/
    reactId: string
    userId: string
    username: string
    displayName?: string
    avatarSrc?: string
}
[inline-code-end]

#### Helpful Tip

Ricorda di codificare in URI i valori come `urlId`.

---