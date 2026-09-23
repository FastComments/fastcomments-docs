## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| editKey | string | Nej |  |

## Svar

Returnerer: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/VoteDeleteResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'deleteVote Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function run() {
  const tenantId: string = "tenant_12345";
  const commentId: string = "comment_98765";
  const editKey: string = "edit_abcde";

  const responseWithKey: VoteDeleteResponse = await deleteVote(tenantId, commentId, editKey);
  const responseWithoutKey: VoteDeleteResponse = await deleteVote(tenantId, commentId);
}

run();
[inline-code-end]