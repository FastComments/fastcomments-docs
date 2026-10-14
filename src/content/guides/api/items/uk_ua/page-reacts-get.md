[api-resource-header-start name = 'Page Reacts'; route = 'GET /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

Повертає кількість кожної реакції на сторінці та які реакції додав поточний користувач.

Передайте `includeUserInfo=public`, щоб також отримати користувачів, які відреагували, у вигляді одного запису на реакцію з її ідентифікатором, спочатку найновіші. Користувачі з приватною активністю профілю та анонімні користувачі виключаються, і шукаються до 100 реакцій. Тут приймається лише `public`; `include-private` повертає `422` і доступний через [`GET /api/v1/pages/reacts`](/guide-api.html#pages-reacts-get) з API‑ключем. На сторінках, які вимагають SSO, `includeUserInfo` повертає `403` для користувачів, які не можуть переглянути сторінку.

[inline-code-attrs-start title = 'Приклад cURL для Page Reacts'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запиту Page Reacts'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsRequestQueryParams {
    urlId: string
    /** URI‑закодований JSON вашого SSO‑об’єкта. Пропустіть для анонімних користувачів. **/
    sso?: string
    /** Встановіть 'public', щоб включити користувачів, які відреагували. **/
    includeUserInfo?: 'public'
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура відповіді Page Reacts'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsResponse {
    status: 'success' | 'failed'
    /** Включено у випадку помилки. **/
    code?: 'include-private-not-allowed' | 'sso-required' | 'page-forbidden' | string
    /** Включено у випадку помилки. **/
    reason?: string
    /** Кількість для кожного ідентифікатора реакції, наприклад {"heart": 12, "laugh": 3}. Не встановлюється, коли на сторінці немає реакцій. **/
    counts?: Record<string, number>
    /** Ідентифікатори реакцій, які додав користувач, що робить запит. Не встановлюється, якщо жодної не додано. **/
    reactedIds?: string[]
    /** Один запис на реакцію, спочатку найновіші. Встановлюється лише коли передано includeUserInfo. **/
    users?: PageCustomReactUser[]
}

interface PageCustomReactUser {
    /** Ідентифікатор реакції, наприклад "heart". **/
    reactId: string
    userId: string
    username: string
    displayName?: string
    avatarSrc?: string
}
[inline-code-end]