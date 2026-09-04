[api-resource-header-start name = 'AuditLog'; route = 'GET /api/v1/audit-logs'; creditsCost = 10; api-resource-header-end]

Този API използва пагинация, предоставена от параметрите `skip`, `limit`, `before` и `after`. AuditLogs се връщат в страници по `5000` по подразбиране, до максимален `limit` от `10000`, подредени по `when` и `id`. Страниците са големи, защото този крайна точка обикновено се използва за изтегляне на история, а не за интерактивно прелистване.

Всеки `100` върнати логове имат разход от `1` кредит.

По подразбиране ще получите списък с **най-новите елементи първи**. По този начин можете да правите заявки, започвайки с `skip=0`, прелистване, докато намерите последния запис, който сте консумирали.

Алтернативно, можете да сортирате най-старите първи и да прелиствате, докато не останат повече записи.

Сортирането може да се извърши, като зададете `order` на `ASC` или `DESC`. По подразбиране е `DESC`.

Запитване по дата е възможно чрез `before` и `after` като времеви печати в милисекунди. `before` и `after` НЕ са включващи и всяко от тях може да се използва самостоятелно.

## Откриване какво се е случило с дадено лице

Всяко събитие записва кой го е извършил (`username`, `userId`, `ip`) и, отделно, върху какво е извършено. `targetLabel` е четим за хора етикет за този обект, например `jsmith (jsmith@example.com)`, а `targetId` е неговият идентификатор. Използвайте `target` за нечувствително към регистъра съвпадение на подниз в етикета, когато знаете името или имейла на лицето, но не неговия идентификатор.

Изтриванията запазват етикета към момента на събитието, така че премахнат потребител или модератор все още може да бъде идентифициран след като основният запис е изтрит.

## Управлявани наематели

Ако вашият наемател управлява други наематели, задайте `includeManagedTenants=true`, за да върнете събития от вашия наемател и всеки наемател, който той управлява, в един отговор. `tenantId` на всеки върнат лог ви казва от кой наемател идва.

[inline-code-attrs-start title = 'Пример за cURL на AuditLog'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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
    /** Максимум 10000. По подразбиране 5000. **/
    limit?: number
    skip?: number
    before?: number
    after?: number
    /** Само събития, извършени от това потребителско име. **/
    username?: string
    /** Само събития от този IP адрес. **/
    ip?: string
    /** Само събития от този тип. **/
    crudType?: 'c' | 'r' | 'u' | 'd' | 'login'
    /** Само събития за този ресурс, напр. User или Moderator. **/
    resourceName?: string
    /** Само събития, чиито засегнат обект има този идентификатор. **/
    targetId?: string
    /** Съвпадение на подниз без значение за регистъра в етикета на засегнатия обект. **/
    target?: string
    /** Също така върнете събития от наематели, които този наемател управлява. **/
    includeManagedTenants?: boolean
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура на отговора за AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLogsResponse {
    status: 'success' | 'failed'
    /** Включено при неуспех. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'invalid-limit' | 'invalid-skip'
    /** Включено при неуспех. **/
    reason?: string
    /** Логовете! **/
    auditLogs: AuditLog[]
}
[inline-code-end]