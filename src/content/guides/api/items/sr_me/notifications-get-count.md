[api-resource-header-start name = 'Notification'; route = 'GET /api/v1/notifications/count'; creditsCost = 2; api-resource-header-end]

Ова рута враћа објекат који садржи број обавештења у параметру `count`.

Спорији је од `/notification-count/` и кошта двоструко више кредита, али омогућава филтрирање по већем броју димензија.

Можете филтрирати истим параметрима као и ендпоинт `/notifications`, као што је `userId`. Са SSO, кориснички id је у формату `<tenant id>:<user id>`.

[inline-code-attrs-start title = 'Пример cURL захтева за број непрочитаних обавештења за корисника'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/notifications/count?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID&viewed=false'
[inline-code-end]

[inline-code-attrs-start title = 'Пример cURL захтева за број непрочитаних обавештења корисника за одређену страницу'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/notifications/count?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID&viewed=false&urlId=some-article-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Структура захтева за број обавештења'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCountGetQueryParams {
    tenantId: string
    API_KEY: string
    /** Филтрирај по кориснику. **/
    userId?: string
    /** Филтрирај по urlId. **/
    urlId?: string
    /** Филтрирај по изворном коментару. **/
    fromCommentId?: string
    /** Филтрирај по прочитано/непрочитано. **/
    viewed?: 'true' | 'false'
    /** Филтрирај по типу. **/
    type?: NotificationType
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура одговора за број обавештења'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCountGetResponse {
    status: 'success' | 'failed'
    /** Укључено у случају грешке. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'unexpected-param' | 'not-found'
    /** Укључено у случају грешке. **/
    reason?: string
    count?: number
}
[inline-code-end]