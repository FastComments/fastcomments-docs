## Parametreler

| İsim | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| page | number | Hayır |  |
| limit | number | Hayır |  |
| skip | number | Hayır |  |
| asTree | boolean | Hayır |  |
| skipChildren | number | Hayır |  |
| limitChildren | number | Hayır |  |
| maxTreeDepth | number | Hayır |  |
| urlId | string | Hayır |  |
| userId | string | Hayır |  |
| anonUserId | string | Hayır |  |
| contextUserId | string | Hayır |  |
| hashTag | string | Hayır |  |
| parentId | string | Hayır |  |
| direction | SortDirections | Hayır |  |
| fromDate | number | Hayır |  |
| toDate | number | Hayır |  |

## Yanıt

Returns: [`GetCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsResponse.ts)

## Örnek

[inline-code-attrs-start title = 'getComments Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const page: number = 2;
const limit: number = 50;
const asTree: boolean = true;
const urlId: string = "article_5678";
const direction: SortDirections = "desc";
const fromDate: number = Date.now() - 7 * 24 * 60 * 60 * 1000; // bir hafta önce
const toDate: number = Date.now();

const commentsResponse: GetCommentsResponse = await getComments({
  tenantId,
  page,
  limit,
  asTree,
  urlId,
  direction,
  fromDate,
  toDate,
});
[inline-code-end]