## Параметри

| Назва | Тип | Обов’язково | Опис |
|------|------|-------------|------|
| commentId | string | Так |  |
| voteId | string | Так |  |
| broadcastId | string | Ні |  |
| tenantId | string | Ні |  |
| sso | string | Ні |  |

## Відповідь

Повертає: [`DeleteModerationVoteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteModerationVoteResponse.ts)

## Приклад

[inline-code-attrs-start title = 'deleteModerationVote Приклад'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const commentId: string = "cmt_12345";
const voteId: string = "vote_9876";
const broadcastId: string = "brd_001";
const tenantId: string = "tenant_42";
const sso: string = "sso_token_abc";

const result: DeleteModerationVoteResponse = await deleteModerationVote(
  commentId,
  voteId,
  broadcastId,
  tenantId,
  sso
);
[inline-code-end]