[api-resource-header-start name = 'AuditLog'; route = 'GET /api/v1/audit-logs'; creditsCost = 10; api-resource-header-end]

Этот API использует пагинацию, предоставляемую параметрами `skip`, `limit`, `before` и `after`. AuditLogs возвращаются страницами по `100` записей по умолчанию, до максимального `limit` в `200`, упорядоченные по `when` и `id`.

Каждые `100` возвращённых записей стоят `1` кредит.

По умолчанию вы получаете список **с новейшими элементами первыми**. Таким образом, вы можете опрашивать, начиная с `skip=0`, постранично, пока не найдёте последнюю запись, которую вы обработали.

Либо вы можете сортировать от старых к новым и постранично получать записи, пока они не закончатся.

Сортировку можно задать, установив `order` в `ASC` или `DESC`. По умолчанию — `DESC`.

Запрос по дате возможен через `before` и `after` в виде меток времени с миллисекундами. `before` и `after` НЕ включают указанные значения, и каждый из них может использоваться отдельно.

## Поиск того, что случилось с человеком

Каждое событие фиксирует, кто его выполнил (`username`, `userId`, `ip`) и, отдельно, над чем оно было выполнено. `targetLabel` — человекочитаемая метка для этого объекта, например `jsmith (jsmith@example.com)`, а `targetId` — его идентификатор. Используйте `target` для поиска подстроки без учёта регистра в метке, когда известно имя или email человека, но неизвестен его id.

Удаления сохраняют метку на момент события, поэтому удалённый пользователь или модератор всё ещё могут быть идентифицированы после того, как исходная запись исчезнет.

## Управляемые арендаторы

Если ваш арендатор управляет другими арендаторами, установите `includeManagedTenants=true`, чтобы вернуть события как вашего арендатора, так и всех арендаторов, которыми он управляет, в одном ответе. `tenantId` каждой возвращённой записи указывает, от какого арендатора она пришла.

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
    limit?: number
    skip?: number
    before?: number
    after?: number
    /** Только события, выполненные этим именем пользователя. **/
    username?: string
    /** Только события с этого IP-адреса. **/
    ip?: string
    /** Только события этого типа. **/
    crudType?: 'c' | 'r' | 'u' | 'd' | 'login'
    /** Только события для этого ресурса, например Пользователь или Модератор. **/
    resourceName?: string
    /** Только события, у которых затронутый объект имеет этот идентификатор. **/
    targetId?: string
    /** Поиск подстроки без учёта регистра в метке затронутого объекта. **/
    target?: string
    /** Также вернуть события от арендаторов, которыми управляет этот арендатор. **/
    includeManagedTenants?: boolean
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLogsResponse {
    status: 'success' | 'failed'
    /** Включается при ошибке. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'invalid-limit' | 'invalid-skip'
    /** Включается при ошибке. **/
    reason?: string
    /** Логи! **/
    auditLogs: AuditLog[]
}
[inline-code-end]