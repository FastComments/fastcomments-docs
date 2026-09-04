[api-resource-header-start name = 'AuditLog'; route = 'GET /api/v1/audit-logs'; creditsCost = 10; api-resource-header-end]

Цей API використовує пагінацію, яку забезпечують параметри `skip`, `limit`, `before` та `after`. AuditLogs повертаються сторінками по `5000` за замовчуванням, до максимальної `limit` у `10000`, впорядковані за `when` та `id`. Сторінки великі, оскільки цей кінцевий пункт зазвичай використовується для вивантаження історії, а не для інтерактивного перегортання.

Кожні `100` записів журналу, що повертаються, коштують `1` кредит.

За замовчуванням ви отримаєте список з **найновішими елементами спочатку**. Таким чином, ви можете опитувати, починаючи з `skip=0`, пагінуючи, доки не знайдете останній запис, який ви спожили.

Альтернативно, ви можете сортувати за найстарішими спочатку і пагінувати, доки не залишиться записів.

Сортування можна виконати, встановивши `order` у `ASC` або `DESC`. За замовчуванням — `DESC`.

Запит за датою можливий за допомогою `before` та `after` у вигляді міток часу з мілісекундами. `before` і `after` НЕ включаються, і кожен з них можна використовувати окремо.

## Пошук того, що сталося з особою

Кожна подія реєструє, хто її виконав (`username`, `userId`, `ip`) і, окремо, над чим вона була виконана. `targetLabel` — це зрозуміла людині мітка для цього об’єкта, наприклад `jsmith (jsmith@example.com)`, а `targetId` — його ідентифікатор. Використовуйте `target` для нечутливого до регістру пошуку підрядка в мітці, коли ви знаєте ім’я або електронну пошту особи, але не її ідентифікатор.

Видалення зберігають мітку на момент події, тому видалений користувач або модератор можуть бути ідентифіковані навіть після того, як базовий запис зник.

## Керовані орендарі

Якщо ваш орендар керує іншими орендарями, встановіть `includeManagedTenants=true`, щоб отримати події вашого орендаря та всіх орендарів, якими він керує, в одному відповіді. `tenantId` кожного поверненого журналу вказує, з якого орендаря він походить.

[inline-code-attrs-start title = 'Приклад cURL для AuditLog'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/audit-logs?tenantId=demo&API_KEY=DEMO_API_SECRET&skip=0&order=ASC&before=123&after=456'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запиту AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Структура відповіді AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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