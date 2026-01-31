## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |

## Response

Returns: [`GetVotes200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetVotes200Response.ts)

## Example

[inline-code-attrs-start title = 'getVotes Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9bd3a1-us-west";
const pagePathFragment: string | undefined = "/product/fastcomments-sdk-update";
const urlId: string = `https://www.examplenews.com${pagePathFragment ?? "/product/launch"}`;
const votes: GetVotes200Response = await getVotes(tenantId, urlId);
[inline-code-end]
