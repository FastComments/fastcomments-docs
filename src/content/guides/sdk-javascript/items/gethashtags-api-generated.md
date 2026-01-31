## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| page | number | No |  |

## Response

Returns: [`GetHashTags200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetHashTags200Response.ts)

## Example

[inline-code-attrs-start title = 'getHashTags Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_9876';
const firstPageResult: GetHashTags200Response = await getHashTags(tenantId);
const secondPageResult: GetHashTags200Response = await getHashTags(tenantId, 2);
[inline-code-end]
