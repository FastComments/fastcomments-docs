[api-resource-header-start name = 'NotificationCount'; route = 'DELETE /api/v1/notification-count/:user_id'; creditsCost = 1; api-resource-header-end]

Ова рута брише један `NotificationCount` по корисничком ид-у. Код SSO, кориснички ид је у формату `<tenant id>:<user id>`.

Ово ће очистити број непрочитаних обавештења корисника (црвено звонце у коментарском виџету ће избледети и број ће нестати).

[inline-code-attrs-start title = 'Пример cURL захтева за брисање NotificationCount'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/notification-count/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
# -> {"status":"success"}
[inline-code-end]

[inline-code-attrs-start title = 'Структура захтева за брисање NotificationCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCountDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура одговора за брисање NotificationCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCountDeleteResponse {
    status: 'success' | 'failed'
    /** Укључено при неуспеху. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'not-found'
    /** Укључено при неуспеху. **/
    reason?: string
}
[inline-code-end]