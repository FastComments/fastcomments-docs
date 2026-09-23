## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| commentId | string | Да |  |
| publicBlockFromCommentParams | PublicBlockFromCommentParams | Да |  |
| sso | string | Нет |  |

## Ответ

Возвращает: [`BlockSuccess`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BlockSuccess.ts)

## Пример

[inline-code-attrs-start title = 'Пример blockFromCommentPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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