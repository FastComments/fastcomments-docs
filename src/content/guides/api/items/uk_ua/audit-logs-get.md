[api-resource-header-start name = 'AuditLog'; route = 'GET /api/v1/audit-logs'; creditsCost = 10; api-resource-header-end]

Цей API використовує пагінацію, яку забезпечують параметри `skip`, `before` та `after`. AuditLogs повертаються сторінками по `1000`, впорядкованими за `when` та `id`.

Отримання кожних `1000` логів коштує `10` кредитів.

За замовчуванням ви отримаєте список з **найновішими елементами першими**. Таким чином ви можете опитувати, починаючи з `skip=0`, пагінувати далі, поки не знайдете останній запис, який вже оброблено.

Альтернативно, можна сортувати спочатку найстаріші та пагінувати, поки записи не закінчаться.

Сортування задається параметром `order` зі значенням `ASC` або `DESC`. За замовчуванням — `ASC`.

Фільтрувати за датою можна за допомогою `before` та `after` як міток часу в мілісекундах. `before` та `after` НЕ включні.

[inline-code-attrs-start title = 'Приклад cURL-запиту AuditLog'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/audit-logs?tenantId=demo&API_KEY=DEMO_API_SECRET&skip=0&order=ASC&before=123&after=456'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запиту AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsRequestQueryParams {
    tenantId: string
    API_KEY: string
    order?: 'ASC' | 'DESC'
    skip?: number
    before?: number
    after?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура відповіді AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsResponse {
    status: 'success' | 'failed'
    /** Включається у випадку невдачі. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Включається у випадку невдачі. **/
    reason?: string
    /** Журнали! **/
    auditLogs: AuditLog[]
}
[inline-code-end]