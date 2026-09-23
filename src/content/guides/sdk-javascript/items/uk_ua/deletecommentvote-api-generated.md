## Параметри

| Назва | Тип | Обов’язковий | Опис |
|------|------|--------------|------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| voteId | string | Yes |  |
| urlId | string | Yes |  |
| broadcastId | string | Yes |  |
| editKey | string | No |  |
| sso | string | No |  |

## Відповідь

Повертає: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/VoteDeleteResponse.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад deleteCommentVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const commentId: string = "cmt_9876";
const voteId: string = "vote_555";
const urlId: string = "url_abcde";
const broadcastId: string = "brd_2023";
const editKey: string = "edit_abc123"; // необов’язковий
const ssoToken: string = "sso_token_xyz"; // необов’язковий

// Виклик лише з обов’язковими параметрами та одним необов’язковим
const deleteResult: VoteDeleteResponse = await deleteCommentVote(
  tenantId,
  commentId,
  voteId,
  urlId,
  broadcastId,
  editKey
);

// Виклик з обома необов’язковими параметрами
const deleteResultWithSSO: VoteDeleteResponse = await deleteCommentVote(
  tenantId,
  commentId,
  voteId,
  urlId,
  broadcastId,
  editKey,
  ssoToken
);
[inline-code-end]