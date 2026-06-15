---
Минулі коментатори на сторінці, які НЕ зараз онлайн. Відсортовано за displayName.
Використовуйте це після вичерпання /users/online, щоб відобразити розділ "Учасники".
Курсорна пагінація по commenterName: сервер проходить по частковому {tenantId, urlId, commenterName}
індексу від afterName вперед через $gt, без витрат на $skip.

## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| afterName | string | No |  |
| afterUserId | string | No |  |

## Відповідь

Повертає: [`GetOfflineUsers200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetOfflineUsers200Response.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад getOfflineUsers'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_prod_001';
const urlId: string = 'article-2026-06-15-how-ai-impacts';
const afterName: string = 'michael.smith';
const afterUserId: string = 'user_72b9';

const response: GetOfflineUsers200Response = await getOfflineUsers(tenantId, urlId, afterName, afterUserId);
[inline-code-end]

---