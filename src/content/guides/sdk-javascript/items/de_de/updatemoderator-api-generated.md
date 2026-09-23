---
## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|---------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| updateModeratorBody | UpdateModeratorBody | Ja |  |

## Antwort

Rückgabe: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'updateModerator Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9f8b7c6d";
const moderatorId: string = "mod_12345";

const updateBody: UpdateModeratorBody = {
  isActive: true,
  // role?: string ist optional und hier weggelassen
};

const result: APIEmptyResponse = await updateModerator(tenantId, moderatorId, updateBody);
[inline-code-end]

---