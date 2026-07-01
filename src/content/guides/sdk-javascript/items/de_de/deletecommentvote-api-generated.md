## Parameters

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenantId | string | Ja |  |
| commentId | string | Ja |  |
| voteId | string | Ja |  |
| urlId | string | Ja |  |
| broadcastId | string | Ja |  |
| editKey | string | Nein |  |
| sso | string | Nein |  |

## Response

Rückgabe: [`DeleteCommentVoteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteCommentVoteResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'deleteCommentVote Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function removeVote() {
  const tenantId: string = "tenant_12345";
  const commentId: string = "cmt_98765";
  const voteId: string = "vote_abcde";
  const urlId: string = "url_56789";
  const broadcastId: string = "brd_001";
  const editKey: string = "edit_456";
  // sso ist optional und wird weggelassen

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