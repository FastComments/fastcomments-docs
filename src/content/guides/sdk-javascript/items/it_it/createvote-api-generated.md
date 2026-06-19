## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| commentId | string | Sì |  |
| direction | CreateVoteDirectionEnum | Sì |  |
| userId | string | No |  |
| anonUserId | string | No |  |

## Risposta

Restituisce: [`VoteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/VoteResponse.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di createVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_82a1c4b7';
const commentId: string = 'cmt_5f4d3a2b1c';
const direction: CreateVoteDirectionEnum = CreateVoteDirectionEnum.UP;
const anonUserId: string = 'anon_9f8e7d6c';
const voteResponse: VoteResponse = await createVote(tenantId, commentId, direction, undefined, anonUserId);
[inline-code-end]

---