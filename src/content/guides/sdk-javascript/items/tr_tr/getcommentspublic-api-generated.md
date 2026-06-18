req
tenantId
urlId

## Parametreler

| Name | Type | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| urlId | string | Evet |  |
| page | number | Hayır |  |
| direction | SortDirections | Hayır |  |
| sso | string | Hayır |  |
| skip | number | Hayır |  |
| skipChildren | number | Hayır |  |
| limit | number | Hayır |  |
| limitChildren | number | Hayır |  |
| countChildren | boolean | Hayır |  |
| fetchPageForCommentId | string | Hayır |  |
| includeConfig | boolean | Hayır |  |
| countAll | boolean | Hayır |  |
| includei10n | boolean | Hayır |  |
| locale | string | Hayır |  |
| modules | string | Hayır |  |
| isCrawler | boolean | Hayır |  |
| includeNotificationCount | boolean | Hayır |  |
| asTree | boolean | Hayır |  |
| maxTreeDepth | number | Hayır |  |
| useFullTranslationIds | boolean | Hayır |  |
| parentId | string | Hayır |  |
| searchText | string | Hayır |  |
| hashTags | Array<string> | Hayır |  |
| userId | string | Hayır |  |
| customConfigStr | string | Hayır |  |
| afterCommentId | string | Hayır |  |
| beforeCommentId | string | Hayır |  |

## Yanıt

Döndürür: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsPublic200Response.ts)

## Örnek

[inline-code-attrs-start title = 'getCommentsPublic Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-news';
const urlId: string = '/articles/2026/fastcomments-update';
const page: number = 1;
const skip: number = 0;
const limit: number = 25;
const countChildren: boolean = true;
const includeConfig: boolean = true;
const result: GetCommentsPublic200Response = await getCommentsPublic(
  tenantId,
  urlId,
  page,
  undefined,
  undefined,
  skip,
  undefined,
  limit,
  undefined,
  countChildren,
  undefined,
  includeConfig
);
[inline-code-end]