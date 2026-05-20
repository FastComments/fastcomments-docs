## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| search | string | Yes |  |
| locale | string | No |  |
| rating | string | No |  |
| page | number | No |  |

## Response

Returns: [`GifSearchResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GifSearchResponse.ts)

## Example

[inline-code-attrs-start title = 'getGifsSearch Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const detailedResult: GifSearchResponse = await getGifsSearch("tenant_12345", "funny dachshund", "en-GB", "g", 2);
const minimalResult: GifSearchResponse = await getGifsSearch("tenant_12345", "dancing toddler");
[inline-code-end]
