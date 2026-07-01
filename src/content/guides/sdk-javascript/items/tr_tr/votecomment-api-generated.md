## Parameters

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| commentId | string | Evet |  |
| urlId | string | Evet |  |
| broadcastId | string | Evet |  |
| voteBodyParams | VoteBodyParams | Evet |  |
| sessionId | string | Hayır |  |
| sso | string | Hayır |  |

## Response

Döndürür: [`VoteCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/VoteCommentResponse.ts)

## Örnek

[inline-code-attrs-start title = 'voteComment Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp";
const commentId: string = "cmt_9f8e7d6c";
const urlId: string = "url_123456";
const broadcastId: string = "bcast_2024_01";

const voteBodyParams: VoteBodyParams = {
  vote: "up",               // örnek, "up" | "down"
  weight: 1,                // oyun isteğe bağlı ağırlığı
};

const sessionId: string = "sess_abc123def";
const sso: string = "sso_token_xyz";

const result: VoteCommentResponse = await voteComment(
  tenantId,
  commentId,
  urlId,
  broadcastId,
  voteBodyParams,
  sessionId,
  sso
);
[inline-code-end]

---