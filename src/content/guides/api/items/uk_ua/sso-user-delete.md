[api-resource-header-start name = 'SSOUser'; route = 'DELETE /api/v1/sso-users/:id'; creditsCost = 1; api-resource-header-end]

Цей маршрут дозволяє видалити одного SSO-користувача за його id.

Зверніть увагу, що повторне завантаження віджета коментарів з payload для цього користувача просто повторно створить користувача без помітних змін.

Видалення коментарів користувача можливе через параметр запиту `deleteComments`. Зауважте, що якщо це true:

1. Усі коментарі користувача будуть видалені в режимі live.
2. Всі __child__ (тепер сирітські) коментарі будуть видалені або анонімізовані залежно від конфігурації сторінки, пов'язаної з кожним коментарем. Наприклад, якщо режим видалення треду — "anonymize", то відповіді залишаться, а коментарі користувача будуть анонімізовані. Це застосовується лише коли `commentDeleteMode` рівний `Remove` (значення за замовчуванням).
3. Значення `creditsCost` стає `2`.

### Анонімізовані коментарі

Ви можете зберегти коментарі користувача, але просто анонімізувати їх, встановивши `commentDeleteMode=1`.

Якщо коментарі користувача анонімізовано, то наступні значення встановлюються в null:

    - commenterName
    - commenterEmail
    - avatarSrc
    - userId
    - anonUserId
    - mentions
    - badges

Поля `isDeleted` та `isDeletedUser` встановлюються в `true`.

Під час рендерингу віджет коментарів використовуватиме `DELETED_USER_PLACEHOLDER` (за замовчуванням: "[deleted]") для імені користувача та `DELETED_CONTENT_PLACEHOLDER` для коментаря. Це можна налаштувати через інтерфейс налаштування віджета.

### Приклади

[inline-code-attrs-start title = 'Приклад cURL для видалення SSOUser'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/sso-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запиту для видалення SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum SSOUserCommentDeleteMode {
    Remove = 0, // за замовчуванням
    Anonymize = 1
}

interface SSOUsersDeleteQueryParams {
    tenantId: string
    API_KEY: string
    /** Ви можете встановити це в true, щоб також видалити коментарі користувача. Це подвоїть вартість у кредитах. **/
    deleteComments?: 'true' | 'false'
    /** Ви можете встановити це, щоб визначити, як обробляти коментарі користувача. **/
    commentDeleteMode?: SSOUserCommentDeleteMode
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура відповіді при видаленні SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUsersResponse {
    status: 'success' | 'failed'
    /** Включено у разі помилки. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'user-does-not-exist'
    /** Включено у разі помилки. **/
    reason?: string
    user?: SSOUser; // Ми повертаємо видаленого користувача при успіху.
}
[inline-code-end]