## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|-------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| id | string | Ja |  |

## Antwort

Gibt zurück: [`GetV2PageReactUsers200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetV2PageReactUsers200Response.ts)

## Beispiel

[inline-code-attrs-start title = 'getV2PageReactUsers Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "7421";
const urlId: string = "sports/london-marathon";
const id: string = "reactUser-3fa85f64-5717-4562-b3fc-2c963f66afa6";
const includeDeleted: boolean | undefined = undefined; // optionales Flag (Demonstration)

const result: GetV2PageReactUsers200Response = await getV2PageReactUsers(tenantId, urlId, id);
[inline-code-end]

---