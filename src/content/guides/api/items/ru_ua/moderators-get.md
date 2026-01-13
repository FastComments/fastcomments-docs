[api-resource-header-start name = 'Moderator'; route = 'GET /api/v1/moderators'; creditsCost = 1; api-resource-header-end]

Этот API использует постраничную навигацию, реализуемую параметром запроса `skip`. Модераторы возвращаются страницами по `100`, упорядоченными по `createdAt` и `id`.

Стоимость основана на количестве возвращённых модераторов: `1 кредит за каждые 10` возвращённых модераторов.

[inline-code-attrs-start title = 'Пример cURL для Moderator'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/moderators?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запроса Moderator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Количество модераторов для пропуска при постраничной навигации. **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа Moderator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

---