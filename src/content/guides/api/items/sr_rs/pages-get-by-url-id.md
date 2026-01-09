[api-resource-header-start name = 'Page'; route = 'GET /api/v1/pages/by-url-id'; creditsCost = 1; api-resource-header-end]

Појединачне странице могу бити преузете по њиховом одговарајућем `urlId`-у. Ово може бити корисно за преглед наслова страница или броја коментара. 

[inline-code-attrs-start title = 'cURL пример за страницу по URL ИД'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pages/by-url-id?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Структура захтева за страницу по URL ID'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagesRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура одговора за страницу по URL ID'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagesResponse {
    status: 'success' | 'failed'
    /** Укључено у случају неуспеха. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id'
    /** Укључено у случају неуспеха. **/
    reason?: string
    page?: Page[] | null
}
[inline-code-end]

#### Корисан савет

Запамтите да вредности као што је `urlId` кодирајте у URI формату.