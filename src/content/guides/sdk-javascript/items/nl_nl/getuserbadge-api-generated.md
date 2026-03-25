## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |

## Respons

Retourneert: [`GetUserBadge200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserBadge200Response.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getUserBadge Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_acme_01";
const id: string = "badge_8c7d2f";
const response: GetUserBadge200Response = await getUserBadge(tenantId, id);
[inline-code-end]

---