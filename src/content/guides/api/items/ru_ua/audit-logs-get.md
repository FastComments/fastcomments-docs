[api-resource-header-start name = 'AuditLog'; route = 'GET /api/v1/audit-logs'; creditsCost = 10; api-resource-header-end]

Этот API использует пагинацию, обеспечиваемую параметрами `skip`, `before` и `after`. AuditLogs возвращаются страницами по `1000`, отсортированными по `when` и `id`.

Получение каждых `1000` логов стоит `10` кредитов.

По умолчанию вы получите список с **самыми новыми элементами первыми**. Так вы можете опрашивать, начиная с `skip=0`, переходя по страницам до тех пор, пока не найдёте последнюю запись, которую вы уже обработали.

В качестве альтернативы можно сортировать по старым записям сначала и постранично получать записи, пока они не закончатся.

Сортировку можно задать установкой `order` в `ASC` или `DESC`. По умолчанию `ASC`.

Запросы по дате возможны с помощью `before` и `after` в виде меток времени с миллисекундами. `before` и `after` НЕ включительны.

[inline-code-attrs-start title = 'Пример cURL запроса AuditLog'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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
    /** Включается при ошибке. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Включается при ошибке. **/
    reason?: string
    /** Журналы аудита. **/
    auditLogs: AuditLog[]
}
[inline-code-end]

---