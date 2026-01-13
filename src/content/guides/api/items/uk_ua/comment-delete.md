[api-resource-header-start name = 'Comment'; route = 'DELETE /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

Ця кінцева точка API надає можливість видалити коментар.

Примітки:

- Цей API може оновлювати віджет коментарів у режимі реального часу, якщо потрібно (це збільшує `creditsCost` з `1` до `2`).
- Цей API видалить всі дочірні коментарі.

[inline-code-attrs-start title = 'Приклад запиту cURL для видалення коментаря'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json'
[inline-code-end]


[inline-code-attrs-start title = 'Структура запиту для видалення коментаря'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentDeleteQueryParams {
    tenantId: string
    API_KEY: string
	/** Користувач, що здійснює оновлення. За потреби може використовуватися для перевірки, чи має він право видалити коментар.  **/
    contextUserId?: string
	/** Чи слід видалити коментар «в реальному часі» для користувачів, що переглядають екземпляри віджета коментарів з тим самим urlId. ПРИМІТКА: Подвоює вартість у кредитах з 1 до 2. **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура відповіді для видалення коментаря'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentDeleteResponse {
    status: 'success' | 'failed'
    /** Включається у разі помилки. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found'
    /** Включається у разі помилки. **/
    reason?: string
}
[inline-code-end]

---