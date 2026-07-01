## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenantId | string | Ja |  |
| commentId | string | Ja |  |
| direction | CreateVoteDirectionEnum | Ja |  |
| userId | string | Nein |  |
| anonUserId | string | Nein |  |

## Antwort

Rückgabe: [`CreateVoteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateVoteResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'createVote Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const commentId: string = "comment_987654";
const direction: CreateVoteDirectionEnum = CreateVoteDirectionEnum.Upvote;
const userId: string = "user_7f9a2b";

const voteResult: CreateVoteResponse = await createVote(
  tenantId,
  commentId,
  direction,
  userId
);
[inline-code-end]