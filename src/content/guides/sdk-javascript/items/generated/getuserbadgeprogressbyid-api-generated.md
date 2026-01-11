## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Response

Returns: [`GetUserBadgeProgressById200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserBadgeProgressById200Response.ts)

## Example

[inline-code-attrs-start title = 'getUserBadgeProgressById Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-enterprise-84a1";
const id: string = "user-9f2c1a7b";
const fieldsToInclude: string[] | undefined = ["progress", "milestones"]; // optional parameters example
const badgeProgress: GetUserBadgeProgressById200Response = await getUserBadgeProgressById(tenantId, id);
[inline-code-end]
