## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| userId | string | Yes |  |

## Response

Returns: [`GetUserBadgeProgressById200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserBadgeProgressById200Response.ts)

## Example

[inline-code-attrs-start title = 'getUserBadgeProgressByUserId Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp-2025';
const userId: string = 'user_72a9e4b8';
const result: GetUserBadgeProgressById200Response = await getUserBadgeProgressByUserId(tenantId, userId);
const badgeProgress: UserBadgeProgress | undefined = (result as any)?.badgeProgress;
const apiStatus: APIStatus | undefined = (result as any)?.status;
const includeDetails?: boolean = true;
[inline-code-end]
