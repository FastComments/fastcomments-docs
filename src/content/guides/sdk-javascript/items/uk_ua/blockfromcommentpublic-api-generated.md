## Параметри

| Назва | Тип | Обов’язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| publicBlockFromCommentParams | PublicBlockFromCommentParams | Yes |  |
| sso | string | No |  |

## Відповідь

Повертає: [`BlockSuccess`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BlockSuccess.ts)

## Приклад

[inline-code-attrs-start title = 'blockFromCommentPublic Приклад'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoBlock() {
  const tenantId: string = "tenant_9f8b7c";
  const commentId: string = "comment_3e2d1a";
  const blockParams: PublicBlockFromCommentParams = {
    reason: "harassment",
    expiresInHours: 48
  };
  const ssoToken: string = "sso_5g6h7i";

  const resultWithSso: BlockSuccess = await blockFromCommentPublic(tenantId, commentId, blockParams, ssoToken);
  const resultWithoutSso: BlockSuccess = await blockFromCommentPublic(tenantId, commentId, blockParams);
}
[inline-code-end]

---