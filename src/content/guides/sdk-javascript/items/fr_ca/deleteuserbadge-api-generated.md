## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| id | string | Oui |  |

## Réponse

Renvoie : [`APIEmptySuccessResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptySuccessResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de deleteUserBadge'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp-42';
const badgeId: string = 'badge_9f8b2c1d';
const includeAudit: boolean | undefined = undefined; // indicateur facultatif (non requis par deleteUserBadge)
const result: APIEmptySuccessResponse = await deleteUserBadge(tenantId, badgeId);
[inline-code-end]

---