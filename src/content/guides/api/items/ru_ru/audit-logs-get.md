[api-resource-header-start name = 'AuditLog'; route = 'GET /api/v1/audit-logs'; creditsCost = 10; api-resource-header-end]

Этот API использует пагинацию, предоставляемую параметрами `skip`, `limit`, `before` и `after`. AuditLogs возвращаются страницами по `5000` записей по умолчанию, до максимального `limit` в `10000`, упорядоченные по `when` и `id`. Страницы большие, потому что этот эндпоинт обычно используется для выгрузки истории, а не для интерактивного постраничного просмотра.

Каждые `100` возвращаемых записей журнала стоят `1` кредит.

По умолчанию вы получите список с **самыми новыми элементами первыми**. Таким образом, вы можете опрашивать, начиная с `skip=0`, постранично, пока не найдете последнюю запись, которую вы обработали.

Либо вы можете сортировать от старых к новым и постранично получать данные, пока не останется записей.

Сортировку можно выполнить, установив `order` в `ASC` или `DESC`. По умолчанию — `DESC`.

Запрос по дате возможен через `before` и `after` в виде меток времени с миллисекундами. `before` и `after` НЕ включают указанные значения, и каждый из них может использоваться отдельно.

## Поиск того, что случилось с человеком

Каждое событие фиксирует, кто его выполнил (`username`, `userId`, `ip`) и, отдельно, над чем оно было выполнено. `targetLabel` — человекочитаемая метка для этого объекта, например `jsmith (jsmith@example.com)`, а `targetId` — его идентификатор. Используйте `target` для нечувствительного к регистру поиска подстроки в метке, когда известны имя или email человека, но не его id.

Удаления фиксируют метку в момент события, поэтому удалённый пользователь или модератор могут быть идентифицированы даже после удаления исходной записи.

## Управляемые арендаторы

Если ваш арендатор управляет другими арендаторами, установите `includeManagedTenants=true`, чтобы получить события от вашего арендатора и всех арендаторов, которыми он управляет, в одном ответе. `tenantId` каждой возвращённой записи журнала указывает, от какого арендатора она пришла.

[inline-code-attrs-start title = 'Пример cURL для AuditLog'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/audit-logs?tenantId=demo&API_KEY=DEMO_API_SECRET&skip=0&order=ASC&before=123&after=456'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запроса AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLogsRequestQueryParams {
    tenantId: string
    API_KEY: string
    order?: 'ASC' | 'DESC'
    /** Max 10000. Defaults to 5000. **/
    limit?: number
    skip?: number
    before?: number
    after?: number
    /** Only events performed by this username. **/
    username?: string
    /** Only events from this IP address. **/
    ip?: string
    /** Only events of this type. **/
    crudType?: 'c' | 'r' | 'u' | 'd' | 'login'
    /** Only events for this resource, e.g. User or Moderator. **/
    resourceName?: string
    /** Only events whose affected object has this id. **/
    targetId?: string
    /** Case-insensitive substring match on the affected object's label. **/
    target?: string
    /** Also return events from tenants this tenant manages. **/
    includeManagedTenants?: boolean
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLogsResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'invalid-limit' | 'invalid-skip'
    /** Included on failure. **/
    reason?: string
    /** The logs! **/
    auditLogs: AuditLog[]
}
[inline-code-end]