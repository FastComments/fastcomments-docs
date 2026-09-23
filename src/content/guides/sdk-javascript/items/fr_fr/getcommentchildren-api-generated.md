## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| sso | string | No |  |

## Réponse

Renvoie : [`ModerationAPIChildCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationAPIChildCommentsResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple getCommentChildren'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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