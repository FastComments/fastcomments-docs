---
## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| commentId | string | Sì |  |
| direction | CreateVoteDirectionEnum | Sì |  |
| userId | string | No |  |
| anonUserId | string | No |  |

## Risposta

Restituisce: [`VoteComment200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/VoteComment200Response.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di createVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'acme-tenant-812';
  const commentId: string = '5e8f8b7a-3d4b-4f1b-9a2e-1c9f2d6a7b8c';
  const direction: CreateVoteDirectionEnum = CreateVoteDirectionEnum.UP;
  const anonUserId: string = 'anon-84b9c2d';
  const voteResult: VoteComment200Response = await createVote(tenantId, commentId, direction, undefined, anonUserId);
  console.log(voteResult);
})();
[inline-code-end]

---