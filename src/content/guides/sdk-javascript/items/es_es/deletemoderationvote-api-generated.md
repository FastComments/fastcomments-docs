## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| commentId | string | Sí |  |
| voteId | string | Sí |  |
| broadcastId | string | No |  |
| sso | string | No |  |

## Respuesta

Devuelve: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/VoteDeleteResponse.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de deleteModerationVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runDeleteModerationVote(): Promise<void> {
  const tenantId: string = "tenant_9f8b7c6a";
  const commentId: string = "comment_4d3e2f1a";
  const voteId: string = "vote_12345abcde";
  const broadcastId: string | undefined = "broadcast_2023_09_15";
  const sso: string | undefined = "sso_user_7890token";

  const result: VoteDeleteResponse = await deleteModerationVote(
    tenantId,
    commentId,
    voteId,
    broadcastId,
    sso
  );

  console.log(result);
}
[inline-code-end]

---