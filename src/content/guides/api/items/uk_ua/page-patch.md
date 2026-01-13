[api-resource-header-start name = 'Page'; route = 'PATCH /api/v1/pages/:id'; creditsCost = 1; api-resource-header-end]

Цей маршрут дає змогу оновити один `Page`. Відповідні коментарі будуть оновлені.

[inline-code-attrs-start title = 'Приклад cURL‑запиту для оновлення сторінки'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/pages/my-page-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"url": "https://example.com/some-page"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запиту оновлення сторінки'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagePatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура відповіді оновлення сторінки'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagePatchResponse {
    status: 'success' | 'failed'
    /** Включається у разі помилки. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'missing-id' | 'page-does-not-exist' | 'empty-request' | 'internal' | 'invalid-input' | 'invalid-title' | 'extra-params' | 'accessible-by-group-ids-not-array' | 'too-many-group-ids' | 'group-id-too-large'
    /** Включається у разі помилки. **/
    reason?: string
    user?: Page; // Ми повертаємо повністю оновлену сторінку у разі успіху.
}
[inline-code-end]

#### Примітка

Деякі параметри в об'єкті `Page` оновлюються автоматично. Ці параметри — підрахунки та атрибути заголовка. Підрахунки не можна оновити
через API, оскільки вони є обчислюваними значеннями. Заголовок сторінки `title` можна встановити через API, але він буде перезаписаний, якщо віджет коментарів використовується на
сторінці з тим самим `urlId` та іншим заголовком сторінки.