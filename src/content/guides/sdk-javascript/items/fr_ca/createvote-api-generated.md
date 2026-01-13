## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| commentId | string | Oui |  |
| direction | CreateVoteDirectionEnum | Oui |  |
| userId | string | Non |  |
| anonUserId | string | Non |  |

## Réponse

Renvoie: [`VoteComment200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/VoteComment200Response.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de createVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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