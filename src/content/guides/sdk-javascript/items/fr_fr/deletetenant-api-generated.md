## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| id | string | Oui |  |
| sure | string | Non |  |

## Réponse

Renvoie: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de deleteTenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_5f2b9c8a';
const id: string = 'site_9d4f2a1b';
const confirmation: string = 'CONFIRM_DELETE';
const result: APIEmptyResponse = await deleteTenant(tenantId, id, confirmation);
[inline-code-end]

---