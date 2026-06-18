## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| urlId | string | Oui |  |
| id | string | Oui |  |

## Réponse

Renvoie : [`GetV2PageReactUsers200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetV2PageReactUsers200Response.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple getV2PageReactUsers'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "7421";
const urlId: string = "sports/london-marathon";
const id: string = "reactUser-3fa85f64-5717-4562-b3fc-2c963f66afa6";
const includeDeleted: boolean | undefined = undefined; // paramètre optionnel (démonstration)

const result: GetV2PageReactUsers200Response = await getV2PageReactUsers(tenantId, urlId, id);
[inline-code-end]

---