## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| commentId | string | Da |  |
| direction | string | Ne |  |
| broadcastId | string | Ne |  |
| tenantId | string | Ne |  |
| sso | string | Ne |  |

## Odgovor

Vraća: [`PostVoteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PostVoteResponse.ts)

## Primer

[inline-code-attrs-start title = 'postVote Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let commentId: string = 'cmt_12345';
let direction: string = 'up';
let broadcastId: string = 'brd_67890';
let tenantId: string = 'tenant_abc';
let sso: string = 'sso_token_xyz';

const fullVote: PostVoteResponse = await postVote(
  commentId,
  direction,
  broadcastId,
  tenantId,
  sso
);

const simpleVote: PostVoteResponse = await postVote('cmt_67890');
[inline-code-end]

---