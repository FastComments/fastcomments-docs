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
const tenantId: string = 'tenant-9b2f3c8a';
const urlId: string = '/news/2026/product-launch';
const userId: string | undefined = undefined;
const anonUserId: string | undefined = 'anon-4c7d21';
const votes: GetVotesForUser200Response = await getVotesForUser(tenantId, urlId, userId, anonUserId);
console.log(votes);
[inline-code-end]
