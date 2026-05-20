## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Response

Returns: [`GetUserBadge200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserBadge200Response.ts)

## Example

[inline-code-attrs-start title = 'getUserBadge Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "blue-ocean-llc";
const suffix: string | undefined = "gold";
const id: string = suffix ? `badge-${suffix}-1a2b3c4d` : "badge-1a2b3c4d";
const result: GetUserBadge200Response = await getUserBadge(tenantId, id);
[inline-code-end]
