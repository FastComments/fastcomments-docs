## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| commentId | string | Так |  |
| voteId | string | Так |  |
| urlId | string | Так |  |
| broadcastId | string | Так |  |
| editKey | string | Ні |  |
| sso | string | Ні |  |

## Відповідь

Повертає: [`DeleteCommentVoteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteCommentVoteResponse.ts)

## Приклад

[inline-code-attrs-start title = 'deleteCommentVote Приклад'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function removeVote() {
  const tenantId: string = "tenant_12345";
  const commentId: string = "cmt_98765";
  const voteId: string = "vote_abcde";
  const urlId: string = "url_56789";
  const broadcastId: string = "brd_001";
  const editKey: string = "edit_456";
  // sso є необов'язковим і опущено

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