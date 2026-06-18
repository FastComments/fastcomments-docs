## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| commentId | string | Так |  |
| broadcastId | string | Так |  |
| sso | string | Ні |  |

## Відповідь

Повертає: [`LockComment200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/LockComment200Response.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад lockComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_42f6c1';
const commentId: string = 'cmt-9a8b7c';
const broadcastId: string = 'brd_2026_06_15';
const ssoToken: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiI1Njc4OSIsImlhdCI6MTY1MDAwMDB9.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c';

const lockedWithSso: LockComment200Response = await lockComment(tenantId, commentId, broadcastId, ssoToken);
const lockedWithoutSso: LockComment200Response = await lockComment(tenantId, commentId, broadcastId);
[inline-code-end]

---