## Parametre

| Navn | Type | Obligatorisk | Beskrivelse |
|------|------|--------------|-------------|
| commentId | string | Ja |  |
| direction | string | Nej |  |
| broadcastId | string | Nej |  |
| tenantId | string | Nej |  |
| sso | string | Nej |  |

## Svar

Returnerer: [`PostVoteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PostVoteResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'postVote Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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