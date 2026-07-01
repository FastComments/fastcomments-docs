## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| updateUserBadgeParams | UpdateUserBadgeParams | Yes |  |

## Response

Returns: [`UpdateUserBadgeResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateUserBadgeResponse.ts)

## Example

[inline-code-attrs-start title = 'updateUserBadge Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function applyBadge() {
    const tenantId: string = "acme-corp-tenant";
    const userId: string = "user-98765";

    const params: UpdateUserBadgeParams = {
        badgeId: "gold-contributor",
        // optional field example
        expiresAt: new Date(Date.now() + 30 * 24 * 60 * 60 * 1000).toISOString(),
    };

    const result: UpdateUserBadgeResponse = await updateUserBadge(tenantId, userId, params);
    console.log(result);
}

applyBadge();
[inline-code-end]
