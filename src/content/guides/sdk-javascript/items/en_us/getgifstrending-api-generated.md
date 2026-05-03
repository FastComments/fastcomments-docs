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
const tenantId: string = "acme-tenant-01";
const trendingBasic: GifSearchResponse = await getGifsTrending(tenantId);

const locale: string = "en-GB";
const rating: string = "pg";
const page: number = 1;
const trendingWithOptions: GifSearchResponse = await getGifsTrending(tenantId, locale, rating, page);
[inline-code-end]
