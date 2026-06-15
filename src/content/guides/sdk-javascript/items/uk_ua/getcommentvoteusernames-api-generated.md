## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| commentId | string | Так |  |
| dir | number | Так |  |
| sso | string | Ні |  |

## Відповідь

Повертає: [`GetCommentVoteUserNames200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentVoteUserNames200Response.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад getCommentVoteUserNames'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_67890";
const commentId: string = "comment_abc123";
const dir: number = 1;
const ssoToken: string = "sso-eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9";

const responseWithoutSSO: GetCommentVoteUserNames200Response = await getCommentVoteUserNames(tenantId, commentId, dir);
const responseWithSSO: GetCommentVoteUserNames200Response = await getCommentVoteUserNames(tenantId, commentId, dir, ssoToken);
[inline-code-end]

---