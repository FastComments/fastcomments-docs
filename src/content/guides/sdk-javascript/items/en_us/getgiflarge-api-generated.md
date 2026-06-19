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
const tenantId: string = 'acme-corp-42';
const largeInternalURLSanitized: string = '/assets/internal/gifs/launch-party-9f8b7c.gif';
const correlationId?: string = 'req-20260619-01';
const result: GifGetLargeResponse = await getGifLarge(tenantId, largeInternalURLSanitized);
[inline-code-end]
