[api-resource-header-start name = 'AuditLog'; route = 'GET /api/v1/audit-logs'; creditsCost = 10; api-resource-header-end]

Овај API користи пагинацију, обезбијеђену параметрима `skip`, `before` и `after`. AuditLogs се враћају у страницама од `1000`, поређани по `when` и `id`.

Преузимање сваких `1000` логова кошта `10` кредита.

По подразумеваној поставци, добићете листу са **најновијим ставкама на почетку**. На овај начин можете започети са `skip=0` и пагинирати док не пронађете последњи запис који сте обрадили.

Алтернативно, можете сортирати тако да најстарији буду први и пагинирати док не буде више записа.

Сортирање се врши подешавањем `order` на `ASC` или `DESC`. Подразумевано је `ASC`.

Претраживање по датуму је могуће преко `before` и `after` као временских ознака са милисекундама. `before` и `after` нису укључени.

[inline-code-attrs-start title = 'Примјер cURL захтјева за AuditLog'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/audit-logs?tenantId=demo&API_KEY=DEMO_API_SECRET&skip=0&order=ASC&before=123&after=456'
[inline-code-end]

[inline-code-attrs-start title = 'Структура захтјева за AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    /** Укључено у случају грешке. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Укључено у случају грешке. **/
    reason?: string
    /** Логови! **/
    auditLogs: AuditLog[]
}
[inline-code-end]