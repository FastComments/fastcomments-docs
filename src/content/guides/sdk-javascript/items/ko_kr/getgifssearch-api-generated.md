## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| search | string | 예 |  |
| locale | string | 아니요 |  |
| rating | string | 아니요 |  |
| page | number | 아니요 |  |

## 응답

반환: [`GetGifsSearchResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetGifsSearchResponse.ts)

## 예제

[inline-code-attrs-start title = 'getGifsSearch 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-42';
const search: string = 'golden hour sunset';
const locale: string = 'en-US';
const rating: string = 'pg';
const page: number = 1;
const result: GetGifsSearchResponse = await getGifsSearch(tenantId, search, locale, rating, page);
[inline-code-end]

---