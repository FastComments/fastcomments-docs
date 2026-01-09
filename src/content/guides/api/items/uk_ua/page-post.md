[api-resource-header-start name = 'Page'; route = 'POST /api/v1/pages'; creditsCost = 1; api-resource-header-end]

Ця кінцева точка API надає можливість створювати сторінки.

Одним із поширених випадків використання є контроль доступу.

Примітки:

- Якщо ви залишали коментар у гілці коментарів або викликали API для створення `Comment`, ви вже створили об'єкт `Page`! Ви можете спробувати отримати його через маршрут `Page` `/by-url-id`, передавши той самий `urlId`, який передається віджетові коментарів.
- Структура `Page` містить деякі **обчислені** значення.
  Наразі це `commentCount` та `rootCommentCount`.
  Вони заповнюються автоматично і не можуть бути встановлені через API. Спроба зробити це призведе до того, що API поверне помилку.

[inline-code-attrs-start title = 'Приклад cURL-запиту для Page POST'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Структура запиту Page POST'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagePostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура відповіді Page POST'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface PagePostResponse {
    status: 'success' | 'failed'
    /** Надається у разі невдачі. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-request' | 'internal' | 'invalid-input' | 'invalid-title' | 'extra-params' | 'accessible-by-group-ids-not-array' | 'too-many-group-ids' | 'group-id-too-large';  
    /** Надається у разі невдачі. **/
    reason?: string
    /** Створена сторінка. **/
    page?: Page
}
[inline-code-end]

---