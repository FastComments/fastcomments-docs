req
tenantId
urlId

## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|------|-------------|
| tenantId | string | 是 |  |
| urlId | string | 是 |  |
| page | number | 否 |  |
| direction | SortDirections | 否 |  |
| sso | string | 否 |  |
| skip | number | 否 |  |
| skipChildren | number | 否 |  |
| limit | number | 否 |  |
| limitChildren | number | 否 |  |
| countChildren | boolean | 否 |  |
| fetchPageForCommentId | string | 否 |  |
| includeConfig | boolean | 否 |  |
| countAll | boolean | 否 |  |
| includei10n | boolean | 否 |  |
| locale | string | 否 |  |
| modules | string | 否 |  |
| isCrawler | boolean | 否 |  |
| includeNotificationCount | boolean | 否 |  |
| asTree | boolean | 否 |  |
| maxTreeDepth | number | 否 |  |
| useFullTranslationIds | boolean | 否 |  |
| parentId | string | 否 |  |
| searchText | string | 否 |  |
| hashTags | Array<string> | 否 |  |
| userId | string | 否 |  |
| customConfigStr | string | 否 |  |
| afterCommentId | string | 否 |  |
| beforeCommentId | string | 否 |  |

## 回應

回傳: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsPublic200Response.ts)

## 範例

[inline-code-attrs-start title = 'getCommentsPublic 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

---