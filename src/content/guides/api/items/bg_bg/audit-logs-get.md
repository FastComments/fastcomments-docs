[api-resource-header-start name = 'AuditLog'; route = 'GET /api/v1/audit-logs'; creditsCost = 10; api-resource-header-end]

Този API използва странициране, предоставено от параметрите `skip`, `before` и `after`. AuditLogs се връщат на страници от `1000`, подредени по `when` и `id`.

Извличането на всеки `1000` лога има кредитна цена от `10`.

По подразбиране ще получите списък с **най-новите елементи първи**. По този начин можете да анкетирате, започвайки от `skip=0`, страници докато намерите последния запис, който сте консумирали.

Алтернативно, можете да сортирате най-старите първи и да страницирате докато няма повече записи.

Сортирането може да се направи като зададете `order` на `ASC` или `DESC`. По подразбиране е `ASC`.

Заявка по дата е възможна чрез `before` и `after` като времеви печати с милисекунди. `before` и `after` НЕ са включителни.

[inline-code-attrs-start title = 'Пример за AuditLog с cURL'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/audit-logs?tenantId=demo&API_KEY=DEMO_API_SECRET&skip=0&order=ASC&before=123&after=456'
[inline-code-end]

[inline-code-attrs-start title = 'Структура на заявката за AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Структура на отговора за AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Included on failure. **/
    reason?: string
    /** The logs! **/
    auditLogs: AuditLog[]
}
[inline-code-end]
