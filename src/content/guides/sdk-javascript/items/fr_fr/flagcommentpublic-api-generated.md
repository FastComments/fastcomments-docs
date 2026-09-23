## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| commentId | string | Oui |  |
| isFlagged | boolean | Oui |  |
| sso | string | Non |  |

## Réponse

Retourne : [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple flagCommentPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_42";
const commentId: string = "comment_1001";
const isFlagged: boolean = true;
const sso: string = "sso_user_5678";

const resultWithSso: APIEmptyResponse = await flagCommentPublic(tenantId, commentId, isFlagged, sso);
const resultWithoutSso: APIEmptyResponse = await flagCommentPublic(tenantId, commentId, false);
[inline-code-end]