## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| sso | string | No |  |

## Resposta

Retorna: [`ModerationAPIChildCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationAPIChildCommentsResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo getCommentChildren'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchChildren() {
  const tenantId: string = "tenant_12345";
  const commentId: string = "cmt_98765";
  const ssoToken: string = "sso_user_abc123";

  const responseWithSSO: ModerationAPIChildCommentsResponse = await getCommentChildren(tenantId, commentId, ssoToken);
  const responseWithoutSSO: ModerationAPIChildCommentsResponse = await getCommentChildren(tenantId, commentId);
}

fetchChildren();
[inline-code-end]