---
[api-resource-header-start name = 'AuditLog'; route = 'GET /api/v1/audit-logs'; creditsCost = 10; api-resource-header-end]

Этот API использует пагинацию, предоставляемую параметрами `skip`, `before` и `after`. Журналы аудита возвращаются страницами по `1000`, отсортированные по `when` и `id`.

Получение каждых `1000` записей журнала стоит `10` кредитов.

По умолчанию вы получите список с **самыми новыми элементами первыми**. Таким образом, вы можете опрашивать, начиная с `skip=0`, постранично, пока не найдете последнюю запись, которую вы обработали.

В качестве альтернативы, вы можете сортировать от старых к новым и постранично получать данные, пока записи не закончатся.

Сортировка выполняется установкой `order` в `ASC` или `DESC`. Значение по умолчанию — `ASC`.

Фильтрация по дате возможна с помощью `before` и `after` в виде меток времени с миллисекундами. `before` и `after` НЕ включают указанные значения.

[inline-code-attrs-start title = 'Пример cURL-запроса AuditLog'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/audit-logs?tenantId=demo&API_KEY=DEMO_API_SECRET&skip=0&order=ASC&before=123&after=456'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запроса AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Структура ответа AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsResponse {
    status: 'success' | 'failed'
    /** Указывается при ошибке. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Указывается при ошибке. **/
    reason?: string
    /** Журналы аудита! **/
    auditLogs: AuditLog[]
}
[inline-code-end]

---