## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|--------------|-------------|
| commentId | string | Oui |  |
| tenantId | string | Non |  |
| sso | string | Non |  |

## Réponse

Retourne : [`GetCommentChildrenResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentChildrenResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple getCommentChildren'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const commentId: string = "cmt_12345";
const tenantId: string = "tenant_xyz";
const sso: string = "sso_987654";

const fullResponse: GetCommentChildrenResponse = await getCommentChildren(commentId, tenantId, sso);
const minimalResponse: GetCommentChildrenResponse = await getCommentChildren(commentId);
[inline-code-end]