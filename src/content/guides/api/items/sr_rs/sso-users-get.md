[api-resource-header-start name = 'SSOUser'; route = 'GET /api/v1/sso-users'; creditsCost = 10; api-resource-header-end]

Ова рута враћа SSO кориснике страницама по `100`. Пагинација се обезбеђује помоћу параметра `skip`. Корисници су сортирани по свом `signUpDate` и `id`.

[inline-code-attrs-start title = 'cURL пример за SSOUsers'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Структура захтева за SSOUsers'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUsersRequestQueryParams {
    tenantId: string
    API_KEY: string
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура одговора за SSOUsers'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUsersResponse {
    status: 'success' | 'failed'
    /** Додаје се у случају грешке. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Додаје се у случају грешке. **/
    reason?: string
    users: SSOUser[]
}
[inline-code-end]