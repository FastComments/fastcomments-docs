---
## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| updateModeratorBody | UpdateModeratorBody | Ja |  |

## Respons

Retourneert: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'updateModerator Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9f8b7c6d";
const moderatorId: string = "mod_12345";

const updateBody: UpdateModeratorBody = {
  isActive: true,
  // role?: string is optioneel en hier weggelaten
};

const result: APIEmptyResponse = await updateModerator(tenantId, moderatorId, updateBody);
[inline-code-end]

---