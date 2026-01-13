[api-resource-header-start name = 'TenantDailyUsage'; route = 'GET /api/v1/tenant-daily-usage'; creditsCost = 1; api-resource-header-end]

Ова рута омогућава претрагу коришћења tenant-а по години, месецу и дану. Може се вратити до 365 објеката, а трошак је 1 API кредит на сваких 10 објеката.

Објекти одговора су сортирани по датуму када су креирани (најстарији први).

[inline-code-attrs-start title = 'Пример cURL претраге TenantDailyUsage'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenant-daily-usage?tenantId=demo&API_KEY=DEMO_API_SECRET&yearNumber=2022&monthNumber=2&dayNumber=10'
[inline-code-end]

[inline-code-attrs-start title = 'Структура захтева TenantDailyUsage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantDailyUsageQueryParams {
    tenantId: string
    API_KEY: string
    /** На основу UTC. **/
    yearNumber?: number
    /** Нула-базирано. На основу UTC. **/
    monthNumber?: number
    /** Почиње од 1. На основу UTC. **/
    dayNumber?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура одговора TenantDailyUsage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantDailyUsageResponse {
    status: 'success' | 'failed'
    /** Укључено у случају неуспеха. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found'
    /** Укључено у случају неуспеха. **/
    reason?: string
    tenantDailyUsages: TenantDailyUsage[]
}
[inline-code-end]