## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |

## Response

Returns: [`GetV2PageReacts`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetV2PageReacts.ts)

## Example

[inline-code-attrs-start title = 'getV2PageReacts Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'fastcomments-tenant-82';
const urlId: string = 'https://www.acmecorp.com/blog/product-launch-2026';
const reacts: GetV2PageReacts = await getV2PageReacts(tenantId, urlId);
console.log(reacts);
[inline-code-end]
