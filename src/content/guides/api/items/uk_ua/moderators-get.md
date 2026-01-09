[api-resource-header-start name = 'Moderator'; route = 'GET /api/v1/moderators'; creditsCost = 1; api-resource-header-end]

Цей API використовує пагінацію, яку забезпечує параметр запиту `skip`. Модератори повертаються сторінками по `100`, відсортовані за `createdAt` та `id`.

Вартість залежить від кількості повернених модераторів: `1 credit per 10` повернених модераторів.

[inline-code-attrs-start title = 'Приклад cURL для модератора'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/moderators?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запиту для модераторів'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Кількість модераторів, які потрібно пропустити для пагінації. **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура відповіді для модераторів'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorsResponse {
    status: 'success' | 'failed'
    /** Додається у разі невдачі. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Додається у разі невдачі. **/
    reason?: string
    moderators?: Moderator[]
}
[inline-code-end]