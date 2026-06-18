## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| id | string | Oui |  |
| deleteComments | string | Non |  |
| commentDeleteMode | string | Non |  |

## Réponse

Renvoie: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de deleteTenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_8f3a2b1c4d";
const id: string = "user_62a4f9e0b7";
const deleteComments: string = "true";
const commentDeleteMode: string = "permanent";
const result: FlagCommentPublic200Response = await deleteTenantUser(tenantId, id, deleteComments, commentDeleteMode);
[inline-code-end]

---