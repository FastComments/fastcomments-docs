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
(async () => {
  const tenantId: string = 'tenant_8421';
  const id: string = 'vote_3f9b7c2a';
  const editKey: string = 'edit_7Xk9LpQ';
  const responseWithoutEdit: DeleteCommentVote200Response = await deleteVote(tenantId, id);
  const responseWithEdit: DeleteCommentVote200Response = await deleteVote(tenantId, id, editKey);
  console.log(responseWithoutEdit, responseWithEdit);
})();
[inline-code-end]

---