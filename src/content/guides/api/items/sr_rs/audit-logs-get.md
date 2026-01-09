[api-resource-header-start name = 'AuditLog'; route = 'GET /api/v1/audit-logs'; creditsCost = 10; api-resource-header-end]

Овај API користи пагинацију, обезбеђену параметрима `skip`, `before` и `after`. AuditLogs се враћају у страницама по `1000`, сортиране по `when` и `id`.

Повлачење сваке `1000` ставке кошта `10` кредита.

Подразумевано ћете добити листу са **најновијим ставкама прво**. На тај начин можете почети са `skip=0`, пагинирајући док не пронађете последњи запис који сте обрадили.

Алтернативно, можете сортирати од најстаријих прво и пагинирати док не буде више записа.

Сортирање се врши постављањем параметра `order` на `ASC` или `DESC`. Подразумевана вредност је `ASC`.

Претраживање по датуму је могуће помоћу `before` и `after` као временских ознака у милисекундама. `before` и `after` нису укључиви.

[inline-code-attrs-start title = 'Пример cURL захтева за AuditLog'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/audit-logs?tenantId=demo&API_KEY=DEMO_API_SECRET&skip=0&order=ASC&before=123&after=456'
[inline-code-end]

[inline-code-attrs-start title = 'Структура захтева за AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Структура одговора за AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsResponse {
    status: 'success' | 'failed'
    /** Укључено у случају неуспеха. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Укључено у случају неуспеха. **/
    reason?: string
    /** Записи дневника! **/
    auditLogs: AuditLog[]
}
[inline-code-end]

---