[api-resource-header-start name = 'AuditLog'; route = 'GET /api/v1/audit-logs'; creditsCost = 10; api-resource-header-end]

Овај API користи пагинацију, обезбеђену параметрима `skip`, `limit`, `before` и `after`. AuditLogs се враћају у страницама од `1000` по подразумеваној вредности, до максималног `limit` од `10000`, поређани по `when` и `id`. Странице су велике јер се овај крајњи тачка обично користи за извоз историје уместо да се интерактивно листа.

Сваких `100` логова који се враћају имају цену од `1` кредита.

Подразумевано, добићете листу са **најновијим ставкама прво**. На тај начин можете да пратите почевши од `skip=0`, листајући док не пронађете последњи запис који сте конзумирали.

Алтернативно, можете сортирати од најстаријих према новим и листати док не остане више записа.

Сортирање се може извршити постављањем `order` на `ASC` или `DESC`. Подразумевана вредност је `DESC`.

Упити по датуму су могући преко `before` и `after` као временски печаци у милисекундама. `before` и `after` НЕ укључују датум, и сваки се може користити самостално.

## Проналажење шта се десило са особом

Сваки догађај бележи ко га је извео (`username`, `userId`, `ip`) и, одвојено, на чему је извршен. `targetLabel` је људски читљива ознака за тај објекат, на пример `jsmith (jsmith@example.com)`, а `targetId` је његов идентификатор. Користите `target` за претрагу подстринга без разликовања величине слова у ознаци када знате име или имејл особе, али не и њен id.

Брисања снимају ознаку у моменту догађаја, тако да уклоњени корисник или модератор могу и даље бити идентификовани након што оригинални запис буде уклоњен.

## Управљани тенанти

Уколико ваш тенант управља другим тенантима, поставите `includeManagedTenants=true` да бисте добили догађаје из вашег тенанта и свих тенаната које он управља у једном одговору. `tenantId` сваког враћеног лога вам говори из ког тенанта потиче.

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
    /** Максимум 10000. Подразумевано 1000. **/
    limit?: number
    skip?: number
    before?: number
    after?: number
    /** Само догађаји извршени од стране овог корисничког имена. **/
    username?: string
    /** Само догађаји са ове IP адресе. **/
    ip?: string
    /** Само догађаји ове врсте. **/
    crudType?: 'c' | 'r' | 'u' | 'd' | 'login'
    /** Само догађаји за овај ресурс, нпр. User или Moderator. **/
    resourceName?: string
    /** Само догађаји чији је утицани објекат има овај id. **/
    targetId?: string
    /** Претрага подстринга без разликовања величине слова у ознаци утицаног објекта. **/
    target?: string
    /** Такође враћа догађаје из тенаната које овај тенант управља. **/
    includeManagedTenants?: boolean
}
[inline-code-end]

[inline-code-attrs-start title = 'AuditLog структура одговора'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLogsResponse {
    status: 'success' | 'failed'
    /** Укључено у случају грешке. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'invalid-limit' | 'invalid-skip'
    /** Укључено у случају грешке. **/
    reason?: string
    /** Логови! **/
    auditLogs: AuditLog[]
}
[inline-code-end]