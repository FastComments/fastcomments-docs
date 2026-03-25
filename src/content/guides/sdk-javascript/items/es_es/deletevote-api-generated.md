## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| id | string | Sí |  |
| editKey | string | No |  |

## Respuesta

Devuelve: [`DeleteCommentVote200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteCommentVote200Response.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de deleteVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_7c3f2b4a";
const voteId: string = "vote_4f8d9a11";
const editKey: string = "edit_2b9f8c";
const resultWithoutKey: DeleteCommentVote200Response = await deleteVote(tenantId, voteId);
const resultWithKey: DeleteCommentVote200Response = await deleteVote(tenantId, voteId, editKey);
[inline-code-end]

---