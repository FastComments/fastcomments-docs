[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/un-flag'; creditsCost = 1; api-resource-header-end]

Цей API-ендпоінт надає можливість зняти позначку з коментаря для конкретного користувача.

Примітки:

- Цей виклик завжди має виконуватися в контексті користувача. Користувач може бути FastComments.com User, SSO User, або Tenant User.
- Після того, як коментар було автоматично відхилено (приховано), його можна повторно схвалити лише адміністратором або модератором. Зняття позначки не призведе до повторного схвалення коментаря.

[inline-code-attrs-start title = 'Приклад cURL: зняття позначки з коментаря'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-flag?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

Для анонімного позначення необхідно вказати `anonUserId`. Це може бути ідентифікатор, що представляє анонімну сесію, або випадковий UUID.

[inline-code-attrs-start title = 'Приклад cURL: зняття позначки з анонімного коментаря'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-flag?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]


[inline-code-attrs-start title = 'Структура запиту для зняття позначки з коментаря'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentFlagQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура відповіді при знятті позначки з коментаря'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentUnFlagResponse {
    status: 'success' | 'failed'
    /** Додається у разі невдачі. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id'
    /** Додається у разі невдачі. **/
    reason?: string
}
[inline-code-end]