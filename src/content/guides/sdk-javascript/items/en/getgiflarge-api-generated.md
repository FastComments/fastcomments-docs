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
const tenantId: string = 'acme-tenant-42';
const largeInternalURLSanitized: string = 'https://internal.cdn.acme-corp.com/gifs/promo-2026_large.gif';
const result: GifGetLargeResponse = await getGifLarge(tenantId, largeInternalURLSanitized);
const status: APIStatus | undefined = result.status;
const statusCode: number | undefined = status?.code;
[inline-code-end]
