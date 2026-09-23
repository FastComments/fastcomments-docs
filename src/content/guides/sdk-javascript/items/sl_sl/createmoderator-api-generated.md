## Parameters

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| createModeratorBody | CreateModeratorBody | Da |  |

## Response

Vrne: [`CreateModeratorResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateModeratorResponse.ts)

## Primer

[inline-code-attrs-start title = 'createModerator Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";

const moderatorPayload: CreateModeratorBody = {
  userId: "user_9876",
  // neobvezno polje; lahko se izpusti, če ni potrebno
  notes: "Temporary moderator for event"
};

const response: CreateModeratorResponse = await createModerator(tenantId, moderatorPayload);
[inline-code-end]