## Параметри

| Назва | Тип | Обов’язковий | Опис |
|------|------|--------------|------|
| tenantId | string | Так |  |
| commentId | string | Так |  |
| adjustCommentVotesParams | AdjustCommentVotesParams | Так |  |
| broadcastId | string | Ні |  |
| sso | string | Ні |  |

## Відповідь

Повертає: [`AdjustVotesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AdjustVotesResponse.ts)

## Приклад

[inline-code-attrs-start title = 'postAdjustCommentVotes Приклад'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp";
const commentId: string = "cmt_987654321";
const adjustParams: AdjustCommentVotesParams = { voteDelta: 1 };
const broadcastId: string = "brd_112233";
const sso: string = "sso-token-xyz";

const result: AdjustVotesResponse = await postAdjustCommentVotes(
  tenantId,
  commentId,
  adjustParams,
  broadcastId,
  sso
);
[inline-code-end]