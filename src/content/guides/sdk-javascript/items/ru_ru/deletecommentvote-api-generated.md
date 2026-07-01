## Параметры

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| voteId | string | Yes |  |
| urlId | string | Yes |  |
| broadcastId | string | Yes |  |
| editKey | string | No |  |
| sso | string | No |  |

## Ответ

Возвращает: [`DeleteCommentVoteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteCommentVoteResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример deleteCommentVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function removeVote() {
  const tenantId: string = "tenant_12345";
  const commentId: string = "cmt_98765";
  const voteId: string = "vote_abcde";
  const urlId: string = "url_56789";
  const broadcastId: string = "brd_001";
  const editKey: string = "edit_456";
  // sso является необязательным и опущен

  const response: DeleteCommentVoteResponse = await deleteCommentVote(
    tenantId,
    commentId,
    voteId,
    urlId,
    broadcastId,
    editKey
  );

  console.log(response);
}

removeVote();
[inline-code-end]