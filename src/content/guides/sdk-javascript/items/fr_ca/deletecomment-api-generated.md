## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| id | string | Oui |  |
| contextUserId | string | Non |  |
| isLive | boolean | Non |  |

## Réponse

Retourne: [`DeleteComment200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteComment200Response.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de deleteComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_acme_01";
const id: string = "comment_5f3a2b7c";
const contextUserId: string = "user_1229";
const isLive: boolean = true;
const response: DeleteComment200Response = await deleteComment(tenantId, id, contextUserId, isLive);
[inline-code-end]

---