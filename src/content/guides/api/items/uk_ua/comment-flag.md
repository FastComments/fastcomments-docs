[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/flag'; creditsCost = 1; api-resource-header-end]

Цей API-ендпоінт дозволяє позначити коментар для конкретного користувача.

Примітки:

- Цей виклик завжди має виконуватися в контексті користувача. Користувач може бути користувачем FastComments.com, SSO-користувачем або користувачем орендаря.
- Якщо встановлено поріг flag-to-hide, коментар буде автоматично приховано в реальному часі після того, як його позначать визначену кількість разів.
- Після того, як він автоматично буде знятий зі схвалених (прихований) — коментар може бути повторно схвалений лише адміністратором або модератором. Зняття позначки (un-flagging) не призведе до повторного схвалення коментаря.

[inline-code-attrs-start title = 'Приклад cURL для позначення коментаря'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/flag?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

Для анонімного позначення потрібно вказати `anonUserId`. Це може бути ідентифікатор, який представляє анонімну сесію, або випадковий UUID.
Це дозволяє підтримувати позначення та зняття позначки з коментарів навіть якщо користувач не увійшов у систему. Таким чином, коментар може бути позначений як flagged, коли коментарі витягуються з тим самим `anonUserId`.

[inline-code-attrs-start title = 'Приклад cURL для анонімного позначення коментаря'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/flag?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]


[inline-code-attrs-start title = 'Структура запиту для позначення коментаря'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentFlagQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура відповіді для позначення коментаря'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentFlagResponse {
    status: 'success' | 'failed'
    /** Включено у випадку помилки. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id'
    /** Включено у випадку помилки. **/
    reason?: string
    /** Чи був коментар знятий зі схвалених (прихований) через те, що його позначили занадто багато разів? **/
    wasUnapproved?: boolean;
}
[inline-code-end]