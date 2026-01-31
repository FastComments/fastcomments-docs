## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Response

Returns: [`GetModerator200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetModerator200Response.ts)

## Example

[inline-code-attrs-start title = 'getModerator Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7f3b2a9c';
const moderatorId: string = 'mod_12fa9c4b';
const moderator: GetModerator200Response = await getModerator(tenantId, moderatorId);
[inline-code-end]
