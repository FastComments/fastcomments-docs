## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| userId | string | No |  |
| anonUserId | string | No |  |

## Response

Returns: [`GetVotesForUser200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetVotesForUser200Response.ts)

## Example

[inline-code-attrs-start title = 'getVotesForUser Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acmeCorp_01';
const urlId: string = 'post-2026-05-20-new-feature';
const userId: string | undefined = 'user_9742';
const votes: GetVotesForUser200Response = await getVotesForUser(tenantId, urlId, userId);
[inline-code-end]
