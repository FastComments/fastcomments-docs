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
const tenantId: string = 'acme_marketing_tenant_7';
const largeInternalURLSanitized: string = 'https://cdn.acmeinc.com/gifs/promo-spring-2026_large_sanitized.gif';
const includePreview: boolean | undefined = undefined; // optional flag a caller might use
const result: GifGetLargeResponse = await getGifLarge(tenantId, largeInternalURLSanitized);
console.log(result, includePreview);
[inline-code-end]
