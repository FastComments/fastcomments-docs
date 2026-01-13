[api-resource-header-start name = 'Page'; route = 'PATCH /api/v1/pages/:id'; creditsCost = 1; api-resource-header-end]

Ова рута омогућава ажурирање једне `Page`. Одговарајући коментари ће бити ажурирани.

[inline-code-attrs-start title = 'Пример cURL захтева за ажурирање странице'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/pages/my-page-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"url": "https://example.com/some-page"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Структура захтева за ажурирање странице'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagePatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура одговора за ажурирање странице'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagePatchResponse {
    status: 'success' | 'failed'
    /** Укључено у случају неуспеха. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'missing-id' | 'page-does-not-exist' | 'empty-request' | 'internal' | 'invalid-input' | 'invalid-title' | 'extra-params' | 'accessible-by-group-ids-not-array' | 'too-many-group-ids' | 'group-id-too-large'
    /** Укључено у случају неуспеха. **/
    reason?: string
    user?: Page; // При успеху враћамо потпуно ажурирану страницу.
}
[inline-code-end]

#### Напомена

Неке параметре у објекту Page се аутоматски ажурирају. То су атрибути бројача и атрибут `title`. Атрибути бројача се не могу ажурирати
путем API-ја, јер су то израчунате вредности. Поље странице `title` може бити подешено путем API-ја, али ће бити преписано ако се видгет за коментаре користи на
страници са истим `urlId` и другачијим насловом странице.