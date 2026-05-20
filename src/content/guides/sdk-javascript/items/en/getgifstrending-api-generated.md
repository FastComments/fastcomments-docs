## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| locale | string | No |  |
| rating | string | No |  |
| page | number | No |  |

## Response

Returns: [`GifSearchResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GifSearchResponse.ts)

## Example

[inline-code-attrs-start title = 'getGifsTrending Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_8a3f47';
const locale: string = 'en-US';
const rating: string = 'pg';
const page: number = 2;
const trendingFull: GifSearchResponse = await getGifsTrending(tenantId, locale, rating, page);
const trendingMinimal: GifSearchResponse = await getGifsTrending(tenantId);
[inline-code-end]
