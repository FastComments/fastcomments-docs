[api-resource-header-start name = 'Moderator'; route = 'GET /api/v1/moderators'; creditsCost = 1; api-resource-header-end]

Этот API использует пагинацию, задаваемую параметром запроса `skip`. Модераторы возвращаются страницами по `100`, упорядоченными по `createdAt` и `id`.

Стоимость основана на количестве возвращенных модераторов: `1 credit per 10` возвращенных модераторов.

[inline-code-attrs-start title = 'Пример cURL-запроса для Moderator'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/moderators?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запроса для Moderator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Количество модераторов, которые нужно пропустить для пагинации. **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа для Moderator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorsResponse {
    status: 'success' | 'failed'
    /** Включается при ошибке. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Включается при ошибке. **/
    reason?: string
    moderators?: Moderator[]
}
[inline-code-end]