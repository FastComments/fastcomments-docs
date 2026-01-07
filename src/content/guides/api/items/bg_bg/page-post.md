[api-resource-header-start name = 'Page'; route = 'POST /api/v1/pages'; creditsCost = 1; api-resource-header-end]

Тази API крайна точка предоставя възможност за създаване на страници.

Често срещан случай на употреба е контролът на достъпа.

Забележки:

- Ако сте коментирали в нишка за коментари или сте извикали API, за да създадете `Comment`, вече сте създали обект `Page`! Можете да опитате да го извлечете чрез
  маршрута `/by-url-id` `Page`, подавайки същия `urlId`, подаден на уиджета за коментари.
- Структурата `Page` съдържа някои **изчислени** стойности.
  В момента това са `commentCount` и `rootCommentCount`.
  Те се попълват автоматично и не могат да бъдат зададени от API. Опитът да направите това ще накара API да върне грешка.

[inline-code-attrs-start title = 'Пример за POST на Page с cURL'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Структура на заявката за POST на Page'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagePostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура на отговора за POST на Page'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface PagePostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-request' | 'internal' | 'invalid-input' | 'invalid-title' | 'extra-params' | 'accessible-by-group-ids-not-array' | 'too-many-group-ids' | 'group-id-too-large';
    /** Included on failure. **/
    reason?: string
    /** The created page. **/
    page?: Page
}
[inline-code-end]
