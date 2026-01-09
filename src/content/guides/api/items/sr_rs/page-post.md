[api-resource-header-start name = 'Page'; route = 'POST /api/v1/pages'; creditsCost = 1; api-resource-header-end]

Овај API ендпоинт омогућава креирање страница.

Чест случај употребе је контрола приступа.

Напомене:

- Ако сте коментарисали у нити коментара, или позвали API да бисте креирали `Comment`, ви сте већ креирали `Page` објекат! Можете покушати да га преузмете преко `/by-url-id` `Page` руте, прослеђујући исти `urlId` који сте проследили у видгету за коментаре.
- Структура `Page` садржи неке **израчунате** вредности.
  Тренутно то су `commentCount` и `rootCommentCount`.
  Оне се попуњавају аутоматски и не могу се поставити преко API-ја. Покушај да се то уради ће узроковати да API врати грешку.

[inline-code-attrs-start title = 'Пример cURL захтева за Page POST'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Структура Page POST захтева'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagePostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура одговора Page POST'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface PagePostResponse {
    status: 'success' | 'failed'
    /** Укључено у случају неуспеха. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-request' | 'internal' | 'invalid-input' | 'invalid-title' | 'extra-params' | 'accessible-by-group-ids-not-array' | 'too-many-group-ids' | 'group-id-too-large';  
    /** Укључено у случају неуспеха. **/
    reason?: string
    /** Креирана страница. **/
    page?: Page
}
[inline-code-end]

---