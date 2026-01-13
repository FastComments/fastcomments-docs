[api-resource-header-start name = 'TenantUser'; route = 'DELETE /api/v1/tenant-users/:id'; creditsCost = 5; api-resource-header-end]

Цей маршрут виконує видалення `TenantUser` за id.

Видалення коментарів користувача можливе через параметр запиту `deleteComments`. Зверніть увагу, що якщо це true:

1. Всі коментарі користувача будуть видалені в реальному часі.
2. Всі __child__ (тепер сирітські) коментарі будуть видалені або анонімізовані залежно від конфігурації сторінки, пов'язаної з кожним коментарем. Наприклад, якщо режим видалення нитки — "анонімізувати", тоді відповіді залишаться, а коментарі користувача будуть анонімізовані. Це застосовується лише коли `commentDeleteMode` дорівнює `Remove` (значення за замовчуванням).
3. `creditsCost` стає `2`.

### Анонімізовані коментарі

Ви можете зберегти коментарі користувача, але анонімізувати їх, встановивши `commentDeleteMode=1`.

Якщо коментарі користувача анонімізовані, то наступні значення встановлюються в null:

    - commenterName
    - commenterEmail
    - avatarSrc
    - userId
    - anonUserId
    - mentions
    - badges

`isDeleted` і `isDeletedUser` встановлюються в `true`.

Під час відображення віджет коментарів використовуватиме `DELETED_USER_PLACEHOLDER` (за замовчуванням: "[deleted]") для імені користувача та `DELETED_CONTENT_PLACEHOLDER` для коментаря. Ці значення можна налаштувати через інтерфейс налаштування віджету.

### Приклади

[inline-code-attrs-start title = 'Приклад cURL для видалення TenantUser'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запиту для видалення TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum TenantUserCommentDeleteMode {
    Remove = 0, // значення за замовчуванням
    Anonymize = 1
}

interface TenantUserDeleteQueryParams {
    tenantId: string
    API_KEY: string
    /** Ви можете встановити це в true, щоб також видалити коментарі користувача. Це подвоїть вартість у кредитах. **/
    deleteComments?: 'true' | 'false'
    /** Ви можете встановити це за бажанням, щоб визначити, як обробляти коментарі користувача. **/
    commentDeleteMode?: TenantUserCommentDeleteMode
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура відповіді при видаленні TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserDeleteResponse {
    status: 'success' | 'failed'
    /** Включається у випадку помилки. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'unauthorized'
    /** Включається у випадку помилки. **/
    reason?: string
}
[inline-code-end]

---