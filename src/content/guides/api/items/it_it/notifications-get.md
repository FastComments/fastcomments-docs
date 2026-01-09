[api-resource-header-start name = 'Notification'; route = 'GET /api/v1/notifications'; creditsCost = 1; api-resource-header-end]

Questa route restituisce fino a 30 oggetti `Notification` ordinati per `createdAt`, dal più recente al più vecchio.

Puoi filtrare per `userId`. Con SSO, l'id utente è nel formato `<tenant id>:<user id>`.

[inline-code-attrs-start title = 'Esempio cURL: notifiche non lette per utente'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/notifications?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID&viewed=false'
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della richiesta delle notifiche'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationsGetQueryParams {
    tenantId: string
    API_KEY: string
    /** Paginazione saltando record. **/
    skip?: number
    /** Filtra per utente. **/
    userId?: string
    /** Filtra per urlId. **/
    urlId?: string
    /** Filtra per commento sorgente. **/
    fromCommentId?: string
    /** Filtra per letto/non letto. **/
    viewed?: 'true' | 'false'
    /** Filtra per tipo. **/
    type?: NotificationType
}
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della risposta delle notifiche'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationsGetResponse {
    status: 'success' | 'failed'
    /** Incluso in caso di errore. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'unexpected-param' | 'not-found'
    /** Incluso in caso di errore. **/
    reason?: string
    notifications?: Notification[]
}
[inline-code-end]

---