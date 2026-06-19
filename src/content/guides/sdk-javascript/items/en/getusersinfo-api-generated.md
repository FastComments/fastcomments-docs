Bulk user info for a tenant. Given userIds, return display info from User / SSOUser.
Used by the comment widget to enrich users that just appeared via a presence event.
No page context: privacy is enforced uniformly (private profiles are masked).

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| ids | string | Yes |  |

## Response

Returns: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersInfoResponse.ts)

## Example

[inline-code-attrs-start title = 'getUsersInfo Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_78f9';
const ids: string = 'user_10234,user_10235,user_10236';
const usersInfo: PageUsersInfoResponse = await getUsersInfo(tenantId, ids);
// getUsersInfo only requires tenantId and ids; optional parameters are not applicable here.
[inline-code-end]
