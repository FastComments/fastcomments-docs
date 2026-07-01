## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| commentId | string | Да |  |
| broadcastId | string | Не |  |
| tenantId | string | Не |  |
| sso | string | Не |  |

## Отговор

Връща: [`PostFlagCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PostFlagCommentResponse.ts)

## Пример

[inline-code-attrs-start title = 'postFlagComment Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const commentId: string = "cmt_20230915_001";
const broadcastId: string = "brd_20230915_live";
const tenantId: string = "tenant_42";
const sso: string = "sso_token_abc123";

const flaggedResponse: PostFlagCommentResponse = await postFlagComment(
  commentId,
  broadcastId,
  tenantId,
  sso
);
[inline-code-end]