[api-resource-header-start name = 'Notification'; route = 'GET /api/v1/notifications/count'; creditsCost = 2; api-resource-header-end]

Questa route restituisce un oggetto contenente il numero di notifiche sotto il parametro `count`.

È più lenta di `/notification-count/` e costa il doppio in crediti, ma permette di filtrare su più dimensioni.

Puoi filtrare con gli stessi parametri dell'endpoint `/notifications` come `userId`. Con SSO, l'id utente è nel formato `<tenant id>:<user id>`.

[inline-code-attrs-start title = 'Esempio cURL: Conteggio notifiche non lette per utente'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/notifications/count?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID&viewed=false'
[inline-code-end]

[inline-code-attrs-start title = 'Esempio cURL: Conteggio notifiche non lette per utente per pagina specifica'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/notifications/count?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID&viewed=false&urlId=some-article-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della richiesta: conteggio notifiche'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCountGetQueryParams {
    tenantId: string
    API_KEY: string
    /** Filtra per utente. **/
    userId?: string
    /** Filtra per urlId. **/
    urlId?: string
    /** Filtra per commento di origine. **/
    fromCommentId?: string
    /** Filtra per letto/non letto. **/
    viewed?: 'true' | 'false'
    /** Filtra per tipo. **/
    type?: NotificationType
}
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della risposta: conteggio notifiche'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCountGetResponse {
    status: 'success' | 'failed'
    /** Incluso in caso di errore. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'unexpected-param' | 'not-found'
    /** Incluso in caso di errore. **/
    reason?: string
    count?: number
}
[inline-code-end]