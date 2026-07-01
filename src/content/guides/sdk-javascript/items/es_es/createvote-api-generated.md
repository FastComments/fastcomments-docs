---
## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| commentId | string | Sí |  |
| direction | CreateVoteDirectionEnum | Sí |  |
| userId | string | No |  |
| anonUserId | string | No |  |

## Respuesta

Devuelve: [`CreateVoteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateVoteResponse.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo createVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

---