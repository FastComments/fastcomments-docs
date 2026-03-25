## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| id | string | Oui |  |
| redirectURL | string | Non |  |

## Réponse

Renvoie: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de sendLoginLink'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12a9f3b7";
const id: string = "user_84b2c7d1";
const redirectURL: string = "https://app.mycompany.com/welcome?ref=login_email";
const resultWithoutRedirect: FlagCommentPublic200Response = await sendLoginLink(tenantId, id);
const resultWithRedirect: FlagCommentPublic200Response = await sendLoginLink(tenantId, id, redirectURL);
[inline-code-end]