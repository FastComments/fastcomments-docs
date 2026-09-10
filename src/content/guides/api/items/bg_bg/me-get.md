[api-resource-header-start name = 'Me'; route = 'GET /api/v1/me'; creditsCost = 1; api-resource-header-end]

Описва идентификационните данни, които правят заявката: наема, към който принадлежи, и за OAuth токени за достъп – потребителя, който е упълномощил приложението. Интеграциите го използват, за да тестват връзка и да я етикетират.

При използване на API ключ отговорът идентифицира само наема. При OAuth носещ токен той също съдържа упълномощения потребител и предоставените обхвати.

[inline-code-attrs-start title = 'Пример за Me cURL'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/me?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Структура на отговора Me'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface MeResponse {
    status: 'success' | 'failed'
    /** Включено при неуспех. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Включено при неуспех. **/
    reason?: string
    tenantId: string
    tenantName: string
    /** Как е автентицирана заявката. **/
    authType: 'api-key' | 'oauth'
    /** Обхватите, които идентификационните данни притежават. API ключовете притежават и двете. **/
    scopes: ('read' | 'write')[]
    /** Присъства само за OAuth токени. **/
    userId?: string
    username?: string
    email?: string
}
[inline-code-end]