## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| commentId | string | Evet |  |
| voteId | string | Evet |  |
| urlId | string | Evet |  |
| broadcastId | string | Evet |  |
| editKey | string | Hayır |  |
| sso | string | Hayır |  |

## Yanıt

Döndürür: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/VoteDeleteResponse.ts)

## Örnek

[inline-code-attrs-start title = 'deleteCommentVote Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const commentId: string = "cmt_9876";
const voteId: string = "vote_555";
const urlId: string = "url_abcde";
const broadcastId: string = "brd_2023";
const editKey: string = "edit_abc123"; // isteğe bağlı
const ssoToken: string = "sso_token_xyz"; // isteğe bağlı

// Yalnızca gerekli parametrelerle ve bir isteğe bağlı ile çağır
const deleteResult: VoteDeleteResponse = await deleteCommentVote(
  tenantId,
  commentId,
  voteId,
  urlId,
  broadcastId,
  editKey
);

// Her iki isteğe bağlı parametreyle de çağır
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

---