## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| commentId | string | Да |  |
| broadcastId | string | Да |  |
| commentTextUpdateRequest | CommentTextUpdateRequest | Да |  |
| editKey | string | Не |  |
| sso | string | Не |  |

## Отговор

Връща: [`PublicAPISetCommentTextResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PublicAPISetCommentTextResponse.ts)

## Пример

[inline-code-attrs-start title = 'setCommentText Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_12345";
  const commentId: string = "cmt_98765";
  const broadcastId: string = "brd_54321";
  const commentTextUpdateRequest: CommentTextUpdateRequest = {
    text: "Revised comment text with additional details."
  };
  const editKey: string = "edit_abc123";
  const sso: string = "sso_token_456def";

  const response: PublicAPISetCommentTextResponse = await setCommentText(
    tenantId,
    commentId,
    broadcastId,
    commentTextUpdateRequest,
    editKey,
    sso
  );
})();
[inline-code-end]