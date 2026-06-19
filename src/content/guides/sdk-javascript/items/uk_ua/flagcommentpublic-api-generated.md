## Параметри

| Ім'я | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| commentId | string | Так |  |
| isFlagged | boolean | Так |  |
| sso | string | Ні |  |

## Відповідь

Повертає: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Приклад

[inline-code-attrs-start title = 'flagCommentPublic Приклад'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-inc-84";
const commentId: string = "b7f3e9a1-4c2d-4f6b-9f2a-123456789abc";
const isFlaggedOn: boolean = true;
const isFlaggedOff: boolean = false;
const ssoToken: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIn0.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c";
const resultWithoutSso: APIEmptyResponse = await flagCommentPublic(tenantId, commentId, isFlaggedOn);
const resultWithSso: APIEmptyResponse = await flagCommentPublic(tenantId, commentId, isFlaggedOff, ssoToken);
[inline-code-end]

---