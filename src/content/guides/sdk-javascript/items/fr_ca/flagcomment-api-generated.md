## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| id | string | Oui |  |
| userId | string | Non |  |
| anonUserId | string | Non |  |

## Réponse

Retourne: [`FlagComment200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagComment200Response.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de flagComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7f3b21';
const commentId: string = 'cmt_9a2b4';
const userId: string = 'user_1024';
const result: FlagComment200Response = await flagComment(tenantId, commentId, userId);
[inline-code-end]

---