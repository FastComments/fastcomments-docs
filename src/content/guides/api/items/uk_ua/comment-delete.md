[api-resource-header-start name = 'Comment'; route = 'DELETE /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

Цей ендпоінт API надає можливість видалити коментар.

Notes:

- Цей API може оновлювати віджет коментарів у режимі "live", якщо потрібно (це збільшує `creditsCost` з `1` до `2`).
- Цей API видалить всі дочірні коментарі.
- Якщо цільовий коментар заблоковано (`isLocked: true`), запит відхиляється з `code: 'locked'`. Спочатку розблокуйте коментар, потім видаліть його.

[inline-code-attrs-start title = 'Приклад cURL для видалення коментаря'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json'
[inline-code-end]


[inline-code-attrs-start title = 'Структура запиту на видалення коментаря'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentDeleteQueryParams {
    tenantId: string
    API_KEY: string
	/** Користувач, який виконує оновлення. За потреби може використовуватися для перевірки, чи він може видалити коментар.  **/
    contextUserId?: string
	/** Чи повинен коментар бути видалений "live" для користувачів, які переглядають екземпляри віджета коментарів з тим самим urlId. УВАГА: Подвоює вартість у кредитах з 1 до 2. **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура відповіді на видалення коментаря'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentDeleteResponse {
    status: 'success' | 'failed'
    /** Надається у разі невдачі. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'locked'
    /** Надається у разі невдачі. **/
    reason?: string
}
[inline-code-end]