[api-resource-header-start name = 'AuditLog'; route = 'GET /api/v1/audit-logs'; creditsCost = 10; api-resource-header-end]

Този API използва пагинация, предоставена от параметрите `skip`, `limit`, `before` и `after`. AuditLogs се връщат в страници по `1000` по подразбиране, до максимален `limit` от `10000`, подредени по `when` и `id`. Страниците са големи, защото този крайна точка обикновено се използва за изтегляне на история, а не за интерактивно прелистване.

Всеки `100` върнати записи имат кредитна цена от `1`.

По подразбиране ще получите списък с **най-новите елементи първо**. По този начин можете да правите заявки, започвайки с `skip=0`, прелистване, докато намерите последния запис, който сте консумирали.

Алтернативно, можете да сортирате от най-старите към най-новите и да прелиствате, докато не останат повече записи.

Сортирането може да се извърши, като зададете `order` на `ASC` или `DESC`. По подразбиране е `DESC`.

Запитване по дата е възможно чрез `before` и `after` като времеви печати в милисекунди. `before` и `after` НЕ са включващи и всяко от тях може да се използва самостоятелно.

## Откриване какво се е случило с лице

Всяко събитие записва кой го е извършил (`username`, `userId`, `ip`) и, отделно, върху какво е извършено. `targetLabel` е четим за хора етикет за този обект, например `jsmith (jsmith@example.com)`, а `targetId` е неговият идентификатор. Използвайте `target` за нечувствително към регистъра съвпадение на подниз в етикета, когато знаете името или имейла на лицето, но не неговия идентификатор.

Изтриванията запазват етикета по време на събитието, така че премахнат потребител или модератор все още може да бъде идентифициран след като основният запис е изтрит.

## Управлявани наематели

Ако вашият наемател управлява други наематели, задайте `includeManagedTenants=true`, за да върнете събития от вашия наемател и всеки наемател, който той управлява, в един отговор. `tenantId` на всеки върнат запис ви казва от кой наемател е дошъл.

[inline-code-attrs-start title = 'Пример cURL за AuditLog'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/audit-logs?tenantId=demo&API_KEY=DEMO_API_SECRET&skip=0&order=ASC&before=123&after=456'
[inline-code-end]

[inline-code-attrs-start title = 'Структура на заявка за AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLogsRequestQueryParams {
    tenantId: string
    API_KEY: string
    order?: 'ASC' | 'DESC'
    /** Max 10000. Defaults to 1000. **/
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

[inline-code-attrs-start title = 'Структура на отговор за AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

---