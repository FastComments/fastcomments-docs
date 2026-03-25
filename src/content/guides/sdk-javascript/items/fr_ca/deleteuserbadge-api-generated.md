## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| id | string | Oui |  |

## Réponse

Retourne: [`UpdateUserBadge200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateUserBadge200Response.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de deleteUserBadge'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
type DeleteOptions = { notifyModerators?: boolean };

const tenantId: string = 'tenant_8a3f21';
const id: string = 'badge_71f2b';
const options: DeleteOptions = { notifyModerators: true };

const result: UpdateUserBadge200Response = await deleteUserBadge(tenantId, id);
[inline-code-end]

---