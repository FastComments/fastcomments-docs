## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| id | string | Sí |  |
| editKey | string | No |  |

## Respuesta

Devuelve: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/VoteDeleteResponse.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de deleteVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9f7c2b1a';
const id: string = 'vote_4b6e9a23';
const editKey: string = 'editkey_02a8f3';

const deleteResultWithoutKey: VoteDeleteResponse = await deleteVote(tenantId, id);
const deleteResultWithKey: VoteDeleteResponse = await deleteVote(tenantId, id, editKey);
[inline-code-end]

---