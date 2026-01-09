[api-resource-header-start name = 'Page'; route = 'POST /api/v1/pages'; creditsCost = 1; api-resource-header-end]

Ова крајња тачка API-ја омогућава креирање страница.

Чест случај употребе је контрола приступа.

Напомене:

- Ако сте коментарисали у нити коментара, или позвали API да креирате `Comment`, већ сте креирали објекат `Page`! Можете покушати да га преузмете преко `Page` руте `/by-url-id`, просљеђујући исти `urlId` који је прослијеђен коментарском видгету.
- Структура `Page` садржи неке **израчунате** вриједности.
  Тренутно су то `commentCount` и `rootCommentCount`.
  Попуњавају се аутоматски и не могу се поставити преко API-ја. Покушај постављања ће узроковати да API врати грешку.

[inline-code-attrs-start title = 'Page POST cURL примјер'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/pages?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"title": "Test Page",
	"url": "some0-url",
	"urlId": "page2",
	"accessibleByGroupIds": ["SOME_GROUP_ID"]
}'
[inline-code-end]

[inline-code-attrs-start title = 'Page POST структура захтева'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagePostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Page POST структура одговора'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface PagePostResponse {
    status: 'success' | 'failed'
    /** Укључено у случају неуспјеха. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-request' | 'internal' | 'invalid-input' | 'invalid-title' | 'extra-params' | 'accessible-by-group-ids-not-array' | 'too-many-group-ids' | 'group-id-too-large';  
    /** Укључено у случају неуспјеха. **/
    reason?: string
    /** Креирана страница. **/
    page?: Page
}
[inline-code-end]

---