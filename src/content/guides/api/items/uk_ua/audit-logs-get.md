[api-resource-header-start name = 'AuditLog'; route = 'GET /api/v1/audit-logs'; creditsCost = 10; api-resource-header-end]

Цей API використовує пагінацію, яку забезпечують параметри `skip`, `limit`, `before` та `after`. AuditLogs повертаються сторінками по `1000` записів за замовчуванням, до максимальної `limit` у `10000`, впорядковані за `when` та `id`. Сторінки великі, оскільки цей кінцевий пункт зазвичай використовується для вивантаження історії, а не для інтерактивного перегортання.

Кожні `100` повернутих записів коштують `1` кредит.

За замовчуванням ви отримуєте список з **найновішими елементами спочатку**. Таким чином, ви можете опитувати, починаючи з `skip=0`, пагінуючи, доки не знайдете останній запис, який ви спожили.

Альтернативно, ви можете сортувати за найстарішими спочатку і пагінувати, доки не залишиться записів.

Сортування можна виконати, встановивши `order` у `ASC` або `DESC`. За замовчуванням — `DESC`.

Запит за датою можливий за допомогою `before` та `after` у вигляді міток часу з мілісекундами. `before` і `after` НЕ включають зазначені значення, і кожен з них можна використовувати окремо.

## Пошук того, що сталося з особою

Кожна подія реєструє, хто її виконав (`username`, `userId`, `ip`) і, окремо, над чим вона була виконана. `targetLabel` — це зрозуміла людям мітка для цього об’єкта, наприклад `jsmith (jsmith@example.com)`, а `targetId` — його ідентифікатор. Використовуйте `target` для нечутливого до регістру пошуку підрядка в мітці, коли ви знаєте ім’я або електронну пошту особи, але не її ідентифікатор.

Видалення зберігає мітку на момент події, тому видалений користувач або модератор можуть бути ідентифіковані навіть після того, як базовий запис зник.

## Керовані орендарі

Якщо ваш орендар керує іншими орендарями, встановіть `includeManagedTenants=true`, щоб отримати події як від вашого орендаря, так і від усіх орендарів, якими він керує, в одному відповіді. `tenantId` кожного поверненого запису вказує, від якого орендаря він походить.

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
    /** Макс 10000. За замовчуванням 1000. **/
    limit?: number
    skip?: number
    before?: number
    after?: number
    /** Тільки події, виконані цим ім'ям користувача. **/
    username?: string
    /** Тільки події з цієї IP-адреси. **/
    ip?: string
    /** Тільки події цього типу. **/
    crudType?: 'c' | 'r' | 'u' | 'd' | 'login'
    /** Тільки події для цього ресурсу, напр. User або Moderator. **/
    resourceName?: string
    /** Тільки події, у яких уражений об’єкт має цей ідентифікатор. **/
    targetId?: string
    /** Пошук підрядка без урахування регістру в мітці ураженого об’єкта. **/
    target?: string
    /** Також повернути події від орендарів, якими керує цей орендар. **/
    includeManagedTenants?: boolean
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура відповіді AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLogsResponse {
    status: 'success' | 'failed'
    /** Включено у випадку помилки. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'invalid-limit' | 'invalid-skip'
    /** Включено у випадку помилки. **/
    reason?: string
    /** Логи! **/
    auditLogs: AuditLog[]
}
[inline-code-end]

---