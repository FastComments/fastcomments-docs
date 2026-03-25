## Parámetros

| Name | Type | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| commentId | string | Sí |  |
| direction | CreateVoteDirectionEnum | Sí |  |
| userId | string | No |  |
| anonUserId | string | No |  |

## Respuesta

Devuelve: [`VoteComment200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/VoteComment200Response.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de createVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9f8b6a';
const commentId: string = 'comment_3b7d2e';
const direction: CreateVoteDirectionEnum = CreateVoteDirectionEnum.Up;
const anonUserId: string = 'anon_4c2a1f';

const voteResult: VoteComment200Response = await createVote(tenantId, commentId, direction, undefined, anonUserId);
[inline-code-end]

---