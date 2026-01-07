[api-resource-header-start name = 'Notification'; route = 'GET /api/v1/notifications/count'; creditsCost = 2; api-resource-header-end]

Diese Route gibt ein Objekt zurück, das die Anzahl der Benachrichtigungen unter einem `count`-Parameter enthält.

Sie ist langsamer als `/notification-count/` und kostet doppelt so viele Credits, ermöglicht aber die Filterung nach mehr Dimensionen.

Sie können nach denselben Parametern wie beim `/notifications`-Endpunkt filtern, wie z.B. `userId`. Bei SSO hat die Benutzer-ID das Format `<tenant id>:<user id>`.

[inline-code-attrs-start title = 'Anzahl ungelesener Benachrichtigungen für Benutzer cURL Beispiel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/notifications/count?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID&viewed=false'
[inline-code-end]

[inline-code-attrs-start title = 'Anzahl ungelesener Benachrichtigungen für Benutzer für bestimmte Seite cURL Beispiel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/notifications/count?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID&viewed=false&urlId=some-article-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Benachrichtigungsanzahl Anfragestruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCountGetQueryParams {
    tenantId: string
    API_KEY: string
    /** Filter by user. **/
    userId?: string
    /** Filter by urlId. **/
    urlId?: string
    /** Filter by source comment. **/
    fromCommentId?: string
    /** Filter by read/unread. **/
    viewed?: 'true' | 'false'
    /** Filter by type. **/
    type?: NotificationType
}
[inline-code-end]

[inline-code-attrs-start title = 'Benachrichtigungsanzahl Antwortstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCountGetResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'unexpected-param' | 'not-found'
    /** Included on failure. **/
    reason?: string
    count?: number
}
[inline-code-end]
