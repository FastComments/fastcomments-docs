[api-resource-header-start name = 'Notification'; route = 'GET /api/v1/notifications/count'; creditsCost = 2; api-resource-header-end]

Ова рута враћа објекат који садржи број обавештења у параметру `count`.

Он је спорији од `/notification-count/` и дупло скупљи по кредитима, али омогућава филтрирање по више димензија.

Можете филтрирати истим параметрима као и endpoint `/notifications`, као што је `userId`. Са SSO, кориснички id је у формату `<tenant id>:<user id>`.

[inline-code-attrs-start title = 'cURL пример: Број непрочитаних обавештења за корисника'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/notifications/count?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID&viewed=false'
[inline-code-end]

[inline-code-attrs-start title = 'cURL пример: Број непрочитаних обавештења за корисника за одређену страницу'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/notifications/count?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID&viewed=false&urlId=some-article-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Структура захтева за број обавештења'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCountGetQueryParams {
    tenantId: string
    API_KEY: string
    /** Филтрира по кориснику. **/
    userId?: string
    /** Филтрира по urlId. **/
    urlId?: string
    /** Филтрира по изворном коментару. **/
    fromCommentId?: string
    /** Филтрира по прочитано/непрочитано. **/
    viewed?: 'true' | 'false'
    /** Филтрира по типу. **/
    type?: NotificationType
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура одговора за број обавештења'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCountGetResponse {
    status: 'success' | 'failed'
    /** Укључено у случају неуспеха. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'unexpected-param' | 'not-found'
    /** Укључено у случају неуспеха. **/
    reason?: string
    count?: number
}
[inline-code-end]