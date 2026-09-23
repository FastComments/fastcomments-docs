## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| commentId | string | Ja |  |
| voteId | string | Ja |  |
| urlId | string | Ja |  |
| broadcastId | string | Ja |  |
| editKey | string | Nej |  |
| sso | string | Nej |  |

## Svar

Returnerer: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/VoteDeleteResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'deleteCommentVote Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const commentId: string = "cmt_9876";
const voteId: string = "vote_555";
const urlId: string = "url_abcde";
const broadcastId: string = "brd_2023";
const editKey: string = "edit_abc123"; // valgfri
const ssoToken: string = "sso_token_xyz"; // valgfri

// Kald kun med de påkrævede parametre og en valgfri
const deleteResult: VoteDeleteResponse = await deleteCommentVote(
  tenantId,
  commentId,
  voteId,
  urlId,
  broadcastId,
  editKey
);

// Kald med begge valgfrie parametre
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