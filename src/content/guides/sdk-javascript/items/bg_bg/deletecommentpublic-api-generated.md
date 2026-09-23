## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| commentId | string | Да |  |
| broadcastId | string | Да |  |
| editKey | string | Не |  |
| sso | string | Не |  |

## Отговор

Връща: [`PublicAPIDeleteCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PublicAPIDeleteCommentResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример за deleteCommentPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_12345";
  const commentId: string = "cmt_9876";
  const broadcastId: string = "brd_5555";
  const editKey: string = "edit_abc123";
  const sso: string = "sso_token_xyz";

  const resultWithoutOptional: PublicAPIDeleteCommentResponse = await deleteCommentPublic(tenantId, commentId, broadcastId);
  const resultWithOptional: PublicAPIDeleteCommentResponse = await deleteCommentPublic(tenantId, commentId, broadcastId, editKey, sso);
})();
[inline-code-end]

---