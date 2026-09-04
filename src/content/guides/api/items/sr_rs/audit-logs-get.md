[api-resource-header-start name = 'AuditLog'; route = 'GET /api/v1/audit-logs'; creditsCost = 10; api-resource-header-end]

Овај API користи пагинацију, обезбеђену параметрима `skip`, `limit`, `before` и `after`. AuditLogs се враћају у страницама од `5000` по подразумеваној вредности, до максималног `limit` од `10000`, поређани по `when` и `id`. Странице су велике јер се овај крајњи тачка обично користи за извоз историје уместо за интерактивно листање.

Сваких `100` враћених логова има трошак од `1` кредита.

Подразумевано, добићете листу са **најновијим ставкама прво**. На тај начин можете да пратите почевши од `skip=0`, пагинирајући док не пронађете последњи запис који сте конзумирали.

Алтернативно, можете сортирати од најстаријих према новим и пагинирати док не остане више записа.

Сортирање се може извршити постављањем `order` на `ASC` или `DESC`. Подразумевано је `DESC`.

Упити по датуму су могући преко `before` и `after` као временских ознака у милисекундама. `before` и `after` НЕ укључују датум, и сваки се може користити самостално.

## Проналажење шта се десило са особом

Сваки догађај бележи ко га је извео (`username`, `userId`, `ip`) и, одвојено, на чему је извршен. `targetLabel` је људски читљива ознака за тај објекат, на пример `jsmith (jsmith@example.com)`, а `targetId` је његов ид. Користите `target` за претрагу подстринга без разликовања величине слова у ознаци када знате име или имејл особе, али не и њен ид.

Брисања бележе ознаку у тренутку догађаја, тако да уклоњени корисник или модератор могу и даље бити идентификовани након што оригинални запис не постоји.

## Управљани закупци

Ако ваш закупац управља другим закупцима, поставите `includeManagedTenants=true` да бисте добили догађаје из вашег закупца и свих закупаца које он управља у једном одговору. `tenantId` сваког враћеног лога вам говори из ког закупца потиче.

[inline-code-attrs-start title = 'AuditLog cURL пример'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/audit-logs?tenantId=demo&API_KEY=DEMO_API_SECRET&skip=0&order=ASC&before=123&after=456'
[inline-code-end]

[inline-code-attrs-start title = 'AuditLog структура захтева'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'AuditLog структура одговора'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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