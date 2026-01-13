[api-resource-header-start name = 'Subscription'; route = 'GET /api/v1/subscriptions'; creditsCost = 1; api-resource-header-end]

Ова рута враћа до 30 `Subscription` објеката сортираних по `createdAt`, најновији први.

Можете филтрирати по `userId`. Са SSO, идентификатор корисника има формат `<tenant id>:<user id>`.

[inline-code-attrs-start title = 'Пример cURL захтева за претплате корисника'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/subscriptions?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID'
[inline-code-end]

[inline-code-attrs-start title = 'Структура захтева за претплате'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SubscriptionsGetQueryParams {
    tenantId: string
    API_KEY: string
    /** Пагинација прескакањем записа. **/
    skip?: number
    /** Филтрирај по кориснику. **/
    userId?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура одговора за претплате'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SubscriptionsGetResponse {
    status: 'success' | 'failed'
    /** Укључено у случају грешке. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'unexpected-param' | 'not-found'
    /** Укључено у случају грешке. **/
    reason?: string
    subscriptions?: Subscription[]
}
[inline-code-end]