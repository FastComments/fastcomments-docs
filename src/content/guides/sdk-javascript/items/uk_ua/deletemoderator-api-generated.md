## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| id | string | Так |  |
| sendEmail | string | Ні |  |

## Відповідь

Повертає: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад deleteModerator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_4f3b2c9a';
const id: string = 'mod_9c2d1f7b';
const sendEmail: string = 'true';
const response: FlagCommentPublic200Response = await deleteModerator(tenantId, id, sendEmail);
console.log(response);
[inline-code-end]

---