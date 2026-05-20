## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| largeInternalURLSanitized | string | Yes |  |

## Response

Returns: [`GifGetLargeResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GifGetLargeResponse.ts)

## Example

[inline-code-attrs-start title = 'getGifLarge Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_8a92f4';
const largeInternalURLSanitized: string = 'https://cdn.streamingco.com/gifs/product-demo-large.gif';
let maybeStatus: APIStatus | undefined = undefined; // optional metadata when available
const response: GifGetLargeResponse = await getGifLarge(tenantId, largeInternalURLSanitized);
[inline-code-end]
