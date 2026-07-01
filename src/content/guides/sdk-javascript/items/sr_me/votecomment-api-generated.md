## Parametri

| Ime | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| urlId | string | Yes |  |
| broadcastId | string | Yes |  |
| voteBodyParams | VoteBodyParams | Yes |  |
| sessionId | string | No |  |
| sso | string | No |  |

## Odgovor

Vraća: [`VoteCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/VoteCommentResponse.ts)

## Primjer

[inline-code-attrs-start title = 'voteComment Primjer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp";
const commentId: string = "cmt_9f8e7d6c";
const urlId: string = "url_123456";
const broadcastId: string = "bcast_2024_01";

const voteBodyParams: VoteBodyParams = {
  vote: "up",               // npr., "up" | "down"
  weight: 1,                // opcionalno ponderisanje glasa
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